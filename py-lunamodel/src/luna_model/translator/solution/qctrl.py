import re
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
        counts = result.get("final_bitstring_distribution", {})
        mapper = {
            int(re.search(r"\[([^\]]+)\]", k).group(1)): int(v)
            for k, v in result.get("variables_to_bitstring_index_map", {}).items()
        }
        print(mapper)

        # Reorder bitstrings and create the result
        reordered = {}

        for bitstring, count in counts.items():
            # Convert bitstring to list of integers
            unordered_sample = [int(c) for c in bitstring]

            # Reorder according to mapper
            # For each position i, take the bit from position mapper[i]
            sample = [unordered_sample[mapper[i]] for i in range(len(unordered_sample))]

            # Convert back to bitstring
            reordered_bitstring = "".join(str(bit) for bit in sample)

            # Store with count
            reordered[reordered_bitstring] = count

        return Solution.from_counts(
            data=reordered,
            timing=timing,
            env=env,
        )
