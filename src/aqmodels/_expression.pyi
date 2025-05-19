from typing import overload

from aqmodels._constraints import Constraint
from aqmodels._environment import Environment
from aqmodels._variable import Variable

class Expression:
    """
    Polynomial expression supporting symbolic arithmetic, constraint creation, and encoding.

    An `Expression` represents a real-valued mathematical function composed of variables,
    scalars, and coefficients. Expressions may include constant, linear, quadratic, and
    higher-order terms (cubic and beyond). They are used to build objective functions
    and constraints in symbolic optimization models.

    Expressions support both regular and in-place arithmetic, including addition and
    multiplication with integers, floats, `Variable` instances, and other `Expression`s.

    Parameters
    ----------
    env : Environment, optional
        Environment used to scope the expression when explicitly instantiating it.
        Typically, expressions are constructed implicitly via arithmetic on variables.

    Examples
    --------
    Constructing expressions from variables:

    >>> from luna_quantum import Environment, Variable
    >>> with Environment():
    ...     x = Variable("x")
    ...     y = Variable("y")
    ...     expr = 1 + 2 * x + 3 * x * y + x * y * y

    Inspecting terms:

    >>> expr.get_offset()
    1.0
    >>> expr.get_linear(x)
    2.0
    >>> expr.get_quadratic(x, y)
    3.0
    >>> expr.get_higher_order((x, y, y))
    1.0

    In-place arithmetic:

    >>> expr += x
    >>> expr *= 2

    Creating constraints:

    >>> constraint = expr == 10.0
    >>> constraint2 = expr <= 15

    Serialization:

    >>> blob = expr.encode()
    >>> restored = Expression.decode(blob)

    Supported Arithmetic
    --------------------
    The following operations are supported:

    - Addition:
        * `expr + expr` → `Expression`
        * `expr + variable` → `Expression`
        * `expr + int | float` → `Expression`
        * `int | float + expr` → `Expression`

    - In-place addition:
        * `expr += expr`
        * `expr += variable`
        * `expr += int | float`

    - Multiplication:
        * `expr * expr`
        * `expr * variable`
        * `expr * int | float`
        * `int | float * expr`

    - In-place multiplication:
        * `expr *= expr`
        * `expr *= variable`
        * `expr *= int | float`

    - Constraint creation:
        * `expr == constant` → `Constraint`
        * `expr <= constant` → `Constraint`
        * `expr >= constant` → `Constraint`

    Notes
    -----
    - Expressions are mutable: in-place operations (`+=`, `*=`) modify the instance.
    - Expressions are scoped to an environment via the variables they reference.
    - Comparisons like `expr == expr` return `bool`, not constraints.
    - Use `==`, `<=`, `>=` with numeric constants to create constraints.
    """

    @overload
    def __init__(self, /) -> None: ...
    @overload
    def __init__(self, /, env: Environment) -> None: ...
    def __init__(self, /, env: Environment | None = ...) -> None:
        """
         Create a new empty expression scoped to an environment.

         Parameters
         ----------
         env : Environment
             The environment to which this expression is bound.

        Raises
         ------
         NoActiveEnvironmentFoundError
             If no environment is provided and none is active in the context.
        """
        ...

    def get_offset(self, /) -> float:
        """
        Get the constant (offset) term in the expression.

        Returns
        -------
        float
            The constant term.
        """
        ...

    def get_linear(self, /, variable: Variable) -> float:
        """
        Get the coefficient of a linear term for a given variable.

        Parameters
        ----------
        variable : Variable
            The variable whose linear coefficient is being queried.

        Returns
        -------
        float
            The coefficient, or 0.0 if the variable is not present.

        Raises
        ------
        VariableOutOfRangeError
            If the variable index is not valid in this expression's environment.
        """
        ...

    def get_quadratic(self, /, u: Variable, v: Variable) -> float:
        """
        Get the coefficient for a quadratic term (u * v).

        Parameters
        ----------
        u : Variable
        v : Variable

        Returns
        -------
        float
            The coefficient, or 0.0 if not present.

        Raises
        ------
        VariableOutOfRangeError
            If either variable is out of bounds for the expression's environment.
        """
        ...

    def get_higher_order(self, /, variables: tuple[Variable, ...]) -> float:
        """
        Get the coefficient for a higher-order term (degree ≥ 3).

        Parameters
        ----------
        variables : tuple of Variable
            A tuple of variables specifying the term.

        Returns
        -------
        float
            The coefficient, or 0.0 if not present.

        Raises
        ------
        VariableOutOfRangeError
            If any variable is out of bounds for the environment.
        """
        ...
    @property
    def num_variables(self, /) -> int:
        """
        Return the number of distinct variables in the expression.

        Returns
        -------
        int
            Number of variables with non-zero coefficients.
        """
        ...

    def is_equal(self, /, other: Expression) -> bool:
        """
        Compare two expressions for equality.

        Parameters
        ----------
        other : Expression
            The expression to which `self` is compared to.

        Returns
        -------
        bool
            If the two expressions are equal.
        """
        ...

    @overload
    def encode(self, /) -> bytes: ...
    @overload
    def encode(self, /, compress: bool) -> bytes: ...
    @overload
    def encode(self, /, *, level: int) -> bytes: ...
    @overload
    def encode(self, /, compress: bool, level: int) -> bytes: ...
    def encode(self, /, compress: bool | None = ..., level: int | None = ...) -> bytes:
        """
        Serialize the expression into a compact binary format.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the data. Default is True.
        level : int, optional
            Compression level (0–9). Default is 3.

        Returns
        -------
        bytes
            Encoded representation of the expression.

        Raises
        ------
        IOError
            If serialization fails.
        """
        ...

    @overload
    def serialize(self, /) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool) -> bytes: ...
    @overload
    def serialize(self, /, *, level: int) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool, level: int) -> bytes: ...
    def serialize(self, /, compress: bool | None = ..., level: int | None = ...) -> bytes:
        """
        Alias for `encode()`.

        See `encode()` for full documentation.
        """
        ...


    @staticmethod
    def deserialize(data: bytes) -> Expression:
        """
        Alias for `decode()`.

        See `decode()` for full documentation.
        """
        ...

    @staticmethod
    def decode(data: bytes) -> Expression:
        """
        Reconstruct an expression from encoded bytes.

        Parameters
        ----------
        data : bytes
            Binary blob returned by `encode()`.

        Returns
        -------
        Expression
            Deserialized expression object.

        Raises
        ------
        DecodeError
            If decoding fails due to corruption or incompatibility.
        """
        ...

    @overload
    def __add__(self, other: Expression, /) -> Expression: ...
    @overload
    def __add__(self, other: Variable, /) -> Expression: ...
    @overload
    def __add__(self, other: int, /) -> Expression: ...
    @overload
    def __add__(self, other: float, /) -> Expression: ...
    def __add__(self, other: Expression | Variable | int | float, /) -> Expression:
        """
        Add another expression, variable, or scalar.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Expression

        Raises
        ------
        VariablesFromDifferentEnvsError
            If operands are from different environments.
        TypeError
            If the operand type is unsupported.
        """
        ...

    @overload
    def __radd__(self, other: Expression, /) -> Expression: ...
    @overload
    def __radd__(self, other: Variable, /) -> Expression: ...
    @overload
    def __radd__(self, other: int, /) -> Expression: ...
    @overload
    def __radd__(self, other: float, /) -> Expression: ...
    def __radd__(self, other: Expression | Variable | int | float, /) -> Expression:
        """
        Add this expression to a scalar or variable.

        Parameters
        ----------
        other : int, float, or Variable

        Returns
        -------
        Expression

        Raises
        ------
        TypeError
            If the operand type is unsupported.
        """
        ...

    @overload
    def __iadd__(self, other: Expression, /): ...
    @overload
    def __iadd__(self, other: Variable, /): ...
    @overload
    def __iadd__(self, other: int, /): ...
    @overload
    def __iadd__(self, other: float, /): ...
    def __iadd__(self, other: Expression | Variable | int | float, /) -> Expression:
        """
        In-place addition.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Expression

        Raises
        ------
        VariablesFromDifferentEnvsError
            If operands are from different environments.
        TypeError
            If the operand type is unsupported.
        """
        ...

    @overload
    def __isub__(self, other: Expression, /): ...
    @overload
    def __isub__(self, other: Variable, /): ...
    @overload
    def __isub__(self, other: int, /): ...
    @overload
    def __isub__(self, other: float, /): ...
    def __isub__(self, other: Expression | Variable | int | float, /):
        """
        In-place subtraction.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Expression

        Raises
        ------
        VariablesFromDifferentEnvsError
            If operands are from different environments.
        TypeError
            If the operand type is unsupported.
        """
        ...

    @overload
    def __sub__(self, other: Expression, /) -> Expression: ...
    @overload
    def __sub__(self, other: Variable, /) -> Expression: ...
    @overload
    def __sub__(self, other: int, /) -> Expression: ...
    @overload
    def __sub__(self, other: float, /) -> Expression: ...
    def __sub__(self, other: Expression | Variable | int | float, /) -> Expression:
        """
        Subtract another expression, variable, or scalar.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Expression

        Raises
        ------
        VariablesFromDifferentEnvsError
            If operands are from different environments.
        TypeError
            If the operand type is unsupported.
        """
        ...

    @overload
    def __mul__(self, other: Expression, /) -> Expression: ...
    @overload
    def __mul__(self, other: Variable, /) -> Expression: ...
    @overload
    def __mul__(self, other: int, /) -> Expression: ...
    @overload
    def __mul__(self, other: float, /) -> Expression: ...
    def __mul__(self, other: Expression | Variable | int | float, /) -> Expression:
        """
        Multiply this expression by another value.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Expression

        Raises
        ------
        VariablesFromDifferentEnvsError
            If operands are from different environments.
        TypeError
            If the operand type is unsupported.
        """
        ...


    @overload
    def __rmul__(self, other: int, /) -> Expression: ...
    @overload
    def __rmul__(self, other: float, /) -> Expression: ...
    def __rmul__(self, other: int | float, /) -> Expression:
        """
        Right-hand multiplication.

        Parameters
        ----------
        other : int or float

        Returns
        -------
        Expression

        Raises
        ------
        TypeError
            If the operand type is unsupported.
        """
        ...

    @overload
    def __imul__(self, other: Expression, /): ...
    @overload
    def __imul__(self, other: Variable, /): ...
    @overload
    def __imul__(self, other: int, /): ...
    @overload
    def __imul__(self, other: float, /): ...
    def __imul__(self, other: Expression | Variable | int | float, /):
        """
        In-place multiplication.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Expression

        Raises
        ------
        VariablesFromDifferentEnvsError
            If operands are from different environments.
        TypeError
            If the operand type is unsupported.
        """
        ...

    def __pow__(self, other: int, /) -> Expression:
        """
        Raise the expression to the power specified by `other`.

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
    def __eq__(self, rhs: Expression, /) -> Constraint: ...
    @overload
    def __eq__(self, rhs: Variable, /) -> Constraint: ...
    @overload
    def __eq__(self, rhs: int, /) -> Constraint: ...  # type: ignore
    @overload
    def __eq__(self, rhs: float, /) -> Constraint: ...  # type: ignore
    def __eq__(self, rhs: Expression | Variable | int | float, /) -> Constraint: # type: ignore
        """
        Compare to a different expression or create a constraint `expression == scalar`

        If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
        constraint, resulting in the following constraint:

            self - rhs == 0

        Parameters
        ----------
        rhs : Expression or float, int, Variable or Expression

        Returns
        -------
        bool or Constraint

        Raises
        ------
        TypeError
            If the right-hand side is not an Expression or scalar.
        """
        ...

    @overload
    def __le__(self, rhs: Expression, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: Variable, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: int, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: float, /) -> Constraint: ...
    def __le__(self, rhs: Expression | Variable | int | float, /) -> Constraint:
        """
        Create a constraint `expression <= scalar`.

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
    def __ge__(self, rhs: Expression, /) -> Constraint: ...
    @overload
    def __ge__(self, rhs: Variable, /) -> Constraint: ...
    @overload
    def __ge__(self, rhs: int, /) -> Constraint: ...
    @overload
    def __ge__(self, rhs: float, /) -> Constraint: ...
    def __ge__(self, rhs: Expression | Variable | int | float, /) -> Constraint:
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
        Negate the expression, i.e., multiply it by `-1`.

        Returns
        -------
        Expression
        """
        ...

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
