from typing import Any

from aqmodels import Environment
from aqmodels import Solution
from aqmodels import Timing

class AwsTranslator:
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
