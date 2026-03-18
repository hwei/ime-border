use std::path::Path;

use ime_border::cli::{parse_window_kinds, render_state};
use ime_border::komorebi::{RgbColor, apply_border_colours_with};
use ime_border::watcher::StateChangeDetector;
use ime_border::win32_ime::{ImeState, MachineState, is_english_mode};

fn make_state(available: bool, conversion_mode: u32) -> ImeState {
    ImeState {
        foreground_hwnd: if available { 0x100 } else { 0 },
        target_hwnd: if available { 0x100 } else { 0 },
        ime_hwnd: if available { 0x200 } else { 0 },
        thread_id: 1,
        process_id: 2,
        keyboard_layout: 0x0000_0000_0804_0804,
        open_status: 1,
        conversion_mode,
        sentence_mode: 0x8,
    }
}

#[test]
fn native_bit_means_not_english() {
    assert!(is_english_mode(0x0000));
    assert!(is_english_mode(0x0008));
    assert!(!is_english_mode(0x0001));
    assert!(!is_english_mode(0x0009));
}

#[test]
fn render_state_default_output() {
    assert_eq!(render_state(&make_state(true, 0x0000), false), "true");
    assert_eq!(render_state(&make_state(true, 0x0001), false), "false");
    assert_eq!(render_state(&make_state(false, 0x0000), false), "unknown");
}

#[test]
fn parse_window_kinds_rejects_empty() {
    assert_eq!(
        parse_window_kinds("single, stack,floating").unwrap(),
        vec![
            "single".to_string(),
            "stack".to_string(),
            "floating".to_string()
        ]
    );
    assert!(parse_window_kinds(" , ").is_err());
}

#[test]
fn rgb_parse_validates_shape() {
    assert_eq!(
        RgbColor::parse("255,200,0").unwrap(),
        RgbColor {
            red: 255,
            green: 200,
            blue: 0
        }
    );
    assert!(RgbColor::parse("255,0").is_err());
}

#[test]
fn watcher_emits_only_on_change() {
    let mut detector = StateChangeDetector::default();
    assert_eq!(
        detector
            .observe(make_state(true, 0x0001))
            .unwrap()
            .machine_state,
        MachineState::False
    );
    assert_eq!(detector.observe(make_state(true, 0x0001)), None);
    assert_eq!(
        detector
            .observe(make_state(true, 0x0000))
            .unwrap()
            .machine_state,
        MachineState::True
    );
}

#[test]
fn apply_border_colours_calls_each_kind() {
    let mut calls = Vec::new();
    apply_border_colours_with(
        Path::new("komorebic.exe"),
        RgbColor {
            red: 1,
            green: 2,
            blue: 3,
        },
        &["single".to_string(), "floating".to_string()],
        |program, args| {
            calls.push((
                program.display().to_string(),
                args.iter()
                    .map(|value| value.to_string_lossy().to_string())
                    .collect::<Vec<_>>(),
            ));
            Ok(())
        },
    )
    .unwrap();

    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].0, "komorebic.exe");
    assert_eq!(
        calls[0].1,
        vec!["border-colour", "--window-kind", "single", "1", "2", "3"]
    );
    assert_eq!(
        calls[1].1,
        vec!["border-colour", "--window-kind", "floating", "1", "2", "3"]
    );
}
