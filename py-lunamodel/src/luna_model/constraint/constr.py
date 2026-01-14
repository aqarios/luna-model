from __future__ import annotations
from typing import TYPE_CHECKING

from luna_model.constraint.cmp import Comparator
from luna_model._utils import wrap_expr

from luna_model._lm import PyConstraint

if TYPE_CHECKING:
    from luna_model.expression.expr import Expression
    from luna_model.variable.var import Variable


class Constraint:
    _c: PyConstraint

    def __init__(
        self,
        lhs: Variable | Expression,
        rhs: float | Expression | Variable,
        comparator: Comparator,
        name: str | None = None,
    ) -> None:
        from luna_model.variable import Variable
        from luna_model.expression import Expression

        lhs = lhs._v if isinstance(lhs, Variable) else lhs._expr  # type: ignore[attribute]
        rhs = (
            (rhs._v if isinstance(rhs, Variable) else rhs._expr)  # type: ignore[attribute]
            if isinstance(rhs, Variable | Expression)  # type: ignore[attribute]
            else rhs
        )
        self._c = PyConstraint(lhs, rhs, comparator._val, name)

    @classmethod
    def _from_pyc(cls, py_c: PyConstraint) -> Constraint:
        """Construct LunaModel Constraint from FFI PyConstraint object."""
        c = cls.__new__(cls)
        c._c = py_c
        return c

    @property
    def name(self) -> str:
        return self._c.name

    @name.setter
    def name(self, name: str) -> None:
        self._c.name = name

    @property
    def lhs(self) -> Expression:
        return wrap_expr(self._c.lhs)

    @property
    def rhs(self) -> float:
        return self._c.rhs

    @property
    def comparator(self) -> Comparator:
        return Comparator._from_pycmp(self._c.comparator)

    def equal_contents(self, other: Constraint) -> bool:
        return self._c.equal_contents(other._c)

    def __eq__(self, other: Constraint) -> bool:  # type: ignore[override]
        return self._c.__eq__(other._c)
