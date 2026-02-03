"""Q-Ctrl solution translator for LunaModel.

This module provides translation from Q-Ctrl results to
LunaModel's Solution format.
"""

import re
from typing import Any

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing


class QctrlTranslator:
    """Translator for Q-Ctrl solution format.

    Converts Q-Ctrl result dictionaries to LunaModel Solutions.
    Handles variable-to-bitstring mapping and reordering.

    Examples
    --------
    >>> from luna_model.translator import QctrlTranslator
    >>> qctrl_result = {
    ...     "final_bitstring_distribution": {"01": 45, "10": 35},
    ...     "variables_to_bitstring_index_map": {"x[0]": 0, "y[1]": 1},
    ... }
    >>> solution = QctrlTranslator.to_lm(qctrl_result)

    See Also
    --------
    IbmTranslator : IBM Qiskit solution translator
    AwsTranslator : Amazon Braket solution translator
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
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment for variable mapping.

        Returns
        -------
        Solution
            LunaModel Solution with bitstring counts.

        Examples
        --------
        >>> qctrl_result = {
        ...     "final_bitstring_distribution": {"00": 10, "01": 25},
        ...     "variables_to_bitstring_index_map": {"x[0]": 0, "y[1]": 1},
        ... }
        >>> solution = QctrlTranslator.to_lm(qctrl_result)

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
