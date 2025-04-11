# This file is auto-generated.
# Do not edit manually.

from aqmodels._environment import Environment
from aqmodels._model import Model
from aqmodels._solution import Solution, Timing
from aqmodels._variable import Vtype
from dimod import BinaryQuadraticModel
from dimod import SampleSet
from numpy.typing import NDArray

from . import translator

class BqmTranslator:
    @staticmethod
    def to_model(
            bqm: BinaryQuadraticModel, name: str | None = None
    ) -> Model: ...

    @staticmethod
    def to_bqm(model: Model) -> BinaryQuadraticModel: ...

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


__all__ = [
    "BqmTranslator",
    "MatrixTranslator",
    "SampleSetTranslator",
    "translator",
]