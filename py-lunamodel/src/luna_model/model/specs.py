from __future__ import annotations

from luna_model._lm import PyModelSpecs
from luna_model.model.ctype import Ctype
from luna_model.model.sense import Sense
from luna_model.variable import Vtype


class ModelSpecs:
    """Model specifications."""

    _sp: PyModelSpecs

    def __init__(  # noqa: PLR0913
        self,
        sense: Sense | None = None,
        vtypes: set[Vtype] | None = None,
        constraints: set[Ctype] | None = None,
        max_degree: int | None = None,
        max_constraint_degree: int | None = None,
        max_num_variables: int | None = None,
    ) -> None:
        """Create model specifications."""
        self._sp = PyModelSpecs(
            sense=sense._val if sense else None,
            vtypes=[v._val for v in vtypes] if vtypes else None,
            constraints=[c._val for c in constraints] if constraints else None,
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
        """Get sense."""
        pys = self._sp.sense
        if pys:
            return Sense._from_pysense(pys)
        return None

    @property
    def max_degree(self) -> int | None:
        """Get max degree."""
        return self._sp.max_degree

    @property
    def max_constraint_degree(self) -> int | None:
        """Get max constraint degree."""
        return self._sp.max_constraint_degree

    @property
    def max_num_variables(self) -> int | None:
        """Get max num variables."""
        return self._sp.max_num_variables

    @property
    def vtypes(self) -> list[Vtype] | None:
        """Get vtypes."""
        return [Vtype._from_pyvtype(v) for v in self._sp.vtypes]

    @property
    def constraints(self) -> list[Ctype] | None:
        """Get constraints."""
        return [Ctype._from_pyctype(c) for c in self._sp.constraints]

    def satisfies(self, other: ModelSpecs) -> bool:
        """Get satisfies."""
        return self._sp.satisfies(other._sp)
