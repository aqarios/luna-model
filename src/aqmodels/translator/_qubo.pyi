from numpy.typing import NDArray
from aqmodels import Model
from aqmodels import Variable
from aqmodels import Vtype

class Qubo:
    @property
    def offset(self) -> float: ...
    @property
    def matrix(self) -> NDArray: ...
    @property
    def variable_ordering(self) -> list[Variable]: ...

class QuboTranslator:
    @staticmethod
    def to_aq(
        qubo: NDArray, name: str | None = ..., vtype: Vtype | None = ...
    ) -> Model: ...
    @staticmethod
    def from_aq(model: Model) -> Qubo: ...
