from aqmodels._api_utils import export, dispatched


@export
class Timing:
    """
    The object that holds information about an algorithm's runtime.

    This class can only be constructed using a ``Timer``. This ensures that a
    ``Timing`` object always contains a start as well as an end time.

    The ``qpu`` field of this class can only be set after constructing it with a timer.

    Examples
    --------
    >>> from dwave.samplers.tree.solve import BinaryQuadraticModel
    >>> from luna_quantum import Model, Timer, Timing
    >>> model = ... # third-party model
    >>> algorithm = ... # third-party algorithm
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
    @dispatched
    def start(self):
        """The starting time of the algorithm."""
        return

    @property
    @dispatched
    def end(self):
        """The end, or finishing, time of the algorithm."""
        return

    @property
    @dispatched
    def total(self):
        """
        The difference of the end and start time.

        Raises
        ------
        RuntimeError
            If total cannot be computed due to an inconsistent start or end time.
        """
        return

    @property
    @dispatched
    def total_seconds(self):
        """
        The total time in seconds an algorithm needed to run. Computed as the
        difference of end and start time.

        Raises
        ------
        RuntimeError
            If total_seconds cannot be computed due to an inconsistent start or end time.
        """
        return

    @property
    @dispatched
    def qpu(self):
        """The qpu usage time of the algorithm this timing object was created for."""
        return

    @qpu.setter
    @dispatched
    def qpu(self, value):
        """
        Set the qpu usage time.

        Raises
        ------
        ValueError
            If ``value`` is negative."""
        return value

    @dispatched
    def add_qpu(self, value: float):
        """
        Add qpu usage time to the qpu usage time already present. If the current value
        is None, this method acts like a setter.

        Parameters
        ----------
        value : float
            The value to add to the already present qpu value.

        Raises
        ------
        ValueError
            If ``value`` is negative.
        """
        return


@export
class Timer:
    """
    Used to measure the computation time of an algorithm.

    The sole purpose of the ``Timer`` class is to create a ``Timing`` object in a safe
    way, i.e., to ensure that the ``Timing`` object always holds a starting and
    finishing time.

    Examples
    --------
    Basic usage:
    >>> from luna_quantum import  Timer
    >>> timer = Timer.start()
    >>> solution = ... # create a solution by running an algorithm.
    >>> timing = timer.stop()
    """

    @dispatched
    @staticmethod
    def start():
        """
        Create a timer that starts counting immediately.

        Returns
        -------
        Timer
            The timer.
        """
        return

    @dispatched
    def stop(self):
        """
        Stop the timer, and get the resulting ``Timing`` object.

        Returns
        -------
        Timing
            The timing object that holds the start and end time.
        """
        return
