from __future__ import annotations

import ctypes
import ctypes.wintypes as wintypes
from dataclasses import dataclass


user32 = ctypes.WinDLL("user32", use_last_error=True)
imm32 = ctypes.WinDLL("imm32", use_last_error=True)

WM_IME_CONTROL = 0x0283
IMC_GETCONVERSIONMODE = 0x0001
IMC_GETSENTENCEMODE = 0x0003
IMC_GETOPENSTATUS = 0x0005
IME_CMODE_NATIVE = 0x0001


class GUITHREADINFO(ctypes.Structure):
    _fields_ = [
        ("cbSize", wintypes.DWORD),
        ("flags", wintypes.DWORD),
        ("hwndActive", wintypes.HWND),
        ("hwndFocus", wintypes.HWND),
        ("hwndCapture", wintypes.HWND),
        ("hwndMenuOwner", wintypes.HWND),
        ("hwndMoveSize", wintypes.HWND),
        ("hwndCaret", wintypes.HWND),
        ("rcCaret", wintypes.RECT),
    ]


@dataclass(frozen=True)
class ImeState:
    foreground_hwnd: int
    target_hwnd: int
    ime_hwnd: int
    thread_id: int
    process_id: int
    keyboard_layout: int
    open_status: int
    conversion_mode: int
    sentence_mode: int

    @property
    def available(self) -> bool:
        return self.foreground_hwnd != 0 and self.ime_hwnd != 0

    @property
    def is_english(self) -> bool:
        return is_english_mode(self.conversion_mode)

    @property
    def machine_state(self) -> str:
        if not self.available:
            return "unknown"
        return "true" if self.is_english else "false"


def _hwnd_value(hwnd: object) -> int:
    return int(hwnd or 0)


def is_english_mode(conversion_mode: int) -> bool:
    return (conversion_mode & IME_CMODE_NATIVE) == 0


def get_foreground_ime_state() -> ImeState:
    foreground_hwnd = _hwnd_value(user32.GetForegroundWindow())
    if foreground_hwnd == 0:
        return ImeState(0, 0, 0, 0, 0, 0, 0, 0, 0)

    process_id = wintypes.DWORD()
    thread_id = user32.GetWindowThreadProcessId(foreground_hwnd, ctypes.byref(process_id))

    gui = GUITHREADINFO(cbSize=ctypes.sizeof(GUITHREADINFO))
    user32.GetGUIThreadInfo(thread_id, ctypes.byref(gui))

    target_hwnd = _hwnd_value(gui.hwndFocus) or _hwnd_value(gui.hwndActive) or foreground_hwnd
    ime_hwnd = _hwnd_value(imm32.ImmGetDefaultIMEWnd(target_hwnd))
    keyboard_layout = int(user32.GetKeyboardLayout(thread_id) or 0)

    open_status = 0
    conversion_mode = 0
    sentence_mode = 0
    if ime_hwnd:
        open_status = user32.SendMessageW(ime_hwnd, WM_IME_CONTROL, IMC_GETOPENSTATUS, 0)
        conversion_mode = user32.SendMessageW(ime_hwnd, WM_IME_CONTROL, IMC_GETCONVERSIONMODE, 0)
        sentence_mode = user32.SendMessageW(ime_hwnd, WM_IME_CONTROL, IMC_GETSENTENCEMODE, 0)

    return ImeState(
        foreground_hwnd=foreground_hwnd,
        target_hwnd=target_hwnd,
        ime_hwnd=ime_hwnd,
        thread_id=thread_id,
        process_id=process_id.value,
        keyboard_layout=keyboard_layout,
        open_status=open_status,
        conversion_mode=conversion_mode,
        sentence_mode=sentence_mode,
    )
