from __future__ import annotations

from luna_model.expression.expr import Expression
from luna_model.variable.var import Variable

from luna_model.constraint.cmp import Comparator

from luna_model._lm import PyConstraint


class Constraint:
    _c: PyConstraint

    def __init__(
        self,
        lhs: Variable | Expression,
        rhs: float | Expression | Variable,
        comparator: Comparator,
        name: str,
    ) -> None:
        lhs = lhs._v if isinstance(lhs, Variable) else lhs._expr
        rhs = (
            (rhs._v if isinstance(rhs, Variable) else rhs._expr)
            if isinstance(rhs, Variable | Expression)
            else rhs
        )
        self._c = PyConstraint(lhs, rhs, comparator.value, name)

    @classmethod
    def _from_pyc(cls, py_c: PyConstraint) -> Constraint:
        """Construct LunaModel Constraint from FFI PyConstraint object."""
        c = cls.__new__(cls)
        c._c = py_c
        return c

    @property
    def name(self) -> str:
        return self._c.name

    @property
    def lhs(self) -> Expression:
        return self._c.lhs

    @property
    def rhs(self) -> float:
        return self._c.rhs

    @property
    def comparator(self) -> Comparator:
        return self._c.comparator

    def __eq__(self, other: Constraint) -> bool:  # type: ignore[override]
        return self._c.__eq__(other._c)
