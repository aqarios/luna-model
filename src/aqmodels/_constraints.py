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
    Le : Comparator
        Less-than-or-equal constraint (<=).
    Ge : Comparator
        Greater-than-or-equal constraint (>=).

    Examples
    --------
    >>> from aqmodels import Comparator
    >>> str(Comparator.Eq)
    '=='
    """

    Eq = ...
    """Equality (==)"""

    Le = ...
    """Less-than or equal (<=)"""

    Ge = ...
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
    def __init__(self, lhs, rhs, comparator, name):
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
        return lhs, rhs, comparator, name

    @dispatched
    @property
    def name(self):
        """
        Get the name of the constraint.

        Returns
        -------
        str, optional
            Returns the name of the constraint as a string or None if it is unnamed.
        """
        return

    @dispatched
    @property
    def lhs(self):
        """
        Get the left-hand side of the constraint

        Returns
        -------
        Expression
            The left-hand side expression.
        """
        return

    @dispatched
    @property
    def rhs(self):
        """
        Get the right-hand side of the constraint

        Returns
        -------
        float
            The right-hand side expression.
        """
        return

    @property
    def comparator(self):
        """
        Get the right-hand side of the constraint

        Returns
        -------
        Comparator§
            The comparator of the constraint.
        """
        return


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

    @dispatched
    def add_constraint(self, constraint, name):
        """
        Add a constraint to the collection.

        Parameters
        ----------
        constraint : Constraint
            The constraint to be added.
        name : str, optional
            The name of the constraint to be added.
        """
        return constraint, name

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
        return constraint

    @dispatched
    def __getitem__(self, item):
        return item

    @dispatched
    def __eq__(self, other):
        return other
