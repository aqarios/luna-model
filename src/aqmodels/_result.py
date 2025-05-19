from aqmodels._api_utils import export, dispatched


@export
class ResultIterator:
    """
    An iterator over a solution's results.

    Examples
    --------
    >>> from luna_quantum import ResultIterator, Solution
    >>> solution: Solution = ...
    >>> results: ResultIterator = solution.results
    >>> for result in results:
    ...     result.sample
    [0, -5, 0.28]
    [1, -4, -0.42]
    """

    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __next__(self):
        return


@export
class Result:
    """
    A result object can be understood as a solution with only one sample.

    It can be obtained by calling ``model.evaluate_sample`` for a single sample.

    Most properties available for the solution object are also available for a result,
    but in the singular form. For example, you can call ``solution.obj_values``, but
    ``result.obj_value``.

    Examples
    --------
    >>> from luna_quantum import Model, Result, Solution
    >>> model: Model = ...
    >>> solution: Solution = ...
    >>> sample = solution.samples[0]
    >>> result = model.evaluate_sample(sample)
    >>> result.obj_value
    -109.42
    >>> result.sample
    [0, -5, 0.28]
    >>> result.constraints
    [True, False]
    >>> result.feasible
    False
    """

    @dispatched
    def __str__(self):
        return

    @dispatched
    def __repr__(self):
        return

    @property
    @dispatched
    def sample(self):
        """Get the sample of the result."""
        return

    @property
    @dispatched
    def obj_value(self):
        """Get the objective value of the result."""
        return

    @property
    @dispatched
    def constraints(self):
        """
        Get this result's feasibility values of all constraints. Note that
        `results.constraints[i]` iff. `model.constraints[i]` is feasible for
        this result.
        """
        return

    @property
    @dispatched
    def feasible(self):
        """Return whether all constraint results are feasible for this result."""
        return


@export
class ResultView:
    """
    A result view object serves as a view into one row of a solution object.

    The ``Result`` class is readonly as it's merely a helper class for looking into a
    solution's row, i.e., a single sample and this sample's metadata.

    Most properties available for the solution object are also available for a result,
    but in the singular form. For example, you can call ``solution.obj_values``, but
    ``result.obj_value``.

    Examples
    --------
    >>> from luna_quantum import ResultView, Solution
    >>> solution: Solution = ...
    >>> result: ResultView = solution[0]
    >>> result.obj_value
    -109.42
    >>> result.sample
    [0, -5, 0.28]
    >>> result.constraints
    [True, False]
    >>> result.feasible
    False
    """

    @dispatched
    def __str__(self):
        return

    @dispatched
    def __repr__(self):
        return

    @property
    @dispatched
    def sample(self):
        """Get the sample of the result."""
        return

    @property
    @dispatched
    def counts(self):
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
        Get the raw energy returned by the algorithm if present. This value is not
        guaranteed to be accurate under consideration of the corresponding AqModel.
        """
        return

    @property
    @dispatched
    def constraints(self):
        """
        Get this result's feasibility values of all constraints. Note that
        `results.constraints[i]` iff. `model.constraints[i]` is feasible for
        this result.
        """
        return

    @property
    @dispatched
    def feasible(self):
        """Return whether all constraint results are feasible for this result."""
        return
