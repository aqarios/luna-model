from aqmodels._api_utils import export, dispatched


@export("translator", "top")
class QctrlTranslator:
    """
    Utility class for converting between a QCTRL solution and an AqSolution (ours).

    `DimodSolutionTranslator` provides mehtods to:
    - Convert a Dimod-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external dimod solvers/samplers or libraries that operate on dimod-based problem solving/sampling.

    Examples
    --------
    >>> import aqmodels as aqm
    >>> ...
    >>> qctrl_result = ...
    >>> aqs = aqm.translator.QctrlTranslator.from_qctrl(qctrl_result)
    """

    @dispatched
    @staticmethod
    def from_qctrl(result, variable_list, timing, env):
        """
        Convert a QCTRL result to an AqSolution.

        Parameters
        ----------
        result : dict[str, Any]
            The qctrl result as a dictionary.
        variable_list : list[Variable], optional
            An optional list of variables to specify the ordering in the result sample.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.
        """
        return result, variable_list, timing, env
