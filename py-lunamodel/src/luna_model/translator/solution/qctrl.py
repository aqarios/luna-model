# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import re
from typing import Any

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing


class QctrlTranslator:
    """Translator for Q-Ctrl solution format.

    Converts Q-Ctrl result dictionaries to LunaModel Solutions.
    Handles variable-to-bitstring mapping and reordering.
    """

    @staticmethod
    def to_lm(
        result: dict[str, Any],
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert Q-Ctrl result to LunaModel solution.

        Parameters
        ----------
        result : dict[str, Any]
            Q-Ctrl result dictionary containing 'final_bitstring_distribution'
            and 'variables_to_bitstring_index_map'.
        timing : Timing, optional
            Timing information for the solution process.
        env : Environment, optional
            Environment for variable mapping. Required either as parameter or active context.

        Returns
        -------
        Solution
            LunaModel Solution with bitstring counts.

        Notes
        -----
        Extracts indices from bracket notation (e.g., 'var[0]') and reorders
        bitstrings accordingly.
        """
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
