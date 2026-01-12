from luna_model._lm import PyQuboTranslator, PyQubo
from luna_model.model.model import Model
from luna_model.model.sense import Sense
from luna_model.variable.vtype import Vtype
from numpy.typing import NDArray


class Qubo:
    _q: PyQubo

    @property
    def name(self) -> str:
        return self._q.name

    @property
    def variable_names(self) -> list[str]:
        return self._q.variable_names

    @property
    def matrix(self) -> NDArray:
        return self._q.matrix

    @property
    def offset(self) -> float:
        return self._q.offset

    @property
    def vtype(self) -> Vtype:
        return Vtype._from_pyvtype(self._q.vtype)

    @property
    def sense(self) -> Sense:
        return Sense._from_pysense(self._q.sense)


class QuboTranslator:
    @staticmethod
    def to_aq(
        qubo: NDArray,
        *,
        offset: float | None = None,
        variable_names: list[str] | None = None,
        name: str | None = None,
        vtype: Vtype | None = None,
    ) -> Model:
        return Model._from_pym(
            PyQuboTranslator.to_aq(
                qubo,
                offset,
                variable_names,
                name,
                vtype=vtype._val if vtype else None,
            )
        )

    @staticmethod
    def from_aq(model: Model) -> Qubo:
        return PyQuboTranslator.from_aq(model._m)
