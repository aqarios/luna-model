from enum import Enum
from typing import overload

from aqmodels._expression import Expression
from aqmodels._environment import Environment
from aqmodels._variable import Variable

class Comparator(Enum):
    """
    Comparison operators used to define constraints.

    This enum represents the logical relation between the left-hand side (LHS)
    and the right-hand side (RHS) of a constraint.

    Attributes
    ----------
    Eq : Comparator
        Equality constraint (==).
    Le : Comparator
        Less-than-or-equal constraint (<=).
    Ge : Comparator
        Greater-than-or-equal constraint (>=).

    Examples
    --------
    >>> from luna_quantum import Comparator
    >>> str(Comparator.Eq)
    '=='
    """

    Eq = ...
    """Equality (==)"""

    Le = ...
    """Less-than or equal (<=)"""

    Ge = ...
    """Greater-than or equal (>=)"""

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...

class Constraint:
    """
    A symbolic constraint formed by comparing an expression to a constant.

    A `Constraint` captures a relation of the form:
    `expression comparator constant`, where the comparator is one of:
    `==`, `<=`, or `>=`.

    While constraints are usually created by comparing an `Expression` to a scalar
    (e.g., `expr == 3.0`), they can also be constructed manually using this class.

    Parameters
    ----------
    lhs : Expression
        The left-hand side expression.
    rhs : float
        The scalar right-hand side value.
    comparator : Comparator
        The relation between lhs and rhs (e.g., `Comparator.Eq`).

    Examples
    --------
    >>> from luna_quantum import Environment, Variable, Constraint, Comparator
    >>> with Environment():
    ...     x = Variable("x")
    ...     c = Constraint(x + 2, 5.0, Comparator.Eq)

    Or create via comparison:

    >>> expr = 2 * x + 1
    >>> c2 = expr <= 10.0
    """

    @overload
    def __init__(
        self, /, lhs: Expression, rhs: Expression, comparator: Comparator
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Expression, rhs: Variable, comparator: Comparator
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Expression, rhs: int, comparator: Comparator
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Expression, rhs: float, comparator: Comparator
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Expression, rhs: Expression, comparator: Comparator, name: str
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Expression, rhs: Variable, comparator: Comparator, name: str
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Expression, rhs: int, comparator: Comparator, name: str
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Expression, rhs: float, comparator: Comparator, name: str
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Variable, rhs: Expression, comparator: Comparator
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Variable, rhs: Variable, comparator: Comparator
    ) -> None: ...
    @overload
    def __init__(self, /, lhs: Variable, rhs: int, comparator: Comparator) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Variable, rhs: float, comparator: Comparator
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Variable, rhs: Expression, comparator: Comparator, name: str
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Variable, rhs: Variable, comparator: Comparator, name: str
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Variable, rhs: int, comparator: Comparator, name: str
    ) -> None: ...
    @overload
    def __init__(
        self, /, lhs: Variable, rhs: float, comparator: Comparator, name: str
    ) -> None: ...
    def __init__(
        self,
        /, 
        lhs: Variable | Expression,
        rhs: int | float | Expression | Variable,
        comparator: Comparator,
        name: str,
    ) -> None:
        """
        Construct a new symbolic constraint.

        Parameters
        ----------
        lhs : Expression | Variable
            Left-hand side symbolic expression or variable.
        rhs : int | float | Expression | Variable
            Scalar right-hand side constant.
        comparator : Comparator
            Relational operator (e.g., Comparator.Eq, Comparator.Le).
        name : str
            The name of the constraint

        Raises
        ------
        TypeError
            If lhs is not an Expression or rhs is not a scalar float.
        IllegalConstraintNameError
            If the constraint is tried to be created with an illegal name.
        """
        ...

    @property
    def name(self, /) -> str | None:
        """
        Get the name of the constraint.

        Returns
        -------
        str, optional
            Returns the name of the constraint as a string or None if it is unnamed.
        """
        ...
    @property
    def lhs(self, /) -> Expression:
        """
        Get the left-hand side of the constraint

        Returns
        -------
        Expression
            The left-hand side expression.
        """
        ...

    @property
    def rhs(self, /) -> float:
        """
        Get the right-hand side of the constraint

        Returns
        -------
        float
            The right-hand side expression.
        """
        ...

    @property
    def comparator(self, /) -> Comparator:
        """
        Get the comparator of the constraint

        Returns
        -------
        Comparator
            The comparator of the constraint.
        """
        ...

    def __eq__(self, other: Constraint, /) -> bool: ... # type: ignore
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...

class Constraints:
    """
    A collection of symbolic constraints used to define a model.

    The `Constraints` object serves as a container for individual `Constraint`
    instances. It supports adding constraints programmatically and exporting
    them for serialization.

    Constraints are typically added using `add_constraint()` or the `+=` operator.

    Examples
    --------
    >>> from luna_quantum import Constraints, Constraint, Environment, Variable
    >>> with Environment():
    ...     x = Variable("x")
    ...     c = Constraint(x + 1, 0.0, Comparator.Le)

    >>> cs = Constraints()
    >>> cs.add_constraint(c)

    >>> cs += x >= 1.0

    Serialization:

    >>> blob = cs.encode()
    >>> expr = Constraints.decode(blob)

    Notes
    -----
    - This class does not check feasibility or enforce satisfaction.
    - Use `encode()`/`decode()` to serialize constraints alongside expressions.
    """

    def __init__(self, /) -> None: ...
    @overload
    def add_constraint(self, /, constraint: Constraint):
        """
        Add a constraint to the collection.

        Parameters
        ----------
        constraint : Constraint
            The constraint to be added.
        name : str, optional
            The name of the constraint to be added.
        """
        ...

    @overload
    def add_constraint(self, /, constraint: Constraint, name: str): ...
    def add_constraint(self, /, constraint: Constraint, name: str | None = ...):
        """
        Add a constraint to the collection.

        Parameters
        ----------
        constraint : Constraint
            The constraint to be added.
        name : str, optional
            The name of the constraint to be added.
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
        Serialize the constraint collection to a binary blob.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the result. Default is True.
        level : int, optional
            Compression level (0–9). Default is 3.

        Returns
        -------
        bytes
            Encoded representation of the constraints.

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

        See `encode()` for details.
        """
        ...

    @staticmethod
    def decode(data: bytes, env: Environment) -> Expression:
        """
        Deserialize an expression from binary constraint data.

        Parameters
        ----------
        data : bytes
            Encoded blob from `encode()`.

        Returns
        -------
        Expression
            Expression reconstructed from the constraint context.

        Raises
        ------
        DecodeError
            If decoding fails due to corruption or incompatibility.
        """
        ...

    @staticmethod
    def deserialize(data: bytes, env: Environment) -> Expression:
        """
        Alias for `decode()`.

        See `decode()` for usage.
        """
        ...

    @overload
    def __iadd__(self, constraint: Constraint, /): ...
    @overload
    def __iadd__(self, constraint: tuple[Constraint, str], /): ...
    def __iadd__(self, constraint: Constraint | tuple[Constraint, str], /):
        """
        In-place constraint addition using `+=`.

        Parameters
        ----------
        constraint : Constraint | tuple[Constraint, str]
            The constraint to add.

        Returns
        -------
        Constraints
            The updated collection.

        Raises
        ------
        TypeError
            If the value is not a `Constraint` or valid symbolic comparison.
        """
        ...

    def __eq__(self, other: Constraints, /) -> bool: ... # type: ignore
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
    def __getitem__(self, item: int, /) -> Constraint: ...
