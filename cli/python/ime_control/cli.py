from __future__ import annotations

import argparse
import sys
from datetime import datetime

from .komorebi import DEFAULT_WINDOW_KINDS, RgbColor, apply_border_colours, resolve_komorebic
from .watcher import iter_state_changes
from .win32_ime import ImeState, get_foreground_ime_state


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Query Microsoft Pinyin English mode and optionally drive komorebi borders.",
    )
    subparsers = parser.add_subparsers(dest="command")

    once = subparsers.add_parser("once", help="Print the current state once.")
    once.add_argument("--verbose", action="store_true", help="Print diagnostic fields.")

    watch = subparsers.add_parser("watch", help="Print only when the state changes.")
    watch.add_argument("--interval", type=float, default=0.2, help="Polling interval in seconds.")
    watch.add_argument("--verbose", action="store_true", help="Print diagnostic fields.")

    border = subparsers.add_parser("border-watch", help="Project state changes onto komorebi borders.")
    border.add_argument("--interval", type=float, default=0.2, help="Polling interval in seconds.")
    border.add_argument("--verbose", action="store_true", help="Print diagnostic fields.")
    border.add_argument("--english-colour", default="255,200,0", help="RGB colour for English mode.")
    border.add_argument("--non-english-colour", default="66,165,245", help="RGB colour for non-English mode.")
    border.add_argument("--unknown-colour", default="128,128,128", help="RGB colour for unknown mode.")
    border.add_argument("--window-kinds", default=",".join(DEFAULT_WINDOW_KINDS), help="Comma-separated window kinds.")
    border.add_argument("--komorebic", help="Explicit path to komorebic.exe.")
    return parser


def render_state(state: ImeState, verbose: bool) -> str:
    if not verbose:
        return state.machine_state
    timestamp = datetime.now().isoformat(timespec="seconds")
    english = "true" if state.is_english else "false"
    open_status = "true" if state.open_status else "false"
    available = "true" if state.available else "false"
    return (
        f"{timestamp} state={state.machine_state} available={available} english={english} "
        f"open={open_status} conv=0x{state.conversion_mode:08X} sent=0x{state.sentence_mode:08X} "
        f"hkl=0x{state.keyboard_layout:016X} hwnd=0x{state.foreground_hwnd:08X} "
        f"ime_hwnd=0x{state.ime_hwnd:08X}"
    )


def parse_window_kinds(text: str) -> tuple[str, ...]:
    values = tuple(part.strip() for part in text.split(",") if part.strip())
    if not values:
        raise ValueError("At least one komorebi window kind is required")
    return values


def run_once(verbose: bool) -> int:
    state = get_foreground_ime_state()
    print(render_state(state, verbose))
    return 0 if state.available else 2


def run_watch(interval: float, verbose: bool) -> int:
    for event in iter_state_changes(interval=interval):
        print(render_state(event.state, verbose), flush=True)
    return 0


def run_border_watch(args: argparse.Namespace) -> int:
    komorebic_path = resolve_komorebic(args.komorebic)
    colour_map = {
        "true": RgbColor.parse(args.english_colour),
        "false": RgbColor.parse(args.non_english_colour),
        "unknown": RgbColor.parse(args.unknown_colour),
    }
    window_kinds = parse_window_kinds(args.window_kinds)
    for event in iter_state_changes(interval=args.interval):
        apply_border_colours(
            komorebic_path=komorebic_path,
            color=colour_map[event.machine_state],
            window_kinds=window_kinds,
        )
        if args.verbose:
            print(render_state(event.state, True), flush=True)
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    command = args.command or "once"
    if command == "once":
        return run_once(getattr(args, "verbose", False))
    if command == "watch":
        return run_watch(args.interval, args.verbose)
    if command == "border-watch":
        return run_border_watch(args)
    parser.error(f"unsupported command: {command}")
    return 2


if __name__ == "__main__":
    sys.exit(main())
