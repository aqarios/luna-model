from __future__ import annotations
from typing import TYPE_CHECKING, overload

from luna_model._utils import wrap_b, wrap_env, wrap_expr, wrap_c
from luna_model.variable.vtype import Vtype
from luna_model._lm import PyVariable

if TYPE_CHECKING:
    from luna_model._typing import VBounds
    from luna_model.constraint.constr import Constraint
    from luna_model.environment.env import Environment
    from luna_model.expression.expr import Expression
    from luna_model.variable.bounds import Bounds

    from luna_model._lm import PyExpression


class Variable:
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
            vtype.value,
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
        return self._v.id

    @property
    def name(self) -> str:
        return self._v.name

    @property
    def bounds(self) -> VBounds:
        return wrap_b(self._v.bounds)  # type: ignore[return]

    @property
    def vtype(self) -> Vtype:
        return Vtype(self._v.vtype)

    @property
    def environment(self) -> Environment:
        return wrap_env(self._v.environment)

    def is_equal(self, other: Variable | PyVariable) -> bool:
        return self._v.is_equal(other._v)

    def inv(self) -> Variable:
        return self._from_pyvar(self._v.inv())

    def __add__(self, other: Expression | Variable | int | float) -> Expression:
        return wrap_expr(self._op(other, self._v.__add__))

    def __sub__(self, other: Expression | Variable | int | float) -> Expression:
        return wrap_expr(self._op(other, self._v.__sub__))

    def __mul__(self, other: Expression | Variable | int | float) -> Expression:
        return wrap_expr(self._op(other, self._v.__mul__))

    def __radd__(self, other: Expression | Variable | int | float) -> Expression:
        return wrap_expr(self._op(other, self._v.__radd__))

    def __rsub__(self, other: Expression | Variable | int | float) -> Expression:
        return wrap_expr(self._op(other, self._v.__rsub__))

    def __rmul__(self, other: Expression | Variable | int | float) -> Expression:
        return wrap_expr(self._op(other, self._v.__rmul__))

    def __pow__(self, other: int) -> Expression:
        return wrap_expr(self._v.__pow__(other))

    def __neg__(self) -> Expression:
        return wrap_expr(self._v.__neg__())

    def __invert__(self) -> Variable:
        return self._from_pyvar(self._v.__invert__())

    @overload
    def __eq__(self, other: Variable) -> bool: ...  # type: ignore[override]
    @overload
    def __eq__(self, other: Expression | int | float) -> Constraint: ...  # type: ignore[override]
    def __eq__(self, other: Expression | Variable | int | float) -> Constraint | bool:  # type: ignore[override]
        from luna_model.variable import Variable

        if isinstance(other, Variable):
            return self.is_equal(other)
        return self._cmp(other, self._v.__eq__)

    def __le__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        return self._cmp(other, self._v.__le__)

    def __ge__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        return self._cmp(other, self._v.__ge__)

    def __hash__(self) -> int:
        return self._v.__hash__()

    def __str__(self) -> str:
        return self._v.__str__()

    def __repr__(self) -> str:
        return self._v.__repr__()

    def _op(self, other: Expression | Variable | int | float, fn) -> PyExpression:
        from luna_model.expression import Expression
        from luna_model.variable import Variable

        if isinstance(other, Expression):
            res = fn(other._expr)  # type: ignore[attribute]
        elif isinstance(other, Variable):
            res = fn(other._v)  # type: ignore[attribute]
        else:
            res = fn(other)
        return res

    def _cmp(self, other: Expression | Variable | int | float, fn) -> Constraint:
        from luna_model.expression import Expression
        from luna_model.variable import Variable

        if isinstance(other, Expression):
            pyc = fn(other._expr)  # type: ignore[attribute]
        elif isinstance(other, Variable):
            pyc = fn(other._v)  # type: ignore[attribute]
        else:
            pyc = fn(other)
        return wrap_c(pyc)
