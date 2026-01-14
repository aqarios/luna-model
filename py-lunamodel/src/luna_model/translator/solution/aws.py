from typing import Any

import numpy as np

from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment


class AwsTranslator:
    @staticmethod
    def to_lm(
        result: dict[str, Any],
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        (sol_agg, num_occ) = np.unique(
            aws_result["samples"], return_counts=True, axis=0
        )
        energies = aws_result["energies"]

        sol_agg = sol_agg.astype(np.float64, order="C")
        num_occ = num_occ.astype(np.uint64, order="C")
        energies = energies.astype(np.float64, order="C")

        return Solution.from_arrays(
            data=sol_agg,
            env=env,
            timing=timing,
            counts=num_occ,
            energies=energies,
       )
