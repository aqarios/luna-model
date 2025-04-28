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
    >>> aqs = aqm.translator.AwsTranslator.from_aws_result(aws_result)
    """

    @dispatched
    @staticmethod
    def from_aws_result(result, timing, env):
        return result, timing, env
