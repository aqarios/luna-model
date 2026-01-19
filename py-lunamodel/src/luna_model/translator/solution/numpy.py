from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment

import numpy as np
from numpy.typing import NDArray


class NumpyTranslator:
    @staticmethod
    def to_lm(
        result: NDArray,
        energies: NDArray,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        data = result.astype(np.float64, order="C")
        energies = energies.astype(np.float64, order="C")
        return Solution.from_arrays(
            data,
            energies=energies,
            timing=timing,
            env=env,
        )
