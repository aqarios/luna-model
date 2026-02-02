"""Q-Ctrl solution translator for LunaModel.

This module provides translation from Q-Ctrl's quantum optimization results
to LunaModel's Solution format. Q-Ctrl provides quantum control and optimization
solutions for quantum computing applications.
"""

import re
from typing import Any

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing


class QctrlTranslator:
    """Translator for Q-Ctrl solution format.

    QctrlTranslator converts Q-Ctrl's result dictionaries to LunaModel's
    Solution format. Q-Ctrl specializes in quantum control solutions that
    optimize quantum operations and solve optimization problems on quantum hardware.

    The translator handles Q-Ctrl's specific result format which includes:
    - Bitstring distribution with occurrence counts
    - Variable-to-bitstring index mapping
    - Metadata from quantum optimization

    The translator automatically reorders bitstrings according to the correct
    variable mapping provided by Q-Ctrl.

    Examples
    --------
    Convert Q-Ctrl results to LunaModel solution:

    >>> from luna_model.translator import QctrlTranslator
    >>> from luna_model import Timing
    >>> # Q-Ctrl result format
    >>> qctrl_result = {
    ...     "final_bitstring_distribution": {"01": 45, "10": 35, "11": 20},
    ...     "variables_to_bitstring_index_map": {"x[0]": 0, "y[1]": 1},
    ... }
    >>> solution = QctrlTranslator.to_lm(qctrl_result)
    >>> print(solution.best())

    With timing and environment:

    >>> from luna_model import Model
    >>> model = Model()
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> timing = Timing(solver=3.2, total=4.5)
    >>> solution = QctrlTranslator.to_lm(qctrl_result, timing=timing, env=model.env)

    Notes
    -----
    Q-Ctrl's variable naming convention uses bracket notation (e.g., 'var[0]')
    to indicate bitstring positions. The translator parses these indices and
    reorders bitstrings accordingly to match the original variable order.

    The translator uses left-to-right bit ordering, matching Q-Ctrl's convention.

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

        Converts a Q-Ctrl result dictionary to a LunaModel Solution object,
        handling the variable-to-bitstring mapping and reordering.

        Parameters
        ----------
        result : dict[str, Any]
            Q-Ctrl result dictionary containing:
            - 'final_bitstring_distribution': Dictionary mapping bitstrings to
              occurrence counts
            - 'variables_to_bitstring_index_map': Dictionary mapping variable
              names (with indices in brackets) to bitstring positions
            - Optional: additional Q-Ctrl metadata
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment containing variable information for solution mapping.

        Returns
        -------
        Solution
            LunaModel Solution object with bitstring counts.

        Examples
        --------
        Basic usage:

        >>> qctrl_result = {
        ...     "final_bitstring_distribution": {"00": 10, "01": 25, "10": 40, "11": 25},
        ...     "variables_to_bitstring_index_map": {"x[0]": 0, "y[1]": 1},
        ... }
        >>> solution = QctrlTranslator.to_lm(qctrl_result)
        >>> print(f"Best solution appears {max(solution.counts())} times")

        With model context:

        >>> from luna_model import Model
        >>> model = Model()
        >>> vars = [model.add_variable(f"v{i}") for i in range(3)]
        >>> model.objective = sum(vars)
        >>> # After Q-Ctrl optimization...
        >>> solution = QctrlTranslator.to_lm(qctrl_result, env=model.env)
        >>> best = solution.best()

        Notes
        -----
        The translator extracts indices from variable names using regex pattern
        matching on the bracket notation (e.g., 'var[0]' -> 0). It then reorders
        each bitstring according to these indices to ensure correct variable
        assignment.

        Bitstrings are processed in left-to-right order as specified by the
        bit_order="LTR" parameter.
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
