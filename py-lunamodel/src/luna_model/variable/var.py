# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
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
    """A decision variable in an optimization model.

    Variables represent unknowns in an optimization problem that are determined
    by the optimization process. Each variable has a name, type (binary, integer,
    spin, or real), optional bounds, and belongs to an environment.

    Variables can be combined using arithmetic operations (+, -, *, **) to create
    :class:`Expression` objects, and compared using relational operators (==, <=, >=)
    to create :class:`Constraint` objects.

    Parameters
    ----------
    name : str
        The unique name identifying this variable within its environment.
    vtype : Vtype, default=Vtype.BINARY
        The type of the variable. Must be one of:

        - ``Vtype.BINARY``: Variable can be 0 or 1
        - ``Vtype.SPIN``: Variable can be -1 or +1
        - ``Vtype.INTEGER``: Variable can be any integer
        - ``Vtype.REAL``: Variable can be any real number

    bounds : Bounds | tuple[float | None, float | None] | None, optional
        The bounds constraining the variable's value. Can be:

        - A :class:`Bounds` object specifying lower and upper bounds
        - A tuple ``(lower, upper)`` where ``None`` indicates unbounded
        - ``None`` to use default bounds based on ``vtype``

        Default bounds by type:

        - ``BINARY``: [0, 1]
        - ``SPIN``: [-1, 1]
        - ``INTEGER``: [-2^63, 2^63-1]
        - ``REAL``: [-inf, inf]

    env : Environment | None, optional
        The environment managing this variable. If ``None``, uses the currently
        active environment context or creates a new one.

    Attributes
    ----------
    id : int
        Unique integer identifier for this variable within its environment.
    name : str
        The name of the variable.
    bounds : tuple[float | None, float | None]
        The lower and upper bounds of the variable.
    vtype : Vtype
        The type of the variable.
    environment : Environment
        The environment containing this variable.

    Examples
    --------
    Create binary variables for a knapsack problem:

    >>> from luna_model import Variable, Vtype
    >>> x1 = Variable("x1")  # Binary by default
    >>> x2 = Variable("x2", vtype=Vtype.BINARY)

    Create integer variables with bounds:

    >>> from luna_model import Variable, Vtype
    >>> y = Variable("y", vtype=Vtype.INTEGER, bounds=(0, 10))

    Create expressions using arithmetic:

    >>> x = Variable("x")
    >>> y = Variable("y")
    >>> expr = 3 * x + 2 * y - 5  # Creates an Expression

    Create constraints using comparisons:

    >>> x = Variable("x")
    >>> y = Variable("y")
    >>> constraint = x + y <= 1  # Creates a Constraint

    Notes
    -----
    Variables are immutable once created. Their name, type, and environment
    cannot be changed. Bounds can be modified through the model that contains them.

    See Also
    --------
    Expression : Symbolic mathematical expressions built from variables
    Constraint : Constraints on variable values
    Model : Optimization models containing variables
    Environment : Container managing variables and their metadata
    Vtype : Enumeration of variable types
    Bounds : Explicit bounds specification
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
        """Get the unique identifier of the variable.

        Returns
        -------
        int
            Unique integer identifier within the variable's environment.

        Notes
        -----
        Variable IDs are assigned sequentially within an environment and
        are used internally for efficient lookups and comparisons.
        """
        return self._v.id

    @property
    def name(self) -> str:
        """Get the name of the variable.

        Returns
        -------
        str
            The variable's name as specified at creation.
        """
        return self._v.name

    @property
    def bounds(self) -> VBounds:
        """Get the bounds of the variable.

        Returns
        -------
        tuple[float | None, float | None]
            A tuple ``(lower, upper)`` where ``None`` indicates unbounded.

        Examples
        --------
        >>> from luna_model import Variable, Vtype
        >>> x = Variable("x", vtype=Vtype.INTEGER, bounds=(0, 10))
        >>> x.bounds
        (0.0, 10.0)
        """
        return wrap_b(self._v.bounds)  # type: ignore[return]

    @property
    def vtype(self) -> Vtype:
        """Get the type of the variable.

        Returns
        -------
        Vtype
            The variable type (BINARY, SPIN, INTEGER, or REAL).
        """
        return Vtype._from_pyvtype(self._v.vtype)

    @property
    def environment(self) -> Environment:
        """Get the environment containing this variable.

        Returns
        -------
        Environment
            The environment managing this variable's metadata.
        """
        return wrap_env(self._v.environment)

    def is_equal(self, other: Variable) -> bool:
        """Check if this variable is the same as another variable.

        Parameters
        ----------
        other : Variable
            The variable to compare with.

        Returns
        -------
        bool
            ``True`` if both variables refer to the same underlying variable
            (same ID and environment), ``False`` otherwise.

        Notes
        -----
        This compares variable identity, not just name equality. Two variables
        with the same name in different environments are not equal.

        Examples
        --------
        >>> from luna_model import Variable
        >>> x1 = Variable("x")
        >>> x2 = Variable("x")  # Different variable, same name
        >>> x1.is_equal(x2)
        False
        >>> x1.is_equal(x1)
        True
        """
        return self._v.is_equal(other._v)

    def inv(self) -> Variable:
        """Create an inverted version of this variable.

        For SPIN variables, inversion maps -1 ↔ +1.
        For BINARY variables, inversion maps 0 ↔ 1.

        Returns
        -------
        Variable
            A new inverted variable.

        Notes
        -----
        Inverted variables maintain a relationship with their original variable
        in the environment. This is primarily used for spin transformations.

        Examples
        --------
        >>> from luna_model import Variable, Vtype
        >>> x = Variable("x", vtype=Vtype.SPIN)
        >>> x_inv = x.inv()
        >>> # If x = -1, then x_inv = +1
        """
        return self._from_pyvar(self._v.inv())

    def __add__(self, other: Expression | Variable | float) -> Expression:
        """Add this variable to another term.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to add to this variable.

        Returns
        -------
        Expression
            A new expression representing the sum.

        Examples
        --------
        >>> from luna_model import Variable
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> expr = x + y  # Linear expression
        >>> expr2 = x + 5.0  # Variable plus constant
        """
        return wrap_expr(self._op(other, self._v.__add__))

    def __sub__(self, other: Expression | Variable | float) -> Expression:
        """Subtract another term from this variable.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to subtract from this variable.

        Returns
        -------
        Expression
            A new expression representing the difference.
        """
        return wrap_expr(self._op(other, self._v.__sub__))

    def __mul__(self, other: Expression | Variable | float) -> Expression:
        """Multiply this variable by another term.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to multiply with this variable.

        Returns
        -------
        Expression
            A new expression representing the product.

        Examples
        --------
        >>> from luna_model import Variable
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> expr = 3 * x  # Linear term
        >>> expr2 = x * y  # Quadratic term
        """
        return wrap_expr(self._op(other, self._v.__mul__))

    def __radd__(self, other: Expression | Variable | float) -> Expression:
        """Add this variable to another term (right operand).

        Parameters
        ----------
        other : Expression | Variable | float
            The term to add this variable to.

        Returns
        -------
        Expression
            A new expression representing the sum.
        """
        return wrap_expr(self._op(other, self._v.__radd__))

    def __rsub__(self, other: Expression | Variable | float) -> Expression:
        """Subtract this variable from another term (right operand).

        Parameters
        ----------
        other : Expression | Variable | float
            The term to subtract this variable from.

        Returns
        -------
        Expression
            A new expression representing the difference.
        """
        return wrap_expr(self._op(other, self._v.__rsub__))

    def __rmul__(self, other: Expression | Variable | float) -> Expression:
        """Multiply another term by this variable (right operand).

        Parameters
        ----------
        other : Expression | Variable | float
            The term to multiply by this variable.

        Returns
        -------
        Expression
            A new expression representing the product.
        """
        return wrap_expr(self._op(other, self._v.__rmul__))

    def __pow__(self, val: int) -> Expression:
        """Raise this variable to a power.

        Parameters
        ----------
        val : int
            The exponent (must be a positive integer).

        Returns
        -------
        Expression
            A new expression representing this variable raised to the power.

        Examples
        --------
        >>> from luna_model import Variable
        >>> x = Variable("x")
        >>> expr = x**2  # Quadratic term
        >>> expr2 = x**3  # Cubic (higher-order) term
        """
        return wrap_expr(self._v.__pow__(val))

    def __neg__(self) -> Expression:
        """Negate this variable.

        Returns
        -------
        Expression
            A new expression representing the negation of this variable.

        Examples
        --------
        >>> from luna_model import Variable
        >>> x = Variable("x")
        >>> expr = -x  # Negated variable
        """
        return wrap_expr(self._v.__neg__())

    def __invert__(self) -> Variable:
        """Invert this variable using the ~ operator.

        Returns
        -------
        Variable
            A new inverted variable.

        See Also
        --------
        inv : Detailed documentation of variable inversion
        """
        return self._from_pyvar(self._v.__invert__())

    @overload
    def __eq__(self, other: Variable) -> bool: ...  # type: ignore[override]
    @overload
    def __eq__(self, other: Expression | float) -> Constraint: ...  # type: ignore[override]
    def __eq__(self, other: Expression | Variable | float) -> Constraint | bool:  # type: ignore[override]
        """Check equality or create an equality constraint.

        When comparing with another Variable, performs identity comparison.
        When comparing with an Expression or float, creates an equality constraint.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to compare with.

        Returns
        -------
        Constraint | bool
            - ``bool`` if comparing with another Variable (identity check)
            - ``Constraint`` if comparing with Expression or float (constraint creation)

        Examples
        --------
        >>> from luna_model import Variable
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> # Identity comparison
        >>> x == x  # True
        >>> x == y  # False
        >>> # Constraint creation
        >>> constraint = x == 5  # Creates equality constraint
        >>> constraint2 = x + y == 10
        """
        if isinstance(other, Variable):
            return self.is_equal(other)
        return self._cmp(other, self._v.__eq__)

    def __le__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create a less-than-or-equal-to constraint.

        Parameters
        ----------
        other : Expression | Variable | float
            The right-hand side of the inequality.

        Returns
        -------
        Constraint
            A constraint representing ``self <= other``.

        Examples
        --------
        >>> from luna_model import Variable
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> constraint = x <= 10
        >>> constraint2 = x + y <= 5
        """
        return self._cmp(other, self._v.__le__)

    def __ge__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create a greater-than-or-equal-to constraint.

        Parameters
        ----------
        other : Expression | Variable | float
            The right-hand side of the inequality.

        Returns
        -------
        Constraint
            A constraint representing ``self >= other``.

        Examples
        --------
        >>> from luna_model import Variable
        >>> x = Variable("x")
        >>> constraint = x >= 0
        """
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
