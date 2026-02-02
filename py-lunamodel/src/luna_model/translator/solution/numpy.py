"""NumPy array solution translator for LunaModel.

This module provides translation from NumPy array format to LunaModel's
Solution format. This is useful when working with solvers that return
solutions as NumPy arrays (e.g., custom solvers, simulation results).
"""

import numpy as np
from numpy.typing import NDArray

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing


class NumpyTranslator:
    """Translator for NumPy array solutions.

    NumpyTranslator converts solution data in NumPy array format to LunaModel's
    Solution objects. This is useful for integrating custom solvers or working
    with numerical optimization libraries that return results as arrays.

    The translator expects:
    - A 2D array where each row is a solution sample
    - A 1D array of energy/objective values for each sample
    - Optional timing information
    - Optional environment for variable mapping

    Examples
    --------
    Convert NumPy arrays to LunaModel solution:

    >>> import numpy as np
    >>> from luna_model.translator import NumpyTranslator
    >>> from luna_model import Model
    >>> # Solver returned 3 solutions for 2 variables
    >>> samples = np.array([[0, 1], [1, 0], [1, 1]])
    >>> energies = np.array([-2.5, -1.0, 0.5])
    >>> solution = NumpyTranslator.to_lm(samples, energies)
    >>> print(solution.best())

    With timing and environment:

    >>> from luna_model import Model, Timing
    >>> model = Model()
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> model.objective = x * y - 2 * x + y
    >>> # After solving...
    >>> timing = Timing(solver=10.5, total=12.3)
    >>> solution = NumpyTranslator.to_lm(samples, energies, timing=timing, env=model.env)

    Notes
    -----
    This translator is particularly useful for:
    - Custom solver implementations
    - Simulation and benchmarking
    - Integration with NumPy-based optimization libraries
    - Post-processing of raw numerical results

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

        Converts solution samples and energies from NumPy arrays to a
        LunaModel Solution object.

        Parameters
        ----------
        result : NDArray
            2D array of solution samples where each row represents a solution.
            Shape should be (n_samples, n_variables).
        energies : NDArray
            1D array of energy/objective values for each sample.
            Shape should be (n_samples,).
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment containing variable information for mapping array
            indices to variable names. If None, variables are indexed by position.

        Returns
        -------
        Solution
            LunaModel Solution object containing the samples and energies.

        Examples
        --------
        Basic usage:

        >>> import numpy as np
        >>> samples = np.array([[1, 0, 1], [0, 1, 1], [1, 1, 0]])
        >>> energies = np.array([-5.0, -3.0, -4.0])
        >>> solution = NumpyTranslator.to_lm(samples, energies)

        With variable names from environment:

        >>> from luna_model import Model
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> z = model.add_variable("z")
        >>> samples = np.array([[1.0, 0.0, 1.0]])
        >>> energies = np.array([-5.0])
        >>> solution = NumpyTranslator.to_lm(samples, energies, env=model.env)
        >>> print(solution.best())  # Uses variable names x, y, z

        Notes
        -----
        Arrays are automatically converted to float64 and C-contiguous layout
        for optimal performance.
        """
        data = result.astype(np.float64, order="C")
        energies = energies.astype(np.float64, order="C")
        return Solution.from_arrays(
            data,
            energies=energies.tolist(),
            timing=timing,
            env=env,
        )
