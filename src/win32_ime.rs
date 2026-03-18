#[cfg(target_os = "windows")]
mod imp {
    use windows_sys::Win32::Foundation::{HWND, LPARAM, WPARAM};
    use windows_sys::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GUITHREADINFO, GetForegroundWindow, GetGUIThreadInfo, GetWindowThreadProcessId,
        SendMessageW,
    };

    pub const WM_IME_CONTROL: u32 = 0x0283;
    pub const IMC_GETCONVERSIONMODE: usize = 0x0001;
    pub const IMC_GETSENTENCEMODE: usize = 0x0003;
    pub const IMC_GETOPENSTATUS: usize = 0x0005;
    pub const IME_CMODE_NATIVE: u32 = 0x0001;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum MachineState {
        True,
        False,
        Unknown,
    }

    impl MachineState {
        pub fn as_str(self) -> &'static str {
            match self {
                Self::True => "true",
                Self::False => "false",
                Self::Unknown => "unknown",
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ImeState {
        pub foreground_hwnd: u64,
        pub target_hwnd: u64,
        pub ime_hwnd: u64,
        pub thread_id: u32,
        pub process_id: u32,
        pub keyboard_layout: u64,
        pub open_status: u32,
        pub conversion_mode: u32,
        pub sentence_mode: u32,
    }

    impl ImeState {
        pub const fn unavailable() -> Self {
            Self {
                foreground_hwnd: 0,
                target_hwnd: 0,
                ime_hwnd: 0,
                thread_id: 0,
                process_id: 0,
                keyboard_layout: 0,
                open_status: 0,
                conversion_mode: 0,
                sentence_mode: 0,
            }
        }

        pub const fn available(&self) -> bool {
            self.foreground_hwnd != 0 && self.ime_hwnd != 0
        }

        pub const fn is_english(&self) -> bool {
            is_english_mode(self.conversion_mode)
        }

        pub const fn machine_state(&self) -> MachineState {
            if !self.available() {
                MachineState::Unknown
            } else if self.is_english() {
                MachineState::True
            } else {
                MachineState::False
            }
        }
    }

    fn hwnd_value(hwnd: HWND) -> u64 {
        hwnd as usize as u64
    }

    pub const fn is_english_mode(conversion_mode: u32) -> bool {
        (conversion_mode & IME_CMODE_NATIVE) == 0
    }

    fn send_ime_control(ime_hwnd: HWND, command: usize) -> u32 {
        unsafe { SendMessageW(ime_hwnd, WM_IME_CONTROL, command as WPARAM, 0 as LPARAM) as u32 }
    }

    pub fn get_foreground_ime_state() -> ImeState {
        let foreground_hwnd = unsafe { GetForegroundWindow() };
        if foreground_hwnd.is_null() {
            return ImeState::unavailable();
        }

        let mut process_id = 0u32;
        let thread_id = unsafe { GetWindowThreadProcessId(foreground_hwnd, &mut process_id) };

        let mut gui = GUITHREADINFO {
            cbSize: core::mem::size_of::<GUITHREADINFO>() as u32,
            flags: 0,
            hwndActive: core::ptr::null_mut(),
            hwndFocus: core::ptr::null_mut(),
            hwndCapture: core::ptr::null_mut(),
            hwndMenuOwner: core::ptr::null_mut(),
            hwndMoveSize: core::ptr::null_mut(),
            hwndCaret: core::ptr::null_mut(),
            rcCaret: Default::default(),
        };
        let has_gui = unsafe { GetGUIThreadInfo(thread_id, &mut gui) } != 0;
        let target_hwnd = if has_gui && !gui.hwndFocus.is_null() {
            gui.hwndFocus
        } else if has_gui && !gui.hwndActive.is_null() {
            gui.hwndActive
        } else {
            foreground_hwnd
        };

        let ime_hwnd = unsafe { ImmGetDefaultIMEWnd(target_hwnd) };
        let keyboard_layout = unsafe { GetKeyboardLayout(thread_id) as usize as u64 };

        let (open_status, conversion_mode, sentence_mode) = if ime_hwnd.is_null() {
            (0, 0, 0)
        } else {
            (
                send_ime_control(ime_hwnd, IMC_GETOPENSTATUS),
                send_ime_control(ime_hwnd, IMC_GETCONVERSIONMODE),
                send_ime_control(ime_hwnd, IMC_GETSENTENCEMODE),
            )
        };

        ImeState {
            foreground_hwnd: hwnd_value(foreground_hwnd),
            target_hwnd: hwnd_value(target_hwnd),
            ime_hwnd: hwnd_value(ime_hwnd),
            thread_id,
            process_id,
            keyboard_layout,
            open_status,
            conversion_mode,
            sentence_mode,
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod imp {
    pub const IME_CMODE_NATIVE: u32 = 0x0001;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum MachineState {
        True,
        False,
        Unknown,
    }

    impl MachineState {
        pub fn as_str(self) -> &'static str {
            match self {
                Self::True => "true",
                Self::False => "false",
                Self::Unknown => "unknown",
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ImeState {
        pub foreground_hwnd: u64,
        pub target_hwnd: u64,
        pub ime_hwnd: u64,
        pub thread_id: u32,
        pub process_id: u32,
        pub keyboard_layout: u64,
        pub open_status: u32,
        pub conversion_mode: u32,
        pub sentence_mode: u32,
    }

    impl ImeState {
        pub const fn unavailable() -> Self {
            Self {
                foreground_hwnd: 0,
                target_hwnd: 0,
                ime_hwnd: 0,
                thread_id: 0,
                process_id: 0,
                keyboard_layout: 0,
                open_status: 0,
                conversion_mode: 0,
                sentence_mode: 0,
            }
        }

        pub const fn available(&self) -> bool {
            self.foreground_hwnd != 0 && self.ime_hwnd != 0
        }

        pub const fn is_english(&self) -> bool {
            is_english_mode(self.conversion_mode)
        }

        pub const fn machine_state(&self) -> MachineState {
            if !self.available() {
                MachineState::Unknown
            } else if self.is_english() {
                MachineState::True
            } else {
                MachineState::False
            }
        }
    }

    pub const fn is_english_mode(conversion_mode: u32) -> bool {
        (conversion_mode & IME_CMODE_NATIVE) == 0
    }

    pub fn get_foreground_ime_state() -> ImeState {
        ImeState::unavailable()
    }
}

pub use imp::*;
