from aqmodels._api_utils import export, dispatched


@export("translator", "top")
class QctrlTranslator:
    """
    Utility class for converting between a QCTRL solution and an AqSolution (ours).

    `QctrlTranslator` provides methods to:
    - Convert a Qctrl-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external qctrl solvers/samplers or libraries that operate on qctrl-based problem solving/sampling.

    Examples
    --------
    >>> import aqmodels as aqm
    >>> ...
    >>> qctrl_result = ...
    >>> aqs = aqm.translator.QctrlTranslator.to_aq(qctrl_result)
    """

    @dispatched
    @staticmethod
    def to_aq(result, timing, env):
        """
        Convert a QCTRL result to an AqSolution.

        Parameters
        ----------
        result : dict[str, Any]
            The qctrl result as a dictionary.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.
        """
        return result, timing, env
