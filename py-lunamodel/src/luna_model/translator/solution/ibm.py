# type: ignore[reportPossiblyUnboundVariable]
from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing

_QISKIT_AVAILABLE: bool = False
try:
    from qiskit.primitives import PrimitiveResult, PubResult
    from qiskit_optimization import QuadraticProgram

    _QISKIT_AVAILABLE = True
except ImportError:
    _QISKIT_AVAILABLE = False


class IbmTranslator:
    """Ibm solution translator."""

    @staticmethod
    def to_lm(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Translate ibm solution to luna model solution."""
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
