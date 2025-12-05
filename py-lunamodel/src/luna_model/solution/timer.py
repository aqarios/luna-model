from __future__ import annotations
from datetime import datetime, timedelta
from typing import Protocol

from luna_model._lm import PyTimer


class Timing(Protocol):
    """
    The object that holds information about an algorithm's runtime.

    This class can only be constructed using a `Timer`. This ensures that a
    `Timing` object always contains a start as well as an end time.

    The `qpu` field of this class can only be set after constructing it with a timer.

    Examples
    --------
    >>> from dwave.samplers.tree.solve import BinaryQuadraticModel
    >>> from luna_model import Model, Timer, Timing
    >>> model = ...  # third-party model
    >>> algorithm = ...  # third-party algorithm
    >>> timer = Timer.start()
    >>> sol = algorithm.run(model)
    >>> timing: Timing = timer.stop()
    >>> timing.qpu = sol.qpu_time
    >>> timing.total_seconds
    1.2999193
    >>> timing.qpu
    0.02491934
    """

    @property
    def start(self) -> datetime:
        """The starting time of the algorithm."""
        ...

    @property
    def end(self) -> datetime:
        """The end, or finishing, time of the algorithm."""
        ...

    @property
    def total(self) -> timedelta:
        """
        The difference of the end and start time.

        Raises
        ------
        RuntimeError
            If total cannot be computed due to an inconsistent start or end time.
        """
        ...

    @property
    def total_seconds(self) -> float:
        """
        The total time in seconds an algorithm needed to run.

        Computed as the difference of end and start time.

        Raises
        ------
        RuntimeError
            If `total_seconds` cannot be computed due to an inconsistent start or
            end time.
        """
        ...

    @property
    def qpu(self) -> float | None:
        """The qpu usage time of the algorithm this timing object was created for."""
        ...

    @qpu.setter
    def qpu(self, value: float | None) -> None:
        """
        Set the qpu usage time.

        Raises
        ------
        ValueError
            If `value` is negative.
        """
        ...

    def add_qpu(self, value: float) -> None:
        """
        Add qpu usage time to the qpu usage time already present.

        If the current value is None, this method acts like a setter.

        Parameters
        ----------
        value : float
            The value to add to the already present qpu value.

        Raises
        ------
        ValueError
            If `value` is negative.
        """
        ...


class Timer:
    """
    Used to measure the computation time of an algorithm.

    The sole purpose of the `Timer` class is to create a `Timing` object in a safe
    way, i.e., to ensure that the `Timing` object always holds a starting and
    finishing time.

    Examples
    --------
    Basic usage:
    >>> from luna_model import Timer
    >>> timer = Timer.start()
    >>> solution = ...  # create a solution by running an algorithm.
    >>> timing = timer.stop()
    """

    _t: PyTimer

    @classmethod
    def _from_pyt(cls, py_t: PyTimer) -> Timer:
        t = cls.__new__(cls)
        t._t = py_t
        return t

    @classmethod
    def start(cls) -> Timer:
        """
        Create a timer that starts counting immediately.

        Returns
        -------
        Timer
            The timer.
        """
        return cls._from_pyt(PyTimer())

    def stop(self) -> Timing:
        """
        Stop the timer, and get the resulting `Timing` object.

        Returns
        -------
        Timing
            The timing object that holds the start and end time.
        """
        return self._t.stop()
