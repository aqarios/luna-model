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
    >>> import luna_quantum as lq
    >>> ...
    >>> qctrl_result = ...
    >>> aqs = lq.translator.QctrlTranslator.to_aq(qctrl_result)
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
        return result, timing, env
