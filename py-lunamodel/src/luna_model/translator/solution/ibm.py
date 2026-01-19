from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment

# TODO: try, else default and error...
from qiskit.primitives import PrimitiveResult, PubResult  # type: ignore[import]
from qiskit_optimization import QuadraticProgram  # type: ignore[import]


class IbmTranslator:
    @staticmethod
    def to_lm(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
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
