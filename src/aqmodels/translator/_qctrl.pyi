from typing import Any, overload
from aqmodels._solution import Solution, Timing
from aqmodels._variable import Variable
from aqmodels._environment import Environment

class QctrlTranslator:
    @overload
    @staticmethod
    def from_qctrl(result: dict[str, Any]) -> Solution: ...
    @overload
    @staticmethod
    def from_qctrl(
        result: dict[str, Any],
        variable_list: list[Variable] | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_qctrl(
        result: dict[str, Any],
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_qctrl(
        result: dict[str, Any],
        variable_list: list[Variable] | None = ...,
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_qctrl(
        result: dict[str, Any],
        variable_list: list[Variable] | None = ...,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...
