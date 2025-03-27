from enum import Enum
from aqmodels._api_utils import dispatched, export


@export
class Comparator(Enum):
    """
    Comparison operators used to define constraints.

    This enum represents the logical relation between the left-hand side (LHS)
    and the right-hand side (RHS) of a constraint.

    Attributes
    ----------
    Eq : Comparator
        Equality constraint (==).
    Leq : Comparator
        Less-than-or-equal constraint (<=).
    Geq : Comparator
        Greater-than-or-equal constraint (>=).

    Examples
    --------
    >>> from aqmodels import Comparator
    >>> str(Comparator.Eq)
    '=='
    """

    Eq = ...
    """Equality (==)"""

    Leq = ...
    """Less-than or equal (<=)"""

    Geq = ...
    """Greater-than or equal (>=)"""


@export
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
    >>> from aqmodels import Environment, Variable, Constraint, Comparator
    >>> with Environment():
    ...     x = Variable("x")
    ...     c = Constraint(x + 2, 5.0, Comparator.Eq)

    Or create via comparison:

    >>> expr = 2 * x + 1
    >>> c2 = expr <= 10.0
    """

    @dispatched
    def __init__(self, lhs, rhs, comparator):
        """
        Construct a new symbolic constraint.

        Parameters
        ----------
        lhs : Expression
            Left-hand side symbolic expression.
        rhs : float
            Scalar right-hand side constant.
        comparator : Comparator
            Relational operator (e.g., Comparator.Eq, Comparator.Leq).

        Raises
        ------
        RuntimeError
            If lhs is not an Expression or rhs is not a scalar float.
        """
        return lhs, rhs, comparator


@export
class Constraints:
    """
    A collection of symbolic constraints used to define a model.

    The `Constraints` object serves as a container for individual `Constraint`
    instances. It supports adding constraints programmatically and exporting
    them for serialization.

    Constraints are typically added using `add_constraint()` or the `+=` operator.

    Examples
    --------
    >>> from aqmodels import Constraints, Constraint, Environment, Variable
    >>> with Environment():
    ...     x = Variable("x")
    ...     c = Constraint(x + 1, 0.0, Comparator.Leq)

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

    @dispatched
    def add_constraint(self, constraint):
        """
        Add a constraint to the collection.

        Parameters
        ----------
        constraint : Constraint
            The constraint to be added.
        """
        return constraint

    @dispatched
    def encode(self, compress=True, level=3):
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
        return compress, level

    @dispatched
    def serialize(self, compress=True, level=3):
        """
        Alias for `encode()`.

        See `encode()` for details.
        """
        return compress, level

    @dispatched
    @staticmethod
    def decode(data):
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
        return data

    @dispatched
    @staticmethod
    def deserialize(data):
        """
        Alias for `decode()`.

        See `decode()` for usage.
        """
        return data

    @dispatched
    def __iadd__(self, constraint):
        """
        In-place constraint addition using `+=`.

        Parameters
        ----------
        constraint : Constraint
            The constraint to add.

        Returns
        -------
        Constraints
            The updated collection.

        Raises
        ------
        RuntimeError
            If the value is not a `Constraint` or valid symbolic comparison.
        """
        return constraint
