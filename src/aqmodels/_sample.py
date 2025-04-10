from aqmodels._api_utils import export, dispatched


@export
class SamplesIterator:
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

    @dispatched
    def __len__(self):
        return

    @dispatched
    def __iter__(self):
        return

    @dispatched
    def tolist(self):
        return


@export
class Sample:
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
