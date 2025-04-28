from dimod import SampleSet

from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class DwaveTranslator:
    @staticmethod
    def to_aq(
        sample_set: SampleSet,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
