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
        return result, energies, timing, env
