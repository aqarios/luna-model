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
    def variable_names(self) -> list[Variable]: ...
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
