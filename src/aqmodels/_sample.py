from aqmodels._api_utils import export, dispatched


@export
class SamplesIterator:
    """
    An iterator over a solution's samples.

    Examples
    --------
    >>> from aqmodels import Solution
    >>> solution: Solution = ...

    Note: ``solution.samples`` is automatically converted into a ``SamplesIterator``.

    >>> for sample in solution.samples:
    ...     sample
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
class SampleIterator:
    """
    An iterator over the variable assignments of a solution's sample.

    Examples
    --------
    >>> from aqmodels import Solution
    >>> solution: Solution = ...
    >>> sample = solution.samples[0]

    Note: ``sample`` is automatically converted into a ``SampleIterator``.

    >>> for var in sample:
    ...     var
    0
    -5
    0.28
    """

    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __next__(self):
        return


@export
class Samples:
    """
    A samples object is simply the set that contains every different sample of a
    solution.

    The ``Samples`` class is readonly as it's merely a helper class for looking into a
    solution's different samples.

    Examples
    --------
    >>> from aqmodels import Model, Sample, Solution
    >>> model: Model = ...
    >>> solution: Solution = ...
    >>> samples: Samples = solution.samples
    >>> samples
    [0, -5, 0.28]
    [1, -4, -0.42]
    """

    @dispatched
    def __str__(self):  # type: ignore
        return

    @dispatched
    def __getitem__(self, item):
        return item

    @dispatched
    def __len__(self):
        return

    @dispatched
    def __iter__(self):
        return

    @dispatched
    def tolist(self):
        """
        Convert the sample into a 2-dimensional list where a row constitutes a single
        sample, and a column constitutes all assignments for a single variable.

        Returns
        -------
        list[list[int | float]]
            The samples object as a 2-dimensional list.
        """

        return


@export
class Sample:
    """
    A sample object is an assignment of an actual value to each of the models'
    variables.

    The ``Sample`` class is readonly as it's merely a helper class for looking into a
    single sample of a solution.

    Note: a ``Sample`` can be converted to ``list[int | float]`` simply by calling
    ``list(sample)``.

    Examples
    --------
    >>> from aqmodels import Model, Sample, Solution
    >>> model: Model = ...
    >>> solution: Solution = ...
    >>> sample: Sample = solution.samples[0]
    >>> sample
    [0, -5, 0.28]
    """

    @dispatched
    def __str__(self):
        return

    @dispatched
    def __getitem__(self, item):
        return item

    @dispatched
    def __len__(self):
        return

    @dispatched
    def __iter__(self):
        return
