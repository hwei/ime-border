use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

pub const DEFAULT_WINDOW_KINDS: &[&str] = &["single", "stack", "monocle", "floating"];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColor {
    pub fn parse(text: &str) -> Result<Self> {
        let parts: Vec<&str> = text.split(',').map(str::trim).collect();
        if parts.len() != 3 {
            anyhow::bail!("RGB color must have 3 comma-separated parts, got '{text}'");
        }
        let red = parts[0]
            .parse::<u8>()
            .with_context(|| format!("invalid red component in '{text}'"))?;
        let green = parts[1]
            .parse::<u8>()
            .with_context(|| format!("invalid green component in '{text}'"))?;
        let blue = parts[2]
            .parse::<u8>()
            .with_context(|| format!("invalid blue component in '{text}'"))?;
        Ok(Self { red, green, blue })
    }

    pub fn as_args(&self) -> [String; 3] {
        [
            self.red.to_string(),
            self.green.to_string(),
            self.blue.to_string(),
        ]
    }
}

pub fn resolve_komorebic(explicit_path: Option<&Path>) -> Result<PathBuf> {
    if let Some(path) = explicit_path {
        return Ok(path.to_path_buf());
    }
    find_on_path("komorebic.exe")
        .or_else(|| find_on_path("komorebic"))
        .ok_or_else(|| anyhow::anyhow!("komorebic executable was not found on PATH"))
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    env::split_paths(&path_var)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

pub fn border_colour_command_args(window_kind: &str, color: RgbColor) -> Vec<OsString> {
    let rgb = color.as_args();
    vec![
        OsString::from("border-colour"),
        OsString::from("--window-kind"),
        OsString::from(window_kind),
        OsString::from(&rgb[0]),
        OsString::from(&rgb[1]),
        OsString::from(&rgb[2]),
    ]
}

pub fn apply_border_colours(
    komorebic_path: &Path,
    color: RgbColor,
    window_kinds: &[String],
) -> Result<()> {
    apply_border_colours_with(komorebic_path, color, window_kinds, |program, args| {
        let status = Command::new(program)
            .args(args)
            .status()
            .with_context(|| format!("failed to execute {}", program.display()))?;
        if !status.success() {
            anyhow::bail!("komorebic exited with status {status}");
        }
        Ok(())
    })
}

pub fn apply_border_colours_with<F>(
    komorebic_path: &Path,
    color: RgbColor,
    window_kinds: &[String],
    mut runner: F,
) -> Result<()>
where
    F: FnMut(&Path, &[OsString]) -> Result<()>,
{
    for window_kind in window_kinds {
        let args = border_colour_command_args(window_kind, color);
        runner(komorebic_path, &args)?;
    }
    Ok(())
}
