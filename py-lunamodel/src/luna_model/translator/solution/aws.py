"""Amazon Braket solution translator for LunaModel.

This module provides translation from Amazon Braket results to
LunaModel's Solution format.
"""

from typing import Any

import numpy as np

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing


class AwsTranslator:
    """Translator for Amazon Braket solution format.

    Converts Amazon Braket result dictionaries to LunaModel Solutions.

    Examples
    --------
    >>> from luna_model.translator import AwsTranslator
    >>> # braket_result = sampler.sample(bqm, shots=100)
    >>> solution = AwsTranslator.to_lm(braket_result)

    See Also
    --------
    DwaveTranslator : D-Wave solution translator
    BqmTranslator : BQM format translator
    """

    @staticmethod
    def to_lm(
        aws_result: dict[str, Any],
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert Amazon Braket result to LunaModel solution.

        Parameters
        ----------
        aws_result : dict[str, Any]
            Amazon Braket result dictionary containing 'samples' and 'energies'.
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment for variable mapping.

        Returns
        -------
        Solution
            LunaModel Solution with samples and energies.

        Examples
        --------
        >>> import numpy as np
        >>> aws_result = {"samples": np.array([[0, 1], [1, 0]]), "energies": np.array([-2.5, -1.0])}
        >>> solution = AwsTranslator.to_lm(aws_result)
        """
        sol_agg = aws_result["samples"].astype(np.float64, order="C")
        counts = np.ones(sol_agg.shape[0], dtype=np.int64)
        energies = aws_result["energies"].astype(np.float64, order="C")

        return Solution.from_arrays(
            data=sol_agg,
            env=env,
            timing=timing,
            counts=counts.tolist(),
            energies=energies,
        )
