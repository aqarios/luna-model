from __future__ import annotations
from typing import TYPE_CHECKING

from luna_model.constraint.constr import Constraint
from luna_model.environment import Environment
from luna_model.expression.expr import Expression
from luna_model.variable.vtype import Vtype
from luna_model.variable.bounds import Bounds, Unbounded

from luna_model._lm import PyVariable, PyExpression

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
    """
    Represents a symbolic variable within an optimization environment.

    A `Variable` is the fundamental building block of algebraic expressions
    used in optimization models. Each variable is tied to an `Environment`
    which scopes its lifecycle and expression context. Variables can be
    typed and optionally bounded.

    Parameters
    ----------
    name : str
        The name of the variable.
    vtype : Vtype, optional
        The variable type (e.g., `Vtype.Real`, `Vtype.Integer`, etc.).
        Defaults to `Vtype.Binary`.
    bounds : Bounds, optional
        Bounds restricting the range of the variable. Only applicable for
        `Real` and `Integer` variables.
    env : Environment, optional
        The environment in which this variable is created. If not provided,
        the current environment from the context manager is used.

    Examples
    --------
    >>> from luna_model import Variable, Environment, Vtype, Bounds
    >>> with Environment():
    ...     x = Variable("x")
    ...     y = Variable("y", vtype=Vtype.Integer, bounds=Bounds(0, 5))
    ...     expr = 2 * x + y - 1

    Arithmetic Overloads
    --------------------
    Variables support standard arithmetic operations:

    - Addition: `x + y`, `x + 2`, `2 + x`
    - Subtraction: `x - y`, `3 - x`
    - Multiplication: `x * y`, `2 * x`, `x * 2`

    All expressions return `Expression` objects and preserve symbolic structure.

    Notes
    -----
    - A `Variable` is bound to a specific `Environment` instance.
    - Variables are immutable; all operations yield new `Expression` objects.
    - Variables carry their environment, but the environment does not own the variable.
    """

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
        """Get the id of the variable."""
        return self._v.id

    @property
    def name(self) -> str:
        """Get the name of the variable."""
        return self._v.name

    @property
    def bounds(self) -> VBounds:
        """Get the bounds of the variable."""
        return Bounds._from_pyb(self._v.bounds)

    @property
    def vtype(self) -> Vtype:
        """Get the vtype of the variable."""
        return Vtype(self._v.vtype)

    @property
    def environment(self) -> Environment:
        """Get this variables's environment."""
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
        """
        Add this variable to another value.

        Parameters
        ----------
        other : int, float, Variable or Expression.

        Returns
        -------
        Expression
            The resulting symbolic expression.

        Raises
        ------
        VariablesFromDifferentEnvsError
            If the operands belong to different environments.
        TypeError
            If the operand type is unsupported.
        """
        return Expression._from_pyexpr(self._op(other, self._v.__add__))

    def __sub__(self, other: Expression | Variable | int | float) -> Expression:
        """
        Subtract a value from this variable.

        Parameters
        ----------
        other : int, float, Variable or Expression.

        Returns
        -------
        Expression
            The resulting symbolic expression.

        Raises
        ------
        VariablesFromDifferentEnvsError
            If the operands belong to different environments.
        TypeError
            If the operand type is unsupported.
        """
        return Expression._from_pyexpr(self._op(other, self._v.__sub__))

    def __mul__(self, other: Expression | Variable | int | float) -> Expression:
        """
        Multiply this variable by another value.

        Parameters
        ----------
        other : Variable, Expression, int, or float

        Returns
        -------
        Expression
            The resulting symbolic expression.

        Raises
        ------
        VariablesFromDifferentEnvsError
            If the operands belong to different environments.
        TypeError
            If the operand type is unsupported.
        """
        return Expression._from_pyexpr(self._op(other, self._v.__mul__))

    def __radd__(self, other: Expression | Variable | int | float) -> Expression:
        """
        Right-hand addition.

        Parameters
        ----------
        other : int, float, Variable or Expression.

        Returns
        -------
        Expression
            The resulting symbolic expression.

        Raises
        ------
        TypeError
            If the operand type is unsupported.
        """
        return Expression._from_pyexpr(self._op(other, self._v.__radd__))

    def __rsub__(self, other: Expression | Variable | int | float) -> Expression:
        """
        Subtract this variable from a scalar (right-hand subtraction).

        Parameters
        ----------
        other : int or float

        Returns
        -------
        Expression
            The resulting symbolic expression.

        Raises
        ------
        TypeError
            If `other` is not a scalar.
        """
        return Expression._from_pyexpr(self._op(other, self._v.__rsub__))

    def __rmul__(self, other: Expression | Variable | int | float) -> Expression:
        """
        Right-hand multiplication for scalars.

        Parameters
        ----------
        other : int or float

        Returns
        -------
        Expression
            The resulting symbolic expression.

        Raises
        ------
        TypeError
            If the operand type is unsupported.
        """
        return Expression._from_pyexpr(self._op(other, self._v.__rmul__))

    def __pow__(self, other: int) -> Expression:
        """
        Raise the variable to the power specified by `other`.

        Parameters
        ----------
        other : int

        Returns
        -------
        Expression

        Raises
        ------
        RuntimeError
            If the param `modulo` usually supported for `__pow__` is specified.
        """
        return Expression._from_pyexpr(self._v.__pow__(other))

    def __neg__(self) -> Expression:
        """
        Negate the variable, i.e., multiply it by `-1`.

        Returns
        -------
        Expression
        """
        return Expression._from_pyexpr(self._v.__neg__())

    def __invert__(self) -> Variable:
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

        Raises
        ------
        UnsupportedOperationErr
            If the operand is a variable of any type other than `Binary`.
        """

        return self._from_pyvar(self._v.__invert__())

    def __eq__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        """
        Create a constraint: Variable == Expression | Variable | int | float.

        If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
        constraint, resulting in the following constraint:

            self - rhs == 0

        Parameters
        ----------
        rhs : float, int or Expression

        Returns
        -------
        Constraint

        Raises
        ------
        TypeError
            If the right-hand side is not of type float, int or Expression.
        """
        return self._cmp(other, self._v.__eq__)

    def __le__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        """
        Create a constraint: Variable <= Expression | Variable | int | float.

        If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
        constraint, resulting in the following constraint:

            self - rhs <= 0

        Parameters
        ----------
        rhs : float, int, Variable or Expression

        Returns
        -------
        Constraint

        Raises
        ------
        TypeError
            If the right-hand side is not of type float, int, Variable or Expression.
        """
        return self._cmp(other, self._v.__le__)

    def __ge__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        """
        Create a constraint: Variable >= Expression | Variable | int | float.

        If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
        constraint, resulting in the following constraint:

            self - rhs >= 0

        Parameters
        ----------
        rhs : float, int, Variable or Expression

        Returns
        -------
        Constraint

        Raises
        ------
        TypeError
            If the right-hand side is not of type float, int, Variable or Expression.
        """
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
