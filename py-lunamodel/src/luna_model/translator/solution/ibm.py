"""IBM Qiskit solution translator for LunaModel.

This module provides translation from IBM Qiskit's quantum computing results
to LunaModel's Solution format. This enables integration with IBM Quantum
systems and Qiskit's optimization algorithms.
"""

# type: ignore[reportPossiblyUnboundVariable]
from typing import TYPE_CHECKING

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing

_QISKIT_AVAILABLE: bool = False
try:
    from qiskit.primitives import PrimitiveResult, PubResult
    from qiskit_optimization import QuadraticProgram

    _QISKIT_AVAILABLE = True
except ImportError:
    _QISKIT_AVAILABLE = False

if TYPE_CHECKING:
    from qiskit.primitives import PrimitiveResult, PubResult
    from qiskit_optimization import QuadraticProgram


class IbmTranslator:
    """Translator for IBM Qiskit solution format.

    IbmTranslator converts IBM Qiskit's PrimitiveResult objects to LunaModel's
    Solution format. This is used when solving optimization problems on IBM
    Quantum hardware or simulators using Qiskit's quantum algorithms.

    The translator processes measurement results from Qiskit quantum circuits,
    extracting bitstring counts and computing objective values using the original
    QuadraticProgram.

    Requires the ``qiskit`` and ``qiskit-optimization`` packages.

    Examples
    --------
    Convert Qiskit results to LunaModel solution:

    >>> from qiskit_algorithms import QAOA
    >>> from qiskit_algorithms.optimizers import COBYLA
    >>> from qiskit.primitives import Sampler
    >>> from qiskit_optimization.algorithms import MinimumEigenOptimizer
    >>> from luna_model.translator import IbmTranslator
    >>> # Assuming quadratic_program is defined
    >>> # qaoa = QAOA(sampler=Sampler(), optimizer=COBYLA())
    >>> # optimizer = MinimumEigenOptimizer(qaoa)
    >>> # result = optimizer.solve(quadratic_program)
    >>> # Convert Qiskit result to LunaModel solution
    >>> solution = IbmTranslator.to_lm(result.min_eigen_solver_result, quadratic_program)

    With timing information:

    >>> from luna_model import Timing
    >>> timing = Timing(solver=5.2, total=7.8)
    >>> solution = IbmTranslator.to_lm(result.min_eigen_solver_result, quadratic_program, timing=timing)

    Notes
    -----
    IBM Qiskit uses a specific bitstring ordering convention. The translator
    automatically handles the reversal of bitstring order to match LunaModel's
    conventions.

    The objective values are computed by evaluating the QuadraticProgram's
    objective function for each measured bitstring.

    See Also
    --------
    AwsTranslator : Amazon Braket solution translator
    QctrlTranslator : Q-Ctrl solution translator
    """

    @staticmethod
    def to_lm(
        result: "PrimitiveResult[PubResult]",
        quadratic_program: "QuadraticProgram",
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert IBM Qiskit result to LunaModel solution.

        Converts IBM Qiskit's PrimitiveResult containing measurement data to
        a LunaModel Solution object.

        Parameters
        ----------
        result : PrimitiveResult[PubResult]
            Qiskit primitive result containing measurement outcomes from
            quantum circuit execution.
        quadratic_program : QuadraticProgram
            The original QuadraticProgram used to create the quantum circuit.
            Required to evaluate objective values for each bitstring.
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment containing variable information for solution mapping.

        Returns
        -------
        Solution
            LunaModel Solution object with bitstring counts and energies.

        Raises
        ------
        RuntimeError
            If ``qiskit`` or ``qiskit-optimization`` packages are not installed.

        Examples
        --------
        >>> from qiskit_algorithms import QAOA
        >>> from qiskit.primitives import Sampler
        >>> from qiskit_optimization import QuadraticProgram
        >>> from luna_model.translator import IbmTranslator
        >>> # Define problem
        >>> qp = QuadraticProgram()
        >>> qp.binary_var("x")
        >>> qp.binary_var("y")
        >>> qp.minimize(linear={"x": -1, "y": -1}, quadratic={("x", "y"): 2})
        >>> # Solve with QAOA
        >>> # qaoa = QAOA(sampler=Sampler(), optimizer=COBYLA())
        >>> # meo = MinimumEigenOptimizer(qaoa)
        >>> # result = meo.solve(qp)
        >>> # Convert to LunaModel solution
        >>> solution = IbmTranslator.to_lm(result.min_eigen_solver_result, qp)
        >>> print(solution.best())

        Notes
        -----
        The bitstring order is reversed during translation to match LunaModel's
        left-to-right variable ordering convention.
        """
        if not _QISKIT_AVAILABLE:
            msg = (
                "qiskit and qiskit_optimization are required for the IbmTranslator. "
                "You can install it using the 'qiskit' extra."
            )
            raise RuntimeError(msg)
        meas = result[0].data.meas
        counts: dict[str, int] = meas.get_counts()

        energies = []

        for bitstring in counts:
            sample = [int(b) for b in bitstring]
            sample = sample[::-1]  # reverse ordering for correct bitstrings.
            energies.append(float(quadratic_program.objective.evaluate(sample)))

        return Solution.from_counts(
            counts,
            energies=energies,
            timing=timing,
            env=env,
        )
