# This file is auto-generated.
# Do not edit manually.

from aqmodels._environment import Environment
from aqmodels._model import Model
from aqmodels._solution import Solution, Timing
from aqmodels._variable import Vtype
from dimod import SampleSet
from numpy.typing import NDArray
from pathlib import Path

from . import translator

class SampleSetTranslator:
    @staticmethod
    def from_dimod_sample_set(
        sample_set: SampleSet,
        timing: Timing | None = None,
        env: Environment | None = None,
    ) -> Solution: ...

class LpTranslator:
    @staticmethod
    def to_model(filepath: Path) -> Model: ...
    @staticmethod
    def to_file(model: Model) -> str: ...

class MatrixTranslator:
    @staticmethod
    def to_model(
        qubo: NDArray, name: str | None = ..., vtype: Vtype | None = ...
    ) -> Model: ...
    @staticmethod
    def to_dense(model: Model) -> NDArray: ...


__all__ = [
    "LpTranslator",
    "MatrixTranslator",
    "SampleSetTranslator",
    "translator",
]