from numpy.typing import NDArray
from aqmodels import Model
from aqmodels import Vtype

class MatrixTranslator:
    @staticmethod
    def to_aq(
        qubo: NDArray, name: str | None = ..., vtype: Vtype | None = ...
    ) -> Model: ...
    @staticmethod
    def from_aq(model: Model) -> NDArray: ...
