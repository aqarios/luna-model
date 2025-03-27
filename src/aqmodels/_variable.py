from enum import Enum
from aqmodels._api_utils import dispatched, export


@export
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
    >>> from aqmodels import Vtype
    >>> Vtype.Real
    Real

    >>> str(Vtype.Binary)
    'Binary'
    """

    Real = ...
    """Continuous real-valued variable."""

    Integer = ...
    """Discrete integer-valued variable."""

    Binary = ...
    """Binary variable (0 or 1)."""

    Spin = ...
    """Spin variable (-1 or +1)."""


@export
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
    >>> from aqmodels import Bounds
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

    @dispatched
    def __init__(self, lower, upper):
        """
        Create bounds for a variable.

        See class-level docstring for full documentation.
        """
        return lower, upper


@export
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
    env : Environment, optional
        The environment in which this variable is created. If not provided,
        the current environment from the context manager is used.
    vtype : Vtype, optional
        The variable type (e.g., `Vtype.Real`, `Vtype.Integer`, etc.).
        Defaults to `Vtype.Binary`.
    bounds : Bounds, optional
        Bounds restricting the range of the variable. Only applicable for
        `Real` and `Integer` variables.

    Examples
    --------
    >>> from aqmodels import Variable, Environment, Vtype, Bounds
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

    @dispatched
    def __init__(self, name, env, vtype, bounds):
        """
        Initialize a new Variable.

        See class-level docstring for full usage.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no active environment is found and none is explicitly provided.
        VariableExistsError
            If a variable with the same name already exists in the environment.
        """
        return name, env, vtype, bounds

    @dispatched
    def __add__(self, other):
        """
        Add this variable to another value.

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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __radd__(self, other):
        """
        Right-hand addition for scalars.

        Parameters
        ----------
        other : int or float

        Returns
        -------
        Expression
            The resulting symbolic expression.

        Raises
        ------
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __sub__(self, other):
        """
        Subtract a value from this variable.

        Parameters
        ----------
        other : Variable, int, or float

        Returns
        -------
        Expression
            The resulting symbolic expression.

        Raises
        ------
        VariablesFromDifferentEnvsError
            If the operands belong to different environments.
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __rsub__(self, other):
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
        RuntimeError
            Always raised (unsupported operation).
        """
        return other

    @dispatched
    def __mul__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other

    @dispatched
    def __rmul__(self, other):
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
        RuntimeError
            If the operand type is unsupported.
        """
        return other
