from aqmodels._api_utils import export, dispatched


@export("translator", "top")
class IbmTranslator:
    """
    Utility class for converting between an IBM solution and an AqSolution (ours).


    `IbmTranslator` provides mehtods to:
    - Convert an IBM-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external ibm solvers/samplers or libraries that operate on ibm-based problem solving/sampling.

    Examples
    --------
    >>> import aqmodels as aqm
    >>> ...
    >>> qctrl_result = ...
    >>> aqs = aqm.translator.IbmTranslator.to_aq(ibm_result)
    """

    @dispatched
    @staticmethod
    def to_aq(result, quadratic_program, timing, env):
        """
        Convert an IBM solution to an AqSolution.

        Parameters
        ----------
        result : PrimitiveResult[PubResult]
            The ibm result.
        quadratic_program : QuadraticProgram
            The quadratic program defining the optimization problem.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.
        """
        return result, quadratic_program, timing, env
