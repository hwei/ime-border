import unittest
from unittest import mock

from ime_control.komorebi import RgbColor, apply_border_colours


class KomorebiTests(unittest.TestCase):
    @mock.patch("ime_control.komorebi.subprocess.run")
    def test_apply_border_colours_calls_each_kind(self, run_mock: mock.Mock) -> None:
        apply_border_colours(
            komorebic_path="komorebic.exe",
            color=RgbColor(1, 2, 3),
            window_kinds=("single", "floating"),
        )
        self.assertEqual(run_mock.call_count, 2)
