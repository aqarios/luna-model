from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class AwsTranslator:
    """
    Utility class for converting between an AWS result and an AqSolution (ours).

    `AwsTranslator` provides methods to:
    - Convert an AWS-style result into our solution `Solution`.

    The conversions are especially required when interaction with external aws solvers/samplers or libraries that operate on aws-based problem solving/sampling.

    Examples
    --------
    >>> import aqmodels as aqm
    >>> aws_result = ...
    >>> aqs = aqm.translator.AwsTranslator.to_aq(aws_result)
    """

    @dispatched
    @staticmethod
    def to_aq(result, timing, env):
        """
        Convert an AWS Braket result to an AqSolution.

        Parameters
        ----------
        result : dict[str, Any]
            The aws braket result.
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
