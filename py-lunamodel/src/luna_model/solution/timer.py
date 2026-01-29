# TODO: move to it's own module.
from __future__ import annotations

from datetime import datetime, timedelta
from typing import Protocol

from luna_model._lm import PyTimer


class Timing(Protocol):
    @property
    def start(self) -> datetime: ...

    @property
    def end(self) -> datetime: ...

    @property
    def total(self) -> timedelta: ...

    @property
    def total_seconds(self) -> float: ...

    @property
    def qpu(self) -> float | None: ...

    @qpu.setter
    def qpu(self, value: float | None) -> None: ...

    def add_qpu(self, value: float) -> None: ...


class Timer:
    _t: PyTimer

    @classmethod
    def _from_pyt(cls, py_t: PyTimer) -> Timer:
        t = cls.__new__(cls)
        t._t = py_t
        return t

    @classmethod
    def start(cls) -> Timer:
        return cls._from_pyt(PyTimer.start())

    def stop(self) -> Timing:
        return self._t.stop()
