# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from __future__ import annotations

from typing import TYPE_CHECKING, Protocol, runtime_checkable

from luna_model._deprecated import deprecated
from luna_model._lm import PyTimer, PyTiming

if TYPE_CHECKING:
    from datetime import datetime


class Timing:
    """Timing information recorded by a `Timer`.

    Holds the overall elapsed time plus any named sub-timings recorded
    via `Timer.record()`.
    """

    _t: PyTiming

    def __init__(self, total: float | None = None) -> None:
        """Create a `Timing`.

        Parameters
        ----------
        total : float, optional
            The overall elapsed time in seconds. Defaults to 0.0.
        """
        self._t = PyTiming(total)

    @classmethod
    def _from_pyt(cls, py_t: PyTiming) -> Timing:
        """Wrap an existing `PyTiming` instance without going through `__init__`."""
        t = cls.__new__(cls)
        t._t = py_t
        return t

    def __getattr__(self, name: str) -> float | None:
        """Get the total elapsed time for a named sub-timing, pandas-column-style.

        Falls back to `total_for(name)` for any attribute not otherwise
        defined on `Timing`, so e.g. `timing.qpu` is equivalent to
        `timing.total_for("qpu")`. Only invoked when normal attribute
        lookup fails, so it never shadows `total`, `get`, etc.
        """
        if name.startswith("_"):
            raise AttributeError(name)
        return self.total_for(name)

    def __setattr__(self, name: str, value: float) -> None:
        """Set a named sub-timing via attribute access, pandas-column-style.

        Falls back to `self[name] = value` for any attribute not already
        defined on `Timing` (e.g. `_t`, `total`, `get`), so e.g.
        `timing.qpu = 1.0` is equivalent to `timing["qpu"] = 1.0`.
        """
        if name == "_t" or hasattr(type(self), name):
            object.__setattr__(self, name, value)
        else:
            self[name] = value

    @property
    def start(self) -> datetime | None:
        """Get the start time."""
        return self._t.start

    @property
    def end(self) -> datetime | None:
        """Get the end time."""
        return self._t.end

    @property
    def total(self) -> float:
        """Get the overall elapsed time in seconds."""
        return self._t.total

    @property
    @deprecated("total_seconds is deprecated, use `total` instead.")
    def total_seconds(self) -> float:
        """Get the overall elapsed time in seconds."""
        return self.total

    def get(self, key: str) -> list[float]:
        """Get the recorded values for a named sub-timing.

        Parameters
        ----------
        key : str
            The sub-timing name.

        Returns
        -------
        list[float]
            The recorded elapsed times, in seconds. Empty if `key` was
            never recorded.
        """
        return self._t.get(key)

    def total_for(self, key: str) -> float | None:
        """Get the total elapsed time for a named sub-timing.

        Parameters
        ----------
        key : str
            The sub-timing name.

        Returns
        -------
        float or None
            The sum of the recorded values for `key`, in seconds, or
            `None` if `key` was never recorded.
        """
        return self._t.total_for(key)

    def __setitem__(self, key: str, value: float) -> None:
        """Overwrite the recorded values for a named sub-timing.

        Parameters
        ----------
        key : str
            The sub-timing name.
        value : float
            The single value to record for `key`, replacing any
            previously recorded values.
        """
        self._t[key] = value

    def __getitem__(self, key: str) -> float:
        """Get the total elapsed time for a named sub-timing.

        Equivalent to `total_for(key)`, but returns `0.0` instead of
        `None` when `key` was never recorded, so `timing[key] += value`
        works directly.

        Parameters
        ----------
        key : str
            The sub-timing name.

        Returns
        -------
        float
            The sum of the recorded values for `key`, in seconds, or
            `0.0` if `key` was never recorded.
        """
        return self._t[key]

    def __eq__(self, other: object) -> bool:
        """Check equality with another `Timing` by total and recorded values."""
        if not isinstance(other, Timing):
            return NotImplemented
        return self._t == other._t

    def __str__(self) -> str:
        """Return a human-readable string representation."""
        return str(self._t)

    def __repr__(self) -> str:
        """Return a string representation for debugging."""
        return repr(self._t)


@runtime_checkable
class SubTimer(Protocol):
    """A running sub-timer for one named interval.

    Returned by `Timer.record()`. Call `stop()` exactly once to record
    the elapsed time under its name on the parent `Timer`.
    """

    def stop(self) -> float:
        """Stop this sub-timer.

        Returns
        -------
        float
            Elapsed seconds for this interval.

        Raises
        ------
        RuntimeError
            If the sub-timer was already stopped.
        """
        ...


class Timer:
    """Timer for measuring execution time, with named sub-intervals.

    Examples
    --------
    >>> from luna_model.timer import Timer
    >>> t = Timer.start()
    >>> preprocessing = t.record("preprocessing")
    >>> # ... perform preprocessing ...
    >>> _ = preprocessing.stop()
    >>> qpu = t.record("qpu")
    >>> # ... call the QPU ...
    >>> _ = qpu.stop()
    >>> timing = t.stop()
    >>> print(f"Elapsed: {timing.total} seconds")  # doctest: +SKIP
    Elapsed: ... seconds
    """

    _t: PyTimer

    def __init__(self) -> None:
        self._t = PyTimer()

    @classmethod
    def _from_pyt(cls, py_t: PyTimer) -> Timer:
        """Wrap an existing `PyTimer` instance without going through `__init__`."""
        t = cls.__new__(cls)
        t._t = py_t
        return t

    @classmethod
    def start(cls) -> Timer:
        """Start a new timer.

        Returns
        -------
        Timer
            A running timer instance.
        """
        return cls._from_pyt(PyTimer.start())

    def record(self, timing: str) -> SubTimer:
        """Start a named sub-timer.

        Parameters
        ----------
        timing : str
            The name under which the elapsed time will be recorded
            once the returned sub-timer is stopped.

        Returns
        -------
        SubTimer
            A running sub-timer; call `.stop()` on it to record it.
        """
        return self._t.record(timing)

    def stop(self) -> Timing:
        """Stop the timer and return timing information.

        Returns
        -------
        Timing
            The timing information for the measured interval, including
            any named sub-timings recorded along the way.
        """
        return Timing._from_pyt(self._t.stop())
