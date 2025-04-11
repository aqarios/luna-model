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
        Get the objective value of this sample if present. This is the value computed
        by the corresponding AqModel.
        """
        return

    @property
    @dispatched
    def raw_energy(self):
        """
        Get the raw energy returned by the solver if present. This value is not
        guaranteed to be accurate under consideration of the corresponding AqModel.
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
