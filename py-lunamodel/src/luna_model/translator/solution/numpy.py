"""NumPy array solution translator for LunaModel.

This module provides translation from NumPy array format to
LunaModel's Solution format.
"""

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
    >>> from luna_model.translator import NumpyTranslator
    >>> samples = np.array([[0, 1], [1, 0], [1, 1]])
    >>> energies = np.array([-2.5, -1.0, 0.5])
    >>> solution = NumpyTranslator.to_lm(samples, energies)

    See Also
    --------
    DwaveTranslator : D-Wave SampleSet solution translator
    Solution : LunaModel solution object
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
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment for mapping array indices to variable names.

        Returns
        -------
        Solution
            LunaModel Solution with samples and energies.

        Examples
        --------
        >>> import numpy as np
        >>> samples = np.array([[1, 0, 1], [0, 1, 1]])
        >>> energies = np.array([-5.0, -3.0])
        >>> solution = NumpyTranslator.to_lm(samples, energies)
        """
        data = result.astype(np.float64, order="C")
        energies = energies.astype(np.float64, order="C")
        return Solution.from_arrays(
            data,
            energies=energies.tolist(),
            timing=timing,
            env=env,
        )
