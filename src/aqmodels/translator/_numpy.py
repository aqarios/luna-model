from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class NumpyTranslator:
    """
    Utility class for converting between a result consisting of numpy arrays and an AqSolution (ours).

    `NumpyTranslator` provides methods to:
    - Convert a numpy-array result into our solution `Solution`.

    Examples
    --------
    >>> import aqmodels as aqm
    >>> from numpy.typing import NDArray
    >>> result: NDArray = ...
    >>> energies: NDArray = ...
    >>> aqs = aqm.translator.NumpyTranslator.to_aq(result, energies)
    """

    @dispatched
    @staticmethod
    def to_aq(result, energies, timing, env):
        """
        Convert an IBM solution to an AqSolution.

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
        return result, energies, timing, env
