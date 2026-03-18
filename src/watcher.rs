use crate::win32_ime::ImeState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatchEvent {
    pub state: ImeState,
    pub machine_state: crate::win32_ime::MachineState,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StateChangeDetector {
    previous_key: Option<StateKey>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StateKey {
    machine_state: crate::win32_ime::MachineState,
    conversion_mode: u32,
    keyboard_layout: u64,
}

impl StateChangeDetector {
    pub fn observe(&mut self, state: ImeState) -> Option<WatchEvent> {
        let key = StateKey {
            machine_state: state.machine_state(),
            conversion_mode: state.conversion_mode,
            keyboard_layout: state.keyboard_layout,
        };
        if self.previous_key == Some(key) {
            return None;
        }
        self.previous_key = Some(key);
        Some(WatchEvent {
            state,
            machine_state: key.machine_state,
        })
    }
}
