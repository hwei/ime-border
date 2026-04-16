use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use chrono::Local;
use clap::{Parser, Subcommand};

use crate::komorebi::{DEFAULT_WINDOW_KINDS, RgbColor, apply_border_colours, resolve_komorebic};
use crate::watcher::StateChangeDetector;
use crate::win32_ime::{ImeState, get_foreground_ime_state};

#[derive(Debug, Parser)]
#[command(
    name = "ime-border",
    about = "Query Microsoft Pinyin English mode and optionally drive komorebi borders."
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print the current state once.
    Once {
        #[arg(long)]
        verbose: bool,
    },
    /// Print only when the state changes.
    Watch {
        #[arg(long, default_value_t = 0.2)]
        interval: f64,
        #[arg(long)]
        verbose: bool,
    },
    /// Project state changes onto komorebi borders.
    BorderWatch {
        #[arg(long, default_value_t = 0.2)]
        interval: f64,
        #[arg(long)]
        verbose: bool,
        #[arg(long, default_value = "255,200,0")]
        english_colour: String,
        #[arg(long, default_value = "66,165,245")]
        non_english_colour: String,
        #[arg(long, default_value = "128,128,128")]
        unknown_colour: String,
        #[arg(long, default_value = "single,stack,monocle,floating")]
        window_kinds: String,
        #[arg(long)]
        komorebic: Option<PathBuf>,
    },
}

pub fn run(argv: impl IntoIterator<Item = String>) -> Result<ExitCode> {
    let cli = Cli::parse_from(argv);
    match cli.command.unwrap_or(Command::Once { verbose: false }) {
        Command::Once { verbose } => run_once(verbose),
        Command::Watch { interval, verbose } => run_watch(interval, verbose),
        Command::BorderWatch {
            interval,
            verbose,
            english_colour,
            non_english_colour,
            unknown_colour,
            window_kinds,
            komorebic,
        } => run_border_watch(
            interval,
            verbose,
            &english_colour,
            &non_english_colour,
            &unknown_colour,
            &window_kinds,
            komorebic.as_deref(),
        ),
    }
}

pub fn parse_window_kinds(text: &str) -> Result<Vec<String>> {
    let values: Vec<String> = text
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    if values.is_empty() {
        anyhow::bail!("At least one komorebi window kind is required");
    }
    Ok(values)
}

pub fn render_state(state: &ImeState, verbose: bool) -> String {
    if !verbose {
        return state.machine_state().as_str().to_owned();
    }
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S");
    let english = if state.is_english() { "true" } else { "false" };
    let open_status = if state.open_status != 0 {
        "true"
    } else {
        "false"
    };
    let available = if state.available() { "true" } else { "false" };
    format!(
        "{timestamp} state={} available={available} english={english} open={open_status} conv=0x{:08X} sent=0x{:08X} hkl=0x{:016X} hwnd=0x{:08X} ime_hwnd=0x{:08X}",
        state.machine_state().as_str(),
        state.conversion_mode,
        state.sentence_mode,
        state.keyboard_layout,
        state.foreground_hwnd,
        state.ime_hwnd
    )
}

fn sleep_interval(interval: f64) {
    let duration = if interval <= 0.0 {
        Duration::ZERO
    } else {
        Duration::from_secs_f64(interval)
    };
    thread::sleep(duration);
}

pub fn run_once(verbose: bool) -> Result<ExitCode> {
    let state = get_foreground_ime_state();
    println!("{}", render_state(&state, verbose));
    Ok(if state.available() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    })
}

pub fn run_watch(interval: f64, verbose: bool) -> Result<ExitCode> {
    let mut detector = StateChangeDetector::default();
    loop {
        let state = get_foreground_ime_state();
        if let Some(event) = detector.observe(state) {
            println!("{}", render_state(&event.state, verbose));
        }
        sleep_interval(interval);
    }
}

pub fn run_border_watch(
    interval: f64,
    verbose: bool,
    english_colour: &str,
    non_english_colour: &str,
    unknown_colour: &str,
    window_kinds: &str,
    komorebic: Option<&Path>,
) -> Result<ExitCode> {
    let komorebic_path = resolve_komorebic(komorebic).context("failed to resolve komorebic")?;
    let english = RgbColor::parse(english_colour)?;
    let non_english = RgbColor::parse(non_english_colour)?;
    let unknown = RgbColor::parse(unknown_colour)?;
    let window_kinds = parse_window_kinds(window_kinds)?;
    let mut detector = StateChangeDetector::default();

    // 优化：状态缓存 - 记录上次设置的颜色
    let mut last_colour: Option<RgbColor> = None;
    // 优化：调用节流 - 记录上次调用时间
    let mut last_apply_time = Instant::now();
    // 优化：防抖 - 等待状态稳定
    let debounce_duration = Duration::from_millis(150);
    let throttle_duration = Duration::from_millis(300);
    let mut pending_colour: Option<RgbColor> = None;
    let mut pending_since: Option<Instant> = None;

    loop {
        let state = get_foreground_ime_state();
        if let Some(event) = detector.observe(state) {
            let colour = match event.machine_state.as_str() {
                "true" => english,
                "false" => non_english,
                _ => unknown,
            };

            // 优化：防抖 - 记录待处理的颜色变化
            pending_colour = Some(colour);
            pending_since = Some(Instant::now());

            if verbose {
                println!("{}", render_state(&event.state, true));
            }
        }

        // 优化：防抖 - 检查是否有待处理的颜色变化且已稳定
        if let (Some(colour), Some(since)) = (pending_colour, pending_since) {
            if since.elapsed() >= debounce_duration {
                // 优化：状态缓存 - 只有颜色不同时才更新
                let colour_changed = last_colour != Some(colour);
                // 优化：调用节流 - 确保两次调用之间有足够间隔
                let throttle_passed = last_apply_time.elapsed() >= throttle_duration;

                if colour_changed && throttle_passed {
                    apply_border_colours(&komorebic_path, colour, &window_kinds)?;
                    last_colour = Some(colour);
                    last_apply_time = Instant::now();
                }

                // 清除待处理状态
                pending_colour = None;
                pending_since = None;
            }
        }

        sleep_interval(interval);
    }
}

pub fn default_window_kinds_csv() -> String {
    DEFAULT_WINDOW_KINDS.join(",")
}
