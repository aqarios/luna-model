from __future__ import annotations

from typing import TYPE_CHECKING, overload

from luna_model._lm import PyVariable
from luna_model._utils import wrap_b, wrap_c, wrap_env, wrap_expr
from luna_model.variable.vtype import Vtype

if TYPE_CHECKING:
    from collections.abc import Callable

    from luna_model._lm import PyExpression
    from luna_model._typing import VBounds
    from luna_model.constraint.constr import Constraint
    from luna_model.environment.env import Environment
    from luna_model.expression.expr import Expression
    from luna_model.variable.bounds import Bounds


class Variable:
    """The Variable."""

    _v: PyVariable

    def __init__(
        self,
        name: str,
        vtype: Vtype = Vtype.BINARY,
        bounds: Bounds | VBounds | None = None,
        env: Environment | None = None,
    ) -> None:
        self._v = PyVariable(
            name,
            vtype._val,
            bounds._b if bounds else None,
            env._env if env else None,
        )

    @classmethod
    def _from_pyvar(cls, py_var: PyVariable) -> Variable:
        var = cls.__new__(cls)
        var._v = py_var
        return var

    @property
    def id(self) -> int:
        """The id of the variable."""
        return self._v.id

    @property
    def name(self) -> str:
        """The name of the variable."""
        return self._v.name

    @property
    def bounds(self) -> VBounds:
        """The bounds of the variable."""
        return wrap_b(self._v.bounds)  # type: ignore[return]

    @property
    def vtype(self) -> Vtype:
        """The type of the variable."""
        return Vtype._from_pyvtype(self._v.vtype)

    @property
    def environment(self) -> Environment:
        """The environment of the variable."""
        return wrap_env(self._v.environment)

    def is_equal(self, other: Variable) -> bool:
        """Check equality with another variable."""
        return self._v.is_equal(other._v)

    def inv(self) -> Variable:
        """Invert this variable producing a new inverted variable."""
        return self._from_pyvar(self._v.inv())

    def __add__(self, other: Expression | Variable | float) -> Expression:
        """Add `other` to this variable returning an expression."""
        return wrap_expr(self._op(other, self._v.__add__))

    def __sub__(self, other: Expression | Variable | float) -> Expression:
        """Sub `other` to this variable returning an expression."""
        return wrap_expr(self._op(other, self._v.__sub__))

    def __mul__(self, other: Expression | Variable | float) -> Expression:
        """Mul `other` to this variable returning an expression."""
        return wrap_expr(self._op(other, self._v.__mul__))

    def __radd__(self, other: Expression | Variable | float) -> Expression:
        """Right add this variable to `other` returning an expression."""
        return wrap_expr(self._op(other, self._v.__radd__))

    def __rsub__(self, other: Expression | Variable | float) -> Expression:
        """Right sub this variable to `other` returning an expression."""
        return wrap_expr(self._op(other, self._v.__rsub__))

    def __rmul__(self, other: Expression | Variable | float) -> Expression:
        """Right mul this variable to `other` returning an expression."""
        return wrap_expr(self._op(other, self._v.__rmul__))

    def __pow__(self, val: int) -> Expression:
        """Raise this variable to the power producing an expression."""
        return wrap_expr(self._v.__pow__(val))

    def __neg__(self) -> Expression:
        """Negate this variable producing an expression."""
        return wrap_expr(self._v.__neg__())

    def __invert__(self) -> Variable:
        """Invert this variable producing a new inverted variable."""
        return self._from_pyvar(self._v.__invert__())

    @overload
    def __eq__(self, other: Variable) -> bool: ...  # type: ignore[override]
    @overload
    def __eq__(self, other: Expression | float) -> Constraint: ...  # type: ignore[override]
    def __eq__(self, other: Expression | Variable | float) -> Constraint | bool:  # type: ignore[override]
        """Check for equality."""
        if isinstance(other, Variable):
            return self.is_equal(other)
        return self._cmp(other, self._v.__eq__)

    def __le__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create le constraint."""
        return self._cmp(other, self._v.__le__)

    def __ge__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create ge constraint."""
        return self._cmp(other, self._v.__ge__)

    def __hash__(self) -> int:
        """Compoute hash."""
        return self._v.__hash__()

    def __str__(self) -> str:
        """Get str representation."""
        return self._v.__str__()

    def __repr__(self) -> str:
        """Get repr representation."""
        return self._v.__repr__()

    def _op(
        self,
        other: Expression | Variable | float,
        fn: Callable[[PyExpression | PyVariable | float], PyExpression],
    ) -> PyExpression:
        from luna_model.expression import Expression  # noqa: PLC0415

        if isinstance(other, Expression):
            res = fn(other._expr)
        elif isinstance(other, Variable):
            res = fn(other._v)
        else:
            res = fn(other)
        return res

    def _cmp(
        self,
        other: Expression | Variable | float,
        fn: Callable[[PyExpression | PyVariable | float], PyExpression],
    ) -> Constraint:
        from luna_model.expression import Expression  # noqa: PLC0415

        if isinstance(other, Expression):
            pyc = fn(other._expr)
        elif isinstance(other, Variable):
            pyc = fn(other._v)
        else:
            pyc = fn(other)
        return wrap_c(pyc)
