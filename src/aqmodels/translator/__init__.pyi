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

class NumpyTranslator:
    @staticmethod
    def to_aq(
        result: NDArray,
        energies: NDArray,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class ZibTranslator:
    @staticmethod
    def to_aq(
        model: SciModel,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class DwaveTranslator:
    @staticmethod
    def to_aq(
        sample_set: SampleSet,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class CqmTranslator:
    @staticmethod
    def to_aq(cqm: ConstrainedQuadraticModel) -> Model: ...
    @staticmethod
    def from_aq(model: Model) -> ConstrainedQuadraticModel: ...

class Qubo:
    @property
    def offset(self) -> float: ...
    @property
    def matrix(self) -> NDArray: ...
    @property
    def variable_names(self) -> list[str]: ...
    @property
    def name(self) -> str: ...
    @property
    def vtype(self) -> Vtype: ...

class QuboTranslator:
    @staticmethod
    def to_aq(
        qubo: NDArray,
        offset: float | None = ...,
        variable_names: list[str] | None = ...,
        name: str | None = ...,
        vtype: Vtype | None = ...,
    ) -> Model: ...
    @staticmethod
    def from_aq(model: Model) -> Qubo: ...

class BqmTranslator:
    @staticmethod
    def to_aq(bqm: BinaryQuadraticModel, name: str | None = None) -> Model: ...
    @staticmethod
    def from_aq(model: Model) -> BinaryQuadraticModel: ...

class AwsTranslator:
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class QctrlTranslator:
    @overload
    @staticmethod
    def to_aq(result: dict[str, Any]) -> Solution: ...
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
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class IbmTranslator:
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult], quadratic_program: QuadraticProgram
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
        env: Environment | None = ...,
    ) -> Solution: ...

class LpTranslator:
    @overload
    @staticmethod
    def to_aq(file: Path) -> Model: ...
    @overload
    @staticmethod
    def to_aq(file: str) -> Model: ...
    @overload
    @staticmethod
    def from_aq(model: Model) -> str: ...
    @overload
    @staticmethod
    def from_aq(model: Model, file: Path) -> None: ...

__all__ = [
    "AwsTranslator",
    "BqmTranslator",
    "CqmTranslator",
    "DwaveTranslator",
    "IbmTranslator",
    "LpTranslator",
    "NumpyTranslator",
    "QctrlTranslator",
    "Qubo",
    "QuboTranslator",
    "ZibTranslator",
    "translator",
]
