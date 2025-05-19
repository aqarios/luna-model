from typing import Any, overload

from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class AwsTranslator:
    """
    Utility class for converting between an AWS result and our solution format.

    `AwsTranslator` provides methods to:
    - Convert an AWS-style result into our solution `Solution`.

    The conversions are especially required when interacting with external aws solvers/samplers or
    libraries that operate on aws-based problem-solving/sampling.

    Examples
    --------
    >>> import luna_quantum as lq
    >>> aws_result = ...
    >>> aqs = lq.translator.AwsTranslator.to_aq(aws_result)
    """

    @overload
    @staticmethod
    def to_aq(result: dict[str, Any]) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(result: dict[str, Any], timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(result: dict[str, Any], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: dict[str, Any], timing: Timing, *, env: Environment
    ) -> Solution: ...
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        timing: Timing | None = ...,
        *,
        env: Environment | None = ...,
    ) -> Solution:
        """
        Convert an AWS Braket result to our solution format.

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
        ...
