from __future__ import annotations

from luna_model.model.sense import Sense
from luna_model.variable.vtype import Vtype
from luna_model.model.constr import ConstraintType

from luna_model._lm import PyModelSpecs


class ModelSpecs:
    _sp: PyModelSpecs

    def __init__(
        self,
        sense: Sense | None = None,
        vtypes: list[Vtype] | None = None,
        constraints: list[ConstraintType] | None = None,
        max_degree: int | None = None,
        max_constraint_degree: int | None = None,
        max_num_variables: int | None = None,
    ) -> None:
        self._v = PyModelSpecs(
            sense=sense.value if sense else None,
            vtypes=[v.value for v in vtypes] if vtypes else None,
            constraints=[c.value for c in constraints] if constraints else None,
            max_degree=max_degree,
            max_constraint_degree=max_constraint_degree,
            max_num_variables=max_num_variables,
        )

    @classmethod
    def _from_pysp(cls, py_specs: PyModelSpecs) -> ModelSpecs:
        """Construct LunaModel ModelSpecs from FFI PyModelSpecs object."""
        specs = cls.__new__(cls)
        specs._sp = py_specs
        return specs

    @property
    def sense(self) -> Sense | None:
        pys = self._sp.sense
        if pys:
            return Sense(pys)
        return None

    @property
    def max_degree(self) -> int | None:
        return self._sp.max_degree

    @property
    def max_constraint_degree(self) -> int | None:
        return self._sp.max_constraint_degree

    @property
    def max_num_variables(self) -> int | None:
        return self._sp.max_num_variables

    @property
    def vtypes(self) -> list[Vtype] | None:
        return [Vtype(v) for v in self._sp.vtypes]

    @property
    def constraints(self) -> list[ConstraintType] | None:
        return [ConstraintType(c) for c in self._sp.constraints]

    def satisfies(self, other: ModelSpecs) -> bool:
        return self._sp.satisfies(other._sp)
