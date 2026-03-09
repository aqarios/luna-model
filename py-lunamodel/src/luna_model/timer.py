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

from luna_model._lm import PyTimer

if TYPE_CHECKING:
    from datetime import datetime, timedelta


@runtime_checkable
class Timing(Protocol):
    """Timing information for an operation.

    Records the start time, end time, and optional QPU time for an operation.

    Attributes
    ----------
    start : datetime
        When the operation started.
    end : datetime
        When the operation ended.
    total : timedelta
        Total elapsed time.
    total_seconds : float
        Total elapsed time in seconds.
    qpu : float or None
        QPU time in seconds, if applicable.
    """

    @property
    def start(self) -> datetime:
        """Get the start time."""
        ...

    @property
    def end(self) -> datetime:
        """Get the end time."""
        ...

    @property
    def total(self) -> timedelta:
        """Get the total elapsed time."""
        ...

    @property
    def total_seconds(self) -> float:
        """Get the total elapsed time in seconds."""
        ...

    @property
    def qpu(self) -> float | None:
        """Get the QPU time in seconds."""
        ...

    @qpu.setter
    def qpu(self, value: float | None) -> None:
        """Set the QPU time in seconds."""
        ...

    def add_qpu(self, value: float) -> None:
        """Add time to the QPU counter."""
        ...

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the timing.
        """
        ...


class Timer:
    """Timer for measuring execution time.

    Provides start/stop functionality for timing operations.

    Examples
    --------
    >>> from luna_model.timer import Timer
    >>> timer = Timer.start()
    >>> # ... perform operations ...
    >>> timing = timer.stop()
    >>> print(f"Elapsed: {timing.total_seconds} seconds")
    Elapsed: ... seconds
    """

    _t: PyTimer

    def __init__(self) -> None:
        msg = "cannot create 'Timer' instances directly, use 'Timer.start()'"
        raise TypeError(msg)

    @classmethod
    def _from_pyt(cls, py_t: PyTimer) -> Timer:
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

    def stop(self) -> Timing:
        """Stop the timer and return timing information.

        Returns
        -------
        Timing
            The timing information for the measured interval.
        """
        return self._t.stop()
