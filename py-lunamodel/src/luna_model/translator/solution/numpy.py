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

import numpy as np
from numpy.typing import NDArray

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing


class NumpyTranslator:
    """Translator for NumPy array solutions.

    Converts solution data in NumPy array format to LunaModel Solutions.

    Examples
    --------
    >>> import numpy as np
    >>> from luna_model import Environment, Variable
    >>> from luna_model.translator import NumpyTranslator
    >>> samples = np.array([[0, 1], [1, 0], [1, 1]])
    >>> energies = np.array([-2.5, -1.0, 0.5])
    >>> env = Environment()
    >>> with env:
    ...     _ = Variable("x")
    ...     _ = Variable("y")
    >>> solution = NumpyTranslator.to_lm(samples, energies, env=env)
    >>> print(solution)
    x y │ feas  raw obj count
    0 1 │    ? -2.5   ?     1
    1 0 │    ? -1.0   ?     1
    1 1 │    ?  0.5   ?     1
    <BLANKLINE>
    Total samples: 3
    Unique samples: 3
    Total variables: 2
    """

    @staticmethod
    def to_lm(
        result: NDArray,
        energies: NDArray,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert NumPy arrays to LunaModel solution.

        Parameters
        ----------
        result : NDArray
            2D array of solution samples, shape (n_samples, n_variables).
        energies : NDArray
            1D array of energy/objective values, shape (n_samples,).
        timing : Timing, optional
            Timing information for the solution process.
        env : Environment, optional
            Environment for mapping array indices to variable names. Required either as parameter or active context.

        Returns
        -------
        Solution
            LunaModel Solution with samples and energies.
        """
        data = result.astype(np.float64, order="C")
        energies = energies.astype(np.float64, order="C")
        return Solution.from_arrays(
            data,
            energies=energies.tolist(),
            timing=timing,
            env=env,
        )
