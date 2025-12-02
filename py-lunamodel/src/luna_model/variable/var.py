from __future__ import annotations
from typing import TYPE_CHECKING

from luna_model._lm import PyVariable, PyExpression
from luna_model.constraint.constr import Constraint
from luna_model.environment import Environment
from luna_model.expression.expr import Expression
from luna_model.variable.vtype import Vtype
from luna_model.variable.bounds import Bounds, Unbounded

if TYPE_CHECKING:
    from typing import Protocol
    from luna_model._lm import PyBounds

    class VBounds(Protocol):
        @property
        def _b(self) -> PyBounds: ...
        @property
        def upper(self) -> float | type[Unbounded]: ...
        @property
        def lower(self) -> float | type[Unbounded]: ...


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
        """Construct LunaModel Variable from FFI PyVariable object."""
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
        return Bounds._from_pyb(self._v.bounds)

    @property
    def vtype(self) -> Vtype:
        return Vtype(self._v.vtype)

    @property
    def environment(self) -> Environment:
        return Environment._from_pyenv(self._v.environment)

    def inv(self) -> Variable:
        """Invert a binary variable.

        This operation is only supported on Binary variables. For all other variable
        types it raises the
        `UnsupportedOperationError`.

        Inversion of a binary variable `b`: `~b` is equivalent to the
        expression: `(1 - b)`.

        Using inversion, or the alternative `b.inv()` method is a first
        level primitive in luna-model, i.e., internally a `~b` is not treated
        as a `(1 - b)` expression but implies internal optimizations in the model
        representation. Using `~b` or `b.inv()` will give you a **new** variable
        that represents the inverse of `b`. The variable type of this inversed
        binary variable `~b` is `Vtype.InverseBinary`. Inversing `~b` again: `~(~b)`
        will return the original variable `b` with variable type `Vtype.Binary`.

        Returns
        -------
        Variable
            The inverse variable of `self`.
        """
        return self._from_pyvar(self._v.inv())

    def __add__(self, other: Expression | Variable | int | float) -> Expression:
        return Expression._from_pyexpr(self._op(other, self._v.__add__))

    def __sub__(self, other: Expression | Variable | int | float) -> Expression:
        return Expression._from_pyexpr(self._op(other, self._v.__sub__))

    def __mul__(self, other: Expression | Variable | int | float) -> Expression:
        return Expression._from_pyexpr(self._op(other, self._v.__mul__))

    def __radd__(self, other: Expression | Variable | int | float) -> Expression:
        return Expression._from_pyexpr(self._op(other, self._v.__radd__))

    def __rsub__(self, other: Expression | Variable | int | float) -> Expression:
        return Expression._from_pyexpr(self._op(other, self._v.__rsub__))

    def __rmul__(self, other: Expression | Variable | int | float) -> Expression:
        return Expression._from_pyexpr(self._op(other, self._v.__rmul__))

    def __pow__(self, other: int) -> Expression:
        return Expression._from_pyexpr(self._v.__pow__(other))

    def __neg__(self) -> Expression:
        return Expression._from_pyexpr(self._v.__neg__())

    def __invert__(self) -> Variable:
        return self._from_pyvar(self._v.__invert__())

    def __eq__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        return self._cmp(other, self._v.__eq__)

    def __le__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        return self._cmp(other, self._v.__le__)

    def __ge__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        return self._cmp(other, self._v.__ge__)

    def __hash__(self) -> int:
        return self._v.__hash__()

    def _op(self, other: Expression | Variable | int | float, fn) -> PyExpression:
        if isinstance(other, Expression):
            res = fn(other._expr)
        elif isinstance(other, Variable):
            res = fn(other._v)
        else:
            res = fn(other)
        return res

    def _cmp(self, other: Expression | Variable | int | float, fn) -> Constraint:
        if isinstance(other, Expression):
            pyc = fn(other._expr)
        elif isinstance(other, Variable):
            pyc = fn(other._v)
        else:
            pyc = fn(other)
        return Constraint._from_pyc(pyc)
