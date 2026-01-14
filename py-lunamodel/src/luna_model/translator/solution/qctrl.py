from typing import Any

from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment


class QctrlTranslator:
    @staticmethod
    def to_lm(
        result: dict[str, Any],
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        return Solution.from_counts(
            data=result.get("final_bitstring_distribution", {}),
            timing=timing,
            env=env,
        )

