from typing import overload

from numpy.typing import NDArray

from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class NumpyTranslator:
    """
        Utility class for converting between a result consisting of numpy arrays and our solution
        format.

        `NumpyTranslator` provides methods to:
        - Convert a numpy-array result into our solution `Solution`.

        Examples
        --------
        >>> import luna_quantum as lq
        >>> from numpy.typing import NDArray
        >>> result: NDArray = ...
        >>> energies: NDArray = ...
        >>> aqs = lq.translator.NumpyTranslator.to_aq(result, energies)
    #[pyclass(unsendable, name = "NumpyTranslator", module = "aqmodels.translat
    """
    @overload
    @staticmethod
    def to_aq(result: NDArray, energies: NDArray) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: NDArray,
        energies: NDArray,
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: NDArray, energies: NDArray, *, env: Environment | None
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: NDArray,
        energies: NDArray,
        timing: Timing | None = ...,
        *,
        env: Environment | None,
    ) -> Solution:
        """
        Convert an IBM solution to our solution format.

        Parameters
        ----------
        result : NDArray
            The samples as a 2D array where each row corresponds to one sample.
        energies : NDArray
            The energies of the single samples as a 1D array.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment is passed to the method or available from the context.
        SolutionTranslationError
            Generally if the solution translation fails. Might be specified by one of the
                two following errors.
        SampleIncorrectLengthError
            If a solution's sample has a different number of variables than the model
            environment passed to the translator.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...
