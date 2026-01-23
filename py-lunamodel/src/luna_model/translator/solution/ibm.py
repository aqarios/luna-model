from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment

_QISKIT_AVAILABLE: bool = False
try:
    from qiskit.primitives import PrimitiveResult, PubResult  # type: ignore[reportMissingImports]
    from qiskit_optimization import QuadraticProgram  # type: ignore[reportMissingImports]

    _QISKIT_AVAILABLE = True
except ImportError:
    _QISKIT_AVAILABLE = False


class IbmTranslator:
    @staticmethod
    def to_lm(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        if not _QISKIT_AVAILABLE:
            raise RuntimeError(
                "qiskit and qiskit_optimization are required for the IbmTranslator. "
                "You can install it using the 'qiskit' extra."
            )
        meas = result[0].data.meas
        counts: dict[str, int] = meas.get_counts()

        energies = []

        for bitstring, _ in counts.items():
            sample = []
            for b in bitstring:
                sample.append(int(b))
            sample = sample[::-1]  # reverse ordering for correct bitstrings.
            energies.append(float(quadratic_program.objective.evaluate(sample)))

        return Solution.from_counts(
            counts,
            energies=energies,
            timing=timing,
            env=env,
        )
