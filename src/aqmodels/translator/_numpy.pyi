from numpy.typing import NDArray

from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class NumpyTranslator:
    @staticmethod
    def to_aq(
        result: NDArray,
        energies: NDArray,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
