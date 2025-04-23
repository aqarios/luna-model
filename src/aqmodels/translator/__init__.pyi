# This file is auto-generated.
# Do not edit manually.

from aqmodels._environment import Environment
from aqmodels._model import Model
from aqmodels._solution import Solution, Timing
from aqmodels._variable import Variable
from aqmodels._variable import Vtype
from dimod import BinaryQuadraticModel
from dimod import SampleSet
from numpy.typing import NDArray
from pathlib import Path
from typing import Any
from typing import overload

from . import translator

class BqmTranslator:
    @staticmethod
    def to_model(bqm: BinaryQuadraticModel, name: str | None = None) -> Model: ...
    @staticmethod
    def to_bqm(model: Model) -> BinaryQuadraticModel: ...

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

class IbmTranslator:
    @overload
    @staticmethod
    def from_ibm(result: Any, quadratic_program: Any) -> Solution: ...
    @overload
    @staticmethod
    def from_ibm(
        result: Any, quadratic_program: Any, timing: Timing | None = ...
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_ibm(
        result: Any,
        quadratic_program: Any,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class LpTranslator:
    @overload
    @staticmethod
    def to_model(file: Path) -> Model: ...
    @overload
    @staticmethod
    def to_model(file: str) -> Model: ...
    @overload
    @staticmethod
    def from_model(model: Model) -> str: ...
    @overload
    @staticmethod
    def from_model(model: Model, file: Path) -> None: ...

class DimodTranslator:
    @staticmethod
    def from_dimod_sample_set(
        sample_set: SampleSet,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class MatrixTranslator:
    @staticmethod
    def to_model(
        qubo: NDArray, name: str | None = ..., vtype: Vtype | None = ...
    ) -> Model: ...
    @staticmethod
    def to_dense(model: Model) -> NDArray: ...

__all__ = [
    "BqmTranslator",
    "DimodTranslator",
    "IbmTranslator",
    "LpTranslator",
    "MatrixTranslator",
    "QctrlTranslator",
    "translator",
]
