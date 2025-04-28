from dimod import SampleSet
from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class DimodTranslator:
    @staticmethod
    def from_dimod_sample_set(
        sample_set: SampleSet,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
