from typing import Any
from typing import overload
from aqmodels import Solution
from aqmodels import Timing
from aqmodels import Variable
from aqmodels import Environment

class QctrlTranslator:
    @overload
    @staticmethod
    def to_aq(result: dict[str, Any]) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        variable_list: list[Variable] | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        variable_list: list[Variable] | None = ...,
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        variable_list: list[Variable] | None = ...,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
