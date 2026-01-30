from __future__ import annotations

from typing import TYPE_CHECKING, Protocol

from luna_model._lm import PyTimer

if TYPE_CHECKING:
    from datetime import datetime, timedelta


class Timing(Protocol):
    """Timing."""

    @property
    def start(self) -> datetime:
        """Get start."""
        ...

    @property
    def end(self) -> datetime:
        """Get end."""
        ...

    @property
    def total(self) -> timedelta:
        """Get total."""
        ...

    @property
    def total_seconds(self) -> float:
        """Get total as seconds."""
        ...

    @property
    def qpu(self) -> float | None:
        """Get total qpu time as seconds."""
        ...

    @qpu.setter
    def qpu(self, value: float | None) -> None:
        """Set total qpu time as seconds."""
        ...

    def add_qpu(self, value: float) -> None:
        """Add to qpu time."""
        ...


class Timer:
    """Timer."""

    _t: PyTimer

    @classmethod
    def _from_pyt(cls, py_t: PyTimer) -> Timer:
        t = cls.__new__(cls)
        t._t = py_t
        return t

    @classmethod
    def start(cls) -> Timer:
        """Start the timer."""
        return cls._from_pyt(PyTimer.start())

    def stop(self) -> Timing:
        """Stop the timer."""
        return self._t.stop()
