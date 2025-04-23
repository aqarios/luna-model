from dimod import SampleSet
from aqmodels._environment import Environment
from aqmodels._solution import Solution, Timing

class DimodTranslator:
    @staticmethod
    def from_dimod_sample_set(
        sample_set: SampleSet,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
