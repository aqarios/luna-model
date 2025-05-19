from enum import Enum
from typing import overload


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

    def __eq__(self, /, other: Constraint) -> bool: ... # type: ignore
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
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
    def encode(self, /, compress: bool | None = ...) -> bytes: ...
    @overload
    def encode(self, /, level: int | None = ...) -> bytes: ...
    @overload
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
    def serialize(self, /, compress: bool | None = ...) -> bytes: ...
    @overload
    def serialize(self, /, level: int | None = ...) -> bytes: ...
    @overload
    def serialize(
        self, /, compress: bool | None = ..., level: int | None = ...
    ) -> bytes:
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
    def __iadd__(self, /, constraint: Constraint): ...
    @overload
    def __iadd__(self, /, constraint: tuple[Constraint, str]): ...
    def __iadd__(self, /, constraint: Constraint | tuple[Constraint, str]):
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

    def __eq__(self, /, other: Constraints) -> bool: ... # type: ignore
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
    def __getitem__(self, /, item: int) -> Constraint: ...

# _errors.pyi
class VariableOutOfRangeError(Exception):
    """
    Raised when a variable referenced in an expression is out of bounds for the environment.

    This error typically occurs when querying coefficients (linear, quadratic,
    or higher-order) from an `Expression` using a `Variable` whose index does not
    exist in the environment's internal registry.

    This may happen if:
        - A variable is used from a different environment
        - A variable was removed or never registered properly
        - A raw index or tuple refers to a non-existent variable ID
    """
    def __str__(self) -> str: ...

class VariableExistsError(Exception):
    """
    Raised when trying to create a variable with a name that already exists.

    Variable names must be unique within an `Environment`. Attempting to redefine
    a variable with the same name will raise this exception.
    """
    def __str__(self) -> str: ...

class VariableNotExistingError(Exception):
    """
    Raised when trying to get a variable with a name that does not exist.
    """
    def __str__(self) -> str: ...

class VariableCreationError(Exception):
    """
    Raised when an error occurs during the creation of a variable.

    For example, binary and spin variables cannot be created with bounds.
    """
    def __str__(self) -> str: ...

class VariablesFromDifferentEnvsError(Exception):
    """
    Raised when multiple variables from different environments are used together.

    All variables in an expression or constraint must belong to the same
    `Environment`. Mixing across environments is disallowed to ensure consistency.
    """
    def __str__(self) -> str: ...

class DifferentEnvsError(Exception):
    """
    Raised when two incompatible environments are passed to a model or operation.

    Unlike `VariablesFromDifferentEnvsError`, this error may occur at the model level
    or in structural operations that require consistency across multiple environments.
    """
    def __str__(self) -> str: ...

class NoActiveEnvironmentFoundError(Exception):
    """
    Raised when a variable or expression is created without an active environment context.

    This typically happens when not using `with Environment(): ...` and no environment
    was explicitly provided.
    """
    def __str__(self) -> str: ...

class MultipleActiveEnvironmentsError(Exception):
    """
    Raised when multiple environments are active simultaneously.

    This is a logic error, since `aqmodels` only supports one active environment
    at a time. This is enforced to maintain clarity and safety.
    """
    def __str__(self) -> str: ...

class DecodeError(Exception):
    """
    Raised when decoding or deserialization of binary data fails.

    This can occur if the encoded data is corrupted, incompatible, or not generated
    by `aqmodels.encode()`.
    """
    def __str__(self) -> str: ...

class VariableNamesError(Exception):
    """
    Raised when the QuboTranslator tries to create a model from a QUBO matrix, but
    the provided variable names are invalid.

    If variable names are provided to the QuboTranslator, they have to be unique, and
    the number of names has to match the number of variables in the QUBO matrix.
    """
    def __str__(self) -> str: ...

class IllegalConstraintNameError(Exception):
    """
    Raised when a constraint is tried to be created with an illegal name.
    """
    def __str__(self) -> str: ...

class TranslationError(Exception):
    """
    Raised when an error occurred during translation.
    """
    def __str__(self) -> str: ...

class ModelNotQuadraticError(TranslationError):
    """
    Raised when a model is expected to be quadratic but contains higher-order terms.

    Some solvers or transformations require the model to have at most quadratic
    expressions. This error signals that unsupported terms were detected.
    """
    def __str__(self) -> str: ...

class ModelNotUnconstrainedError(TranslationError):
    """
    Raised when an operation requires an unconstrained model, but constraints are present.

    Some solution methods may only work on unconstrained models, such as when
    transforming a symbolic model to a low-level format.
    """
    def __str__(self) -> str: ...

class ModelSenseNotMinimizeError(TranslationError):
    """
    Raised when an operation requires a model with minimization sense, but has
    maximization sense.

    Some model formats only work with minimization sense. In this case, consider
    setting the sense to `minimize` before the transformation, and multiplying the
    objective by `-1` if necessary.
    """
    def __str__(self) -> str: ...

class ModelVtypeError(TranslationError):
    """
    Raised when an operation has certain constraints on a model's variable types that
    are violated.

    Some solution methods may only work on models where all variables have the same
    type, or where only certain variable types are permitted.
    """
    def __str__(self) -> str: ...

class SolutionTranslationError(Exception):
    """
    Raised when something goes wrong during the translation of a solution.

    This may happen during the translation to an AqSolution from a different solution
    format, e.g., when the samples have different lengths or the variable types are not
    consistent with the model the solution is created for.
    """
    def __str__(self) -> str: ...

class SampleIncorrectLengthError(SolutionTranslationError):
    """
    Raised when a sample length is different from the number of model variables.

    When an external solution format is translated to an AqSolution, the number of
    variable assignments in the solution's sample has to exactly match the number of
    variables in the model environment that is passed to the translator.
    """
    def __str__(self) -> str: ...

class SampleUnexpectedVariableError(SolutionTranslationError):
    """
    Raised when a sample contains a variable with a name that is not present in the
    environment.

    When a sample is translated to an AqResult, the currently active environment has to
    contain the same variables as the sample.
    """
    def __str__(self) -> str: ...

class SampleIncompatibleVtypeError(SolutionTranslationError):
    """
    Raised when a sample's assignments have variable types incompatible with the
    model's variable types.

    When an external solution format is translated to an AqSolution, the variable
    assignments are tried to be converted into the model's corresponding variable type.
    This may fail when the assignment types are incompatible.

    Note that conversions with precision loss or truncation are admitted, but
    conversions of variables outside the permitted range will fail.
    """
    def __str__(self) -> str: ...
