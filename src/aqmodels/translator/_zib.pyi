from pyscipopt import Model
from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class ZibTranslator:
    @staticmethod
    def from_zib(
        model: Model,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
