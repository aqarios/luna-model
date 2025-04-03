# This file is auto-generated.
# Do not edit manually.

from aqmodels import Environment
from aqmodels._solution import Solution, Timing
from dimod import SampleSet

from . import solution_translator

class SampleSetTranslator:
    @staticmethod
    def from_dimod_sample_set(
            sample_set: SampleSet,
            timing: Timing | None = None,
            env: Environment | None = None,
    ) -> Solution: ...


__all__ = [
    "SampleSetTranslator",
    "solution_translator",
]