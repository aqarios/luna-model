from aqmodels._api_utils import export, dispatched


@export
class ResultIterator:
    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __next__(self):
        return


@export
class SampleIterator:
    """
    An iterator over the variable assignments of a solution's sample.

    Examples
    --------
    >>> from aqmodels import SampleIterator, Solution
    >>> solution: Solution = <...>
    >>> sample: SampleIterator = list(solution.samples)[0]
    >>> for var in sample:
    ...     str(var)
    '1'
    '-2.4'
    """

    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __next__(self):
        return


@export
class Samples:
    @dispatched
    def __str__(self):
        return

    @dispatched
    def __getitem__(self, item):
        return item


@export
class Sample:
    @dispatched
    def __str__(self):
        return

    @dispatched
    def __getitem__(self, item):
        return item


@export
class Result:
    @dispatched
    def __str__(self):
        return

    @dispatched
    def __repr__(self):
        return

    @property
    @dispatched
    def sample(self):
        return

    @property
    @dispatched
    def obj_value(self):
        return

    @property
    @dispatched
    def constraints(self):
        return

    @property
    @dispatched
    def feasible(self):
        return


@export
class ResultView:
    """
    A view into a single result of a solution.

    Examples
    --------
    >>> from aqmodels import ResultView, Solution
    >>> sol: Solution = <...>
    >>> res: ResultView = sol[0]
    >>> str(res.obj_value)
    '-109.42'
    >>> str(res.sample)
    '[1, 0, 0, 1, 10, -3, -5.1]'
    >>> str(res.feasible)
    'True'
    """

    @dispatched
    def __str__(self):
        return

    @dispatched
    def __repr__(self):
        return

    @dispatched
    def __getitem__(self, item):
        return item

    @property
    @dispatched
    def sample(self):
        """Get an iterator over the variable assignments of the result's samples."""
        return

    @property
    @dispatched
    def num_occurrences(self):
        """Return how often this result appears in the solution."""
        return

    @property
    @dispatched
    def obj_value(self):
        """
        Get the objective value of this sample. If present, this is the value computed
        by the corresponding AqModel, otherwise a raw energy returned by the solver. If
        none of these values are present, return None.
        """
        return

    @property
    @dispatched
    def constraints(self):
        """
        Get the feasibility of each single constraint of the model the solution was
        created for."""
        return

    @property
    @dispatched
    def feasible(self):
        """
        Return whether all constraints of the model the solution was created for are
        feasible.
        """
        return


@export
class Solution:
    @dispatched
    def __str__(self):  # type: ignore[reportIncompatibleMethodOverride]
        return

    @dispatched
    def __repr__(self):  # type: ignore[reportIncompatibleMethodOverride]
        return

    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __getitem__(self, item):
        return item

    @property
    @dispatched
    def results(self):
        return

    @property
    @dispatched
    def samples(self):
        return

    @property
    @dispatched
    def obj_values(self):
        return

    @property
    @dispatched
    def num_occurrences(self):
        return

    @property
    @dispatched
    def runtime(self):
        return

    @property
    @dispatched
    def best_sample_idx(self):
        return


@export
class Timing:
    @property
    @dispatched
    def start(self):
        return

    @property
    @dispatched
    def end(self):
        return

    @property
    @dispatched
    def total(self):
        return

    @property
    @dispatched
    def total_seconds(self):
        return

    @property
    @dispatched
    def qpu(self):
        return

    @qpu.setter
    @dispatched
    def qpu(self, value):
        return value

    @dispatched
    def add_qpu(self, value: float):
        return


@export
class Timer:
    @dispatched
    @staticmethod
    def start(cls):
        return

    @dispatched
    def stop(self):
        return
