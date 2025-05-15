from __future__ import annotations
from aqmodels._api_utils import export, dispatched


@export
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

    >>> from aqmodels import Environment, Variable
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

    @dispatched
    def __init__(self, env):
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
        return env

    @dispatched
    def get_offset(self):
        """
        Get the constant (offset) term in the expression.

        Returns
        -------
        float
            The constant term.
        """
        return

    @dispatched
    def get_linear(self, variable):
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
        return variable

    @dispatched
    def get_quadratic(self, u, v):
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
        return u, v

    @dispatched
    def get_higher_order(self, variables):
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
        return variables

    @dispatched
    def num_variables(self):
        """
        Return the number of distinct variables in the expression.

        Returns
        -------
        int
            Number of variables with non-zero coefficients.
        """
        return

    @dispatched
    def is_equal(self, other):
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
        return other

    @dispatched
    def encode(self, compress=True, level=3):
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
        return compress, level

    @dispatched
    def serialize(self, compress=True, level=3):
        """
        Alias for `encode()`.

        See `encode()` for full documentation.
        """
        return compress, level

    @dispatched
    @staticmethod
    def decode(data):
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
        return data

    @dispatched
    @staticmethod
    def deserialize(data):
        """
        Alias for `decode()`.

        See `decode()` for full documentation.
        """
        return data

    @dispatched
    def __add__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __radd__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __sub__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __rsub__(self, other):
        """
        Subtract this expression from a scalar or variable.

        Parameters
        ----------
        other : int, float, or Variable

        Returns
        -------
        Expression

        Raises
        ------
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __mul__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __rmul__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __iadd__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __imul__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __eq__(self, rhs):
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
        RuntimeError
            If the right-hand side is not of type float, int, Variable or Expression.
        """
        return rhs

    @dispatched
    def __le__(self, rhs):
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
        RuntimeError
            If the right-hand side is not of type float, int, Variable or Expression.
        """
        return rhs

    @dispatched
    def __ge__(self, rhs):
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
        RuntimeError
            If the right-hand side is not of type float, int, Variable or Expression.
        """
        return rhs
