import re
from typing import Any

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing


class QctrlTranslator:
    @staticmethod
    def to_lm(
        result: dict[str, Any],
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        counts = result.get("final_bitstring_distribution", {})
        mapper = {
            int(re.search(r"\[([^\]]+)\]", k).group(1)): int(v)  # type: ignore[report]
            for k, v in result.get("variables_to_bitstring_index_map", {}).items()
        }

        reordered = {}
        for bitstring, count in counts.items():
            unordered_sample = [int(c) for c in bitstring]
            sample = [unordered_sample[mapper[i]] for i in range(len(unordered_sample))]
            reordered_bitstring = "".join(str(bit) for bit in sample)
            reordered[reordered_bitstring] = count

        return Solution.from_counts(data=reordered, timing=timing, env=env, bit_order="LTR")
