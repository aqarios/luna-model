"""Amazon Braket solution translator for LunaModel.

This module provides translation from Amazon Braket's result format to
LunaModel's Solution format. This enables integration with Amazon Braket's
quantum computing service and quantum annealing devices.
"""

from typing import Any

import numpy as np

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing


class AwsTranslator:
    """Translator for Amazon Braket solution format.

    AwsTranslator converts Amazon Braket's result dictionaries to LunaModel's
    Solution format. Amazon Braket provides access to various quantum hardware
    including gate-based quantum computers and quantum annealers.

    The translator expects result dictionaries containing:
    - 'samples': NumPy array of solution samples
    - 'energies': NumPy array of objective values
    - Optional metadata from the quantum device

    Examples
    --------
    Convert Amazon Braket results to LunaModel solution:

    >>> from braket.aws import AwsDevice
    >>> from braket.ocean_plugin import BraketSampler
    >>> from luna_model.translator import BqmTranslator, AwsTranslator
    >>> from luna_model import Model, Timing
    >>> # Create and solve model on Amazon Braket
    >>> model = Model()
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> model.objective = x * y - 2 * x + y
    >>> bqm = BqmTranslator.from_lm(model)
    >>> # sampler = BraketSampler(AwsDevice("arn:aws:braket:::device/qpu/..."))
    >>> # braket_result = sampler.sample(bqm, shots=100)
    >>> # Assuming braket_result is formatted as dict
    >>> solution = AwsTranslator.to_lm(braket_result)

    With timing information:

    >>> timing = Timing(solver=2.5, total=5.3)
    >>> solution = AwsTranslator.to_lm(braket_result, timing=timing, env=model.env)

    Notes
    -----
    Amazon Braket supports multiple quantum computing paradigms:
    - Quantum annealing (D-Wave-like devices)
    - Gate-based quantum computers (IonQ, Rigetti, etc.)
    - Simulators for testing and development

    The translator is designed to work with results from quantum annealing
    devices and compatible simulators that return samples and energies.

    See Also
    --------
    DwaveTranslator : D-Wave solution translator (similar format)
    IbmTranslator : IBM Qiskit solution translator
    BqmTranslator : BQM format translator for quantum annealers
    """

    @staticmethod
    def to_lm(
        aws_result: dict[str, Any],
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert Amazon Braket result to LunaModel solution.

        Converts an Amazon Braket result dictionary to a LunaModel Solution object.

        Parameters
        ----------
        aws_result : dict[str, Any]
            Amazon Braket result dictionary containing:
            - 'samples': 2D NumPy array of solution samples (n_samples, n_variables)
            - 'energies': 1D NumPy array of objective values (n_samples,)
            - Optional: additional metadata from the quantum device
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment containing variable information for solution mapping.

        Returns
        -------
        Solution
            LunaModel Solution object with samples and energies.

        Examples
        --------
        Basic usage:

        >>> import numpy as np
        >>> aws_result = {"samples": np.array([[0, 1], [1, 0], [1, 1]]), "energies": np.array([-2.5, -1.0, 0.5])}
        >>> solution = AwsTranslator.to_lm(aws_result)
        >>> print(solution.best())

        With model environment:

        >>> from luna_model import Model
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> solution = AwsTranslator.to_lm(aws_result, env=model.env)
        >>> best_sample = solution.best()
        >>> print(f"x={best_sample['x']}, y={best_sample['y']}")

        Notes
        -----
        The translator assigns a count of 1 to each sample since Amazon Braket
        typically returns individual solution samples rather than aggregated counts.
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
