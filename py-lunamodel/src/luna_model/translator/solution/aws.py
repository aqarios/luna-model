from typing import Any

import numpy as np

from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment


class AwsTranslator:
    @staticmethod
    def to_lm(
        aws_result: dict[str, Any],
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        sol_agg = aws_result["samples"].astype(np.float64, order="C")
        counts = np.ones(sol_agg.shape[0], dtype=np.int64)
        energies = aws_result["energies"].astype(np.float64, order="C")

        return Solution.from_arrays(
            data=sol_agg,
            env=env,
            timing=timing,
            counts=counts,
            energies=energies,
        )
