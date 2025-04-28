from pyscipopt import Model
from aqmodels._environment import Environment
from aqmodels._solution import Solution
from aqmodels._timing import Timing

class ZibTranslator:
    @staticmethod
    def from_zib(
        model: Model,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
