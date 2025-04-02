from dimod import SampleSet

from aqmodels._solution import Solution, Timing


class SampleSetTranslator:
    @staticmethod
    def from_dimod_sample_set(
            sample_set: SampleSet, timing: Timing | None = None
    ) -> Solution: ...
