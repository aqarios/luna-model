from enum import Enum
from typing import overload

from aqmodels._constraints import Constraint
from aqmodels._environment import Environment
from aqmodels._expression import Expression

class Vtype(Enum):
    """
    Enumeration of variable types supported by the optimization system.

    This enum defines the type of a variable used in a model. The type influences
    the domain and behavior of the variable during optimization. It is often passed
    when defining variables to specify how they should behave.

    Attributes
    ----------
    Real : Vtype
        Continuous real-valued variable. Can take any value within given bounds.
    Integer : Vtype
        Discrete integer-valued variable. Takes integer values within bounds.
    Binary : Vtype
        Binary variable. Can only take values 0 or 1.
    Spin : Vtype
        Spin variable. Can only take values -1 or +1.

    Examples
    --------
    >>> from luna_quantum import Vtype
    >>> Vtype.Real
    Real

    >>> str(Vtype.Binary)
    'Binary'
    """

    Real = ...
    """Continuous real-valued variable. Can take any value within given bounds."""
    Integer = ...
    """Discrete integer-valued variable. Takes integer values within bounds."""
    Binary = ...
    """Binary variable. Can only take values 0 or 1."""

    Spin = ...
    """Spin variable. Can only take values -1 or +1."""

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...

class Bounds:
    """
    Represents bounds for a variable (only supported for real and integer variables).

    A `Bounds` object defines the valid interval for a variable. Bounds are inclusive,
    and can be partially specified by providing only a lower or upper limit. If neither
    is specified, the variable is considered unbounded.

    Parameters
    ----------
    lower : float, optional
        Lower bound of the variable. Defaults to negative infinity if not specified.
    upper : float, optional
        Upper bound of the variable. Defaults to positive infinity if not specified.

    Examples
    --------
    >>> from luna_quantum import Bounds
    >>> Bounds(-1.0, 1.0)
    Bounds { lower: -1, upper: 1 }

    >>> Bounds(lower=0.0)
    Bounds { lower: -1, upper: unlimited }

    >>> Bounds(upper=10.0)
    Bounds { lower: unlimited, upper: 1 }

    Notes
    -----
    - Bounds are only meaningful for variables of type `Vtype.Real` or `Vtype.Integer`.
    - If both bounds are omitted, the variable is unbounded.
    """

    @overload
    def __init__(self, /, *, lower: float) -> None: ...
    @overload
    def __init__(self, /, *, upper: float) -> None: ...
    @overload
    def __init__(self, /, lower: float, upper: float) -> None:
        """
        Create bounds for a variable.

        See class-level docstring for full documentation.
        """
        ...

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...

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
    >>> from luna_quantum import Variable, Environment, Vtype, Bounds
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

    @overload
    def __init__(self, /, name: str) -> None: ...
    @overload
    def __init__(self, /, name: str, *, vtype: Vtype) -> None: ...
    @overload
    def __init__(self, /, name: str, *, vtype: Vtype, bounds: Bounds) -> None: ...
    @overload
    def __init__(
        self, /, name: str, *, vtype: Vtype, bounds: Bounds, env: Environment
    ) -> None: ...
    def __init__(
        self,
        /,
        name: str,
        *,
        vtype: Vtype | None = ...,
        bounds: Bounds | None = ...,
        env: Environment | None = ...,
    ) -> None:
        """
        Initialize a new Variable.

        See class-level docstring for full usage.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no active environment is found and none is explicitly provided.
        VariableExistsError
            If a variable with the same name already exists in the environment.
        VariableCreationError
            If the variable is tried to be created with incompatible bounds.
        """
        ...

    @property
    def name(self, /) -> str:
        """Get the name of the variable."""
        ...

    @overload
    def __add__(self, other: int, /) -> Expression: ...
    @overload
    def __add__(self, other: float, /) -> Expression: ...
    @overload
    def __add__(self, other: Variable, /) -> Expression: ...
    @overload
    def __add__(self, other: Expression, /) -> Expression: ...
    def __add__(self, other: int | float | Variable | Expression, /) -> Expression:
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
        ...

    @overload
    def __radd__(self, other: int, /) -> Expression: ...
    @overload
    def __radd__(self, other: float, /) -> Expression: ...
    @overload
    def __radd__(self, other: Variable, /) -> Expression: ...
    @overload
    def __radd__(self, other: Expression, /) -> Expression: ...
    def __radd__(self, other: int | float | Variable | Expression, /) -> Expression:
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
        ...

    @overload
    def __sub__(self, other: int, /) -> Expression: ...
    @overload
    def __sub__(self, other: float, /) -> Expression: ...
    @overload
    def __sub__(self, other: Variable, /) -> Expression: ...
    @overload
    def __sub__(self, other: Expression, /) -> Expression: ...
    def __sub__(self, other: int | float | Variable | Expression, /) -> Expression:
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
        ...

    @overload
    def __rsub__(self, other: int, /) -> Expression: ...
    @overload
    def __rsub__(self, other: float, /) -> Expression: ...
    def __rsub__(self, other: int | float, /) -> Expression:
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
        ...

    @overload
    def __mul__(self, other: int, /) -> Expression: ...
    @overload
    def __mul__(self, other: float, /) -> Expression: ...
    @overload
    def __mul__(self, other: Variable, /) -> Expression: ...
    @overload
    def __mul__(self, other: Expression, /) -> Expression: ...
    def __mul__(self, other: int | float | Variable | Expression, /) -> Expression:
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
        ...

    @overload
    def __rmul__(self, other: int, /) -> Expression: ...
    @overload
    def __rmul__(self, other: float, /) -> Expression: ...
    @overload
    def __rmul__(self, other: Variable, /) -> Expression: ...
    @overload
    def __rmul__(self, other: Expression, /) -> Expression: ...
    def __rmul__(self, other: int | float | Variable | Expression, /) -> Expression:
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
        ...

    def __pow__(self, other: int, /) -> Expression:
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
        ...

    @overload
    def __eq__(self, rhs: int, /) -> Constraint: ...
    @overload
    def __eq__(self, rhs: float, /) -> Constraint: ...
    @overload
    def __eq__(self, rhs: Variable, /) -> Constraint: ...
    @overload
    def __eq__(self, rhs: Expression, /) -> Constraint: ...
    def __eq__(self, rhs: int | float | Variable | Expression, /) -> Constraint:  # type: ignore
        """
        Create a constraint: expression == scalar.

        If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
        constraint, resulting in the following constraint:

            self - rhs == 0

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

    @overload
    def __le__(self, rhs: int, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: float, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: Variable, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: Expression, /) -> Constraint: ...
    def __le__(self, rhs: int | float | Variable | Expression, /) -> Constraint:  # type: ignore
        """
        Create a constraint: expression <= scalar.

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
        ...

    @overload
    def __ge__(self, rhs: int, /) -> Constraint: ...
    @overload
    def __ge__(self, rhs: float, /) -> Constraint: ...
    @overload
    def __ge__(self, rhs: Variable, /) -> Constraint: ...
    @overload
    def __ge__(self, rhs: Expression, /) -> Constraint: ...
    def __ge__(self, rhs: int | float | Variable | Expression, /) -> Constraint:
        """
        Create a constraint: expression >= scalar.

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
        ...

    def __neg__(self, /) -> Expression:
        """
        Negate the variable, i.e., multiply it by `-1`.

        Returns
        -------
        Expression
        """
        ...

    def __hash__(self, /) -> int: ...
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
