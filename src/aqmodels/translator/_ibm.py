from aqmodels._api_utils import export, dispatched


@export("translator", "top")
class IbmTranslator:
    """
    Utility class for converting between an IBM solution and an AqSolution (ours).


    `IbmTranslator` provides methods to:
    - Convert an IBM-style solution into our solution `Solution`.

    The conversions are especially required when interaction with external ibm solvers/samplers or libraries that operate on ibm-based problem solving/sampling.

    Examples
    --------
    >>> import aqmodels as aqm
    >>> ...
    >>> ibm_result = ...
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
        return result, quadratic_program, timing, env
