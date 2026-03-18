from __future__ import annotations

import time
from collections.abc import Callable, Iterator
from dataclasses import dataclass

from .win32_ime import ImeState, get_foreground_ime_state


@dataclass(frozen=True)
class WatchEvent:
    state: ImeState
    machine_state: str


def state_key(state: ImeState) -> tuple[str, int, int]:
    return (state.machine_state, state.conversion_mode, state.keyboard_layout)


def iter_state_changes(
    *,
    interval: float,
    get_state: Callable[[], ImeState] = get_foreground_ime_state,
) -> Iterator[WatchEvent]:
    previous_key: tuple[str, int, int] | None = None
    while True:
        state = get_state()
        current_key = state_key(state)
        if current_key != previous_key:
            yield WatchEvent(state=state, machine_state=state.machine_state)
            previous_key = current_key
        time.sleep(interval)
