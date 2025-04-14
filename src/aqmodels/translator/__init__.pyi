# This file is auto-generated.
# Do not edit manually.

from aqmodels._environment import Environment
from aqmodels._model import Model
from aqmodels._solution import Solution, Timing
from aqmodels._variable import Vtype
from dimod import SampleSet
from numpy.typing import NDArray
from pathlib import Path
from typing import overload

from . import translator

class SampleSetTranslator:
    @staticmethod
    def from_dimod_sample_set(
        sample_set: SampleSet,
        timing: Timing | None = None,
        env: Environment | None = None,
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
    def from_model(model: Model, filepath: Path) -> None: ...


__all__ = [
    "LpTranslator",
    "MatrixTranslator",
    "SampleSetTranslator",
    "translator",
]