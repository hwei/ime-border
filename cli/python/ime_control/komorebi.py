from __future__ import annotations

import shutil
import subprocess
from dataclasses import dataclass


DEFAULT_WINDOW_KINDS = ("single", "stack", "monocle", "floating")


@dataclass(frozen=True)
class RgbColor:
    red: int
    green: int
    blue: int

    @classmethod
    def parse(cls, text: str) -> "RgbColor":
        parts = [part.strip() for part in text.split(",")]
        if len(parts) != 3:
            raise ValueError(f"RGB color must have 3 comma-separated parts, got '{text}'")
        values = [int(part) for part in parts]
        if any(value < 0 or value > 255 for value in values):
            raise ValueError(f"RGB color values must be in 0..255, got '{text}'")
        return cls(*values)

    def as_args(self) -> list[str]:
        return [str(self.red), str(self.green), str(self.blue)]


def resolve_komorebic(explicit_path: str | None = None) -> str:
    if explicit_path:
        return explicit_path
    path = shutil.which("komorebic.exe") or shutil.which("komorebic")
    if path is None:
        raise FileNotFoundError("komorebic executable was not found on PATH")
    return path


def apply_border_colours(
    *,
    komorebic_path: str,
    color: RgbColor,
    window_kinds: tuple[str, ...] = DEFAULT_WINDOW_KINDS,
) -> None:
    for window_kind in window_kinds:
        subprocess.run(
            [komorebic_path, "border-colour", "--window-kind", window_kind, *color.as_args()],
            check=True,
            capture_output=True,
            text=True,
        )
