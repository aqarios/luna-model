from typing import Any, TYPE_CHECKING

import numpy as np
from dimod import SampleSet

from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment


class DwaveTranslator:
    @staticmethod
    def to_aq(
        sample_set: SampleSet,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
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