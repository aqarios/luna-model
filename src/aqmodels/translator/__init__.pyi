# This file is auto-generated.
# Do not edit manually.

from aqmodels import Environment
from aqmodels import Model
from aqmodels import Solution
from aqmodels import Timing
from aqmodels import Variable
from aqmodels import Vtype
from dimod import BinaryQuadraticModel
from dimod import ConstrainedQuadraticModel
from dimod import SampleSet
from numpy.typing import NDArray
from pathlib import Path
from pyscipopt import Model as SciModel
from qiskit.primitives import PrimitiveResult, PubResult
from qiskit_optimization import QuadraticProgram
from typing import Any
from typing import overload

from . import translator

class ZibTranslator:
    @staticmethod
    def from_zib(
        model: SciModel,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class CqmTranslator:
    @staticmethod
    def to_model(cqm: ConstrainedQuadraticModel) -> Model: ...
    @staticmethod
    def from_model(model: Model) -> ConstrainedQuadraticModel: ...

class BqmTranslator:
    @staticmethod
    def to_model(bqm: BinaryQuadraticModel, name: str | None = None) -> Model: ...
    @staticmethod
    def to_bqm(model: Model) -> BinaryQuadraticModel: ...

class DimodTranslator:
    @staticmethod
    def from_dimod_sample_set(
        sample_set: SampleSet,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

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
    def from_ibm(
        result: PrimitiveResult[PubResult], quadratic_program: QuadraticProgram
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_ibm(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_ibm(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
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


__all__ = [
    "BqmTranslator",
    "CqmTranslator",
    "DimodTranslator",
    "IbmTranslator",
    "LpTranslator",
    "MatrixTranslator",
    "QctrlTranslator",
    "ZibTranslator",
    "translator",
]