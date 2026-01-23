import numpy as np

from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment

_DIMOD_AVAILABLE: bool = False
try:
    from dimod import SampleSet  # type: ignore[reportMissingImports]

    _DIMOD_AVAILABLE = True
except ImportError:
    _DIMOD_AVAILABLE = False


class DwaveTranslator:
    @staticmethod
    def to_lm(
        sample_set: SampleSet,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        if not _DIMOD_AVAILABLE:
            raise RuntimeError(
                "dimod is required for the DwaveTranslator. "
                "You can install it using the 'dimod' extra.",
            )
        sampleset = sample_set.aggregate()
        variables = sampleset.variables
        record = sampleset.record

        samples = record.sample.astype(np.float64, order="C")
        counts = record.num_occurrences.astype(np.int64, order="C")
        energies = record.energy.astype(np.float64, order="C")

        return Solution.from_arrays(
            data=samples,
            variables=variables,
            counts=counts,
            energies=energies,
            timing=timing,
            env=env,
        )
