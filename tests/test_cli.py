import unittest

from ime_control.cli import parse_window_kinds, render_state
from ime_control.komorebi import RgbColor
from ime_control.watcher import iter_state_changes
from ime_control.win32_ime import ImeState, is_english_mode


def make_state(*, available: bool, conversion_mode: int) -> ImeState:
    return ImeState(
        foreground_hwnd=0x100 if available else 0,
        target_hwnd=0x100 if available else 0,
        ime_hwnd=0x200 if available else 0,
        thread_id=1,
        process_id=2,
        keyboard_layout=0x0000000008040804,
        open_status=1,
        conversion_mode=conversion_mode,
        sentence_mode=0x8,
    )


class CliTests(unittest.TestCase):
    def test_native_bit_means_not_english(self) -> None:
        self.assertTrue(is_english_mode(0x0000))
        self.assertTrue(is_english_mode(0x0008))
        self.assertFalse(is_english_mode(0x0001))
        self.assertFalse(is_english_mode(0x0009))

    def test_render_state_default_output(self) -> None:
        self.assertEqual(render_state(make_state(available=True, conversion_mode=0x0000), False), "true")
        self.assertEqual(render_state(make_state(available=True, conversion_mode=0x0001), False), "false")
        self.assertEqual(render_state(make_state(available=False, conversion_mode=0x0000), False), "unknown")

    def test_parse_window_kinds(self) -> None:
        self.assertEqual(parse_window_kinds("single, stack,floating"), ("single", "stack", "floating"))
        with self.assertRaises(ValueError):
            parse_window_kinds(" , ")

    def test_rgb_parse(self) -> None:
        self.assertEqual(RgbColor.parse("255,200,0"), RgbColor(255, 200, 0))
        with self.assertRaises(ValueError):
            RgbColor.parse("255,0")

    def test_watcher_emits_only_on_change(self) -> None:
        states = iter(
            [
                make_state(available=True, conversion_mode=0x0001),
                make_state(available=True, conversion_mode=0x0001),
                make_state(available=True, conversion_mode=0x0000),
            ]
        )

        def next_state() -> ImeState:
            return next(states)

        events = iter_state_changes(interval=0.0, get_state=next_state)
        self.assertEqual(next(events).machine_state, "false")
        self.assertEqual(next(events).machine_state, "true")
