from __future__ import annotations
from typing import Any as PySolution

from luna_model.solution.iter import ResultIter
# from luna_model._lm import PySolution


class Solution:
    """
    The solution object that is obtained by running an algorihtm.

    The `Solution` class represents a summary of all data obtained from solving a
    model. It contains samples, i.e., assignments of values to each model variable as
    returned by the algorithm, metadata about the solution quality, e.g., the objective
    value, and the runtime of the algorithm.

    A `Solution` can be constructed explicitly using `from_dict` or by obtaining a
    solution from an algorithm or by converting a different solution format with one of
    the available translators. Note that the latter requires the environment the model
    was created in.

    Examples
    --------
    Basic usage, assuming that the algorithm already returns a `Solution`:

    >>> from luna_model import Model, Solution
    >>> model: Model = ...
    >>> algorithm = ...
    >>> solution: Solution = algorithm.run(model)
    >>> solution.samples
    [[1, 0, 1], [0, 0, 1]]

    When you have a `dimod.Sampleset` as the raw solution format:

    >>> from luna_model.translator import BqmTranslator
    >>> from luna_model import Model, Solution, DwaveTranslator
    >>> from dimod import SimulatedAnnealingSampler
    >>> model: Model = ...
    >>> bqm = BqmTranslator.from_aq(model)
    >>> sampleset = SimulatedAnnealingSampler().sample(bqm)
    >>> solution = DwaveTranslator.from_dimod_sample_set(sampleset)
    >>> solution.samples
    [[1, 0, 1], [0, 0, 1]]

    Serialization:

    >>> blob = solution.encode()
    >>> restored = Solution.decode(blob)
    >>> restored.samples
    [[1, 0, 1], [0, 0, 1]]

    Notes
    -----
    - To ensure metadata like objective values or feasibility, use
      `model.evaluate(solution)`.
    - Use `encode()` and `decode()` to serialize and recover solutions.
    """

    _s: PySolution

    @classmethod
    def from_(cls) -> Solution: ...

    @classmethod
    def _from_pys(cls, py_s: PySolution) -> Solution:
        s = cls.__new__(cls)
        s._s = py_s
        return s

    def __len__(self) -> int:
        return self._s.__len__()

    def __iter__(self, /) -> ResultIter:
        return self._s.__iter__()
