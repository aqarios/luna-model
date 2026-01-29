from __future__ import annotations

from luna_model._lm import PyConstraint
from luna_model._utils import wrap_expr
from luna_model.constraint.cmp import Comparator
from luna_model.expression.expr import Expression
from luna_model.variable.var import Variable


class Constraint:
    """Constraint docstring."""

    _c: PyConstraint

    def __init__(
        self,
        lhs: Variable | Expression,
        rhs: float | Expression | Variable,
        comparator: Comparator,
        name: str | None = None,
    ) -> None:
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
        """Get the constraint's name."""
        return self._c.name

    @name.setter
    def name(self, name: str) -> None:
        """Set the constraint's name."""
        self._c.name = name

    @property
    def lhs(self) -> Expression:
        """Get the constraint's left-hand side."""
        return wrap_expr(self._c.lhs)

    @property
    def rhs(self) -> float:
        """Get the constraint's right-hand side."""
        return self._c.rhs

    @property
    def comparator(self) -> Comparator:
        """Get the constraint's comparator."""
        return Comparator._from_pycmp(self._c.comparator)

    def equal_contents(self, other: Constraint) -> bool:
        """Check if two constraints have equal contents."""
        return self._c.equal_contents(other._c)

    def __eq__(self, other: Constraint) -> bool:  # type: ignore[override]
        """Check two constraints are equal (exactly)."""
        return self._c.__eq__(other._c)

    def __str__(self) -> str:
        """Get constraint as string (human readable)."""
        return self._c.__str__()

    def __repr__(self) -> str:
        """Get constraint debug representation."""
        return self._c.__repr__()
