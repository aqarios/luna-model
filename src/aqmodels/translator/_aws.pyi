from typing import Any

from aqmodels._environment import Environment
from aqmodels._solution import Solution
from aqmodels._timing import Timing

class AwsTranslator:
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
