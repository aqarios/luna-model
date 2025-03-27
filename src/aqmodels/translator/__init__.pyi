# This file is auto-generated.
# Do not edit manually.

from aqmodels._model import Model
from aqmodels._variable import Vtype
from numpy.typing import NDArray

from . import translator

class MatrixTranslator:
    @staticmethod
    def to_model(
        qubo: NDArray, name: str | None = ..., vtype: Vtype | None = ...
    ) -> Model: ...
    @staticmethod
    def to_dense(model: Model) -> NDArray: ...


__all__ = [
    "MatrixTranslator",
    "translator",
]