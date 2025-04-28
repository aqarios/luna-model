from pyscipopt import Model as SciModel

from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class ZibTranslator:
    @staticmethod
    def to_aq(
        model: SciModel,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
