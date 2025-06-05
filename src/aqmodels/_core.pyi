from datetime import datetime, timedelta
from enum import Enum
from types import TracebackType
from typing import Literal, Self, overload

from numpy.typing import NDArray

from . import errors, translator

# _variable.pyi
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

class Unbounded: ...

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
    def __init__(self, /, *, lower: float | Unbounded) -> None: ...
    @overload
    def __init__(self, /, *, upper: float | type[Unbounded]) -> None: ...
    @overload
    def __init__(
        self, /, lower: float | type[Unbounded], upper: float | type[Unbounded]
    ) -> None: ...
    def __init__(
        self,
        /,
        lower: float | type[Unbounded] | None = ...,
        upper: float | type[Unbounded] | None = ...,
    ) -> None:
        """
        Create bounds for a variable.

        See class-level docstring for full documentation.
        """
        ...

    @property
    def lower(self, /) -> float | Unbounded | None:
        """Get the lower bound."""
        ...

    @property
    def upper(self, /) -> float | Unbounded | None:
        """Get the upper bound."""
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
    def __init__(self, /, name: str, *, env: Environment) -> None: ...
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

    @property
    def bounds(self, /) -> Bounds:
        """Get the bounds of the variable."""
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
    def __eq__(self, rhs: Expression, /) -> Constraint: ...
    @overload
    def __eq__(self, rhs: Variable, /) -> bool:  # noqa: D418
        """
        Check equality of two variables.

        Parameters
        ----------
        rhs : Variable

        Returns
        -------
        bool
        """
    def __eq__(self, rhs: int | float | Expression, /) -> Constraint:  # type: ignore[reportIncompatibleMethodOverride]
        """
        Create a constraint: Variable == float | int | Expression.

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

    @overload
    def __le__(self, rhs: int, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: float, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: Variable, /) -> Constraint: ...
    @overload
    def __le__(self, rhs: Expression, /) -> Constraint: ...
    def __le__(self, rhs: int | float | Variable | Expression, /) -> Constraint:
        """
        Create a constraint: Variable <= scalar.

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
        Create a constraint: Variable >= scalar.

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

    @property
    def _environment(self, /) -> Environment:
        """Get this variables's environment."""
        ...

    def __hash__(self, /) -> int: ...
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...

# _timing.pyi
class Timing:
    """
    The object that holds information about an algorithm's runtime.

    This class can only be constructed using a `Timer`. This ensures that a
    `Timing` object always contains a start as well as an end time.

    The `qpu` field of this class can only be set after constructing it with a timer.

    Examples
    --------
    >>> from dwave.samplers.tree.solve import BinaryQuadraticModel
    >>> from luna_quantum import Model, Timer, Timing
    >>> model = ...  # third-party model
    >>> algorithm = ...  # third-party algorithm
    >>> timer = Timer.start()
    >>> sol = algorithm.run(model)
    >>> timing: Timing = timer.stop()
    >>> timing.qpu = sol.qpu_time
    >>> timing.total_seconds
    1.2999193
    >>> timing.qpu
    0.02491934
    """

    @property
    def start(self, /) -> datetime:
        """The starting time of the algorithm."""
        ...

    @property
    def end(self, /) -> datetime:
        """The end, or finishing, time of the algorithm."""
        ...

    @property
    def total(self, /) -> timedelta:
        """
        The difference of the end and start time.

        Raises
        ------
        RuntimeError
            If total cannot be computed due to an inconsistent start or end time.
        """
        ...

    @property
    def total_seconds(self, /) -> float:
        """
        The total time in seconds an algorithm needed to run.

        Computed as the difference of end and start time.

        Raises
        ------
        RuntimeError
            If `total_seconds` cannot be computed due to an inconsistent start or
            end time.
        """
        ...

    @property
    def qpu(self, /) -> float | None:
        """The qpu usage time of the algorithm this timing object was created for."""
        ...

    @qpu.setter
    def qpu(self, /, value: float | None) -> None:
        """
        Set the qpu usage time.

        Raises
        ------
        ValueError
            If `value` is negative.
        """
        ...

    def add_qpu(self, /, value: float) -> None:
        """
        Add qpu usage time to the qpu usage time already present.

        If the current value is None, this method acts like a setter.

        Parameters
        ----------
        value : float
            The value to add to the already present qpu value.

        Raises
        ------
        ValueError
            If `value` is negative.
        """
        ...

class Timer:
    """
    Used to measure the computation time of an algorithm.

    The sole purpose of the `Timer` class is to create a `Timing` object in a safe
    way, i.e., to ensure that the `Timing` object always holds a starting and
    finishing time.

    Examples
    --------
    Basic usage:
    >>> from luna_quantum import Timer
    >>> timer = Timer.start()
    >>> solution = ...  # create a solution by running an algorithm.
    >>> timing = timer.stop()
    """

    @staticmethod
    def start() -> Timer:
        """
        Create a timer that starts counting immediately.

        Returns
        -------
        Timer
            The timer.
        """
        ...

    def stop(self, /) -> Timing:
        """
        Stop the timer, and get the resulting `Timing` object.

        Returns
        -------
        Timing
            The timing object that holds the start and end time.
        """
        ...

# _solution.pyi
class Solution:
    """
    The solution object that is obtained by running an algorihtm.

    The `Solution` class represents a summary of all data obtained from solving a
    model. It contains samples, i.e., assignments of values to each model variable as
    returned by the algorithm, metadata about the solution quality, e.g., the objective
    value, and the runtime of the algorithm.

    A `Solution` can be constructed explicitly using `from_dict` or by obtaining a
    solution from an algorithm or by converting a different solution format with one of
    the available translators. Note that the latter requires the environment the model
    was created in.

    Examples
    --------
    Basic usage, assuming that the algorithm already returns a `Solution`:

    >>> from luna_quantum import Model, Solution
    >>> model: Model = ...
    >>> algorithm = ...
    >>> solution: Solution = algorithm.run(model)
    >>> solution.samples
    [[1, 0, 1], [0, 0, 1]]

    When you have a `dimod.Sampleset` as the raw solution format:

    >>> from luna_quantum.translator import BqmTranslator
    >>> from luna_quantum import Model, Solution, DwaveTranslator
    >>> from dimod import SimulatedAnnealingSampler
    >>> model: Model = ...
    >>> bqm = BqmTranslator.from_aq(model)
    >>> sampleset = SimulatedAnnealingSampler().sample(bqm)
    >>> solution = DwaveTranslator.from_dimod_sample_set(sampleset)
    >>> solution.samples
    [[1, 0, 1], [0, 0, 1]]

    Serialization:

    >>> blob = solution.encode()
    >>> restored = Solution.decode(blob)
    >>> restored.samples
    [[1, 0, 1], [0, 0, 1]]

    Notes
    -----
    - To ensure metadata like objective values or feasibility, use
      `model.evaluate(solution)`.
    - Use `encode()` and `decode()` to serialize and recover solutions.
    """

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
    def __len__(self, /) -> int: ...
    def __iter__(self, /) -> ResultIterator:
        """
        Extract a result view from the `Solution` object.

        Returns
        -------
        ResultView

        Raises
        ------
        TypeError
            If `item` has the wrong type.
        IndexError
            If the row index is out of bounds for the variable environment.
        """
        ...

    def __getitem__(self, item: int, /) -> ResultView:
        """
        Extract a result view from the `Solution` object.

        Returns
        -------
        ResultView

        Raises
        ------
        TypeError
            If `item` has the wrong type.
        IndexError
            If the row index is out of bounds for the variable environment.
        """
        ...

    def __eq__(self, other: Solution, /) -> bool:  # type: ignore[reportIncompatibleMethodOverride]
        """
        Check whether this solution is equal to `other`.

        Parameters
        ----------
        other : Model

        Returns
        -------
        bool
        """
        ...

    def best(self, /) -> ResultView | None:
        """
        Get the best result of the solution if it exists.

        A best solution is defined as the result with the lowest (in case of Sense.Min)
        or the highest (in case of Sense.Max) objective value that is feasible.

        Returns
        -------
        ResultView
            The best result of the solution as a view.
        """
        ...

    @property
    def results(self, /) -> ResultIterator:
        """Get an iterator over the single results of the solution."""
        ...

    @property
    def samples(self, /) -> Samples:
        """Get a view into the samples of the solution."""
        ...

    @property
    def obj_values(self, /) -> NDArray:
        """
        Get the objective values of the single samples as a ndarray.

        A value will be None if the sample hasn't yet been evaluated.
        """
        ...

    @property
    def raw_energies(self, /) -> NDArray:
        """Get the raw energies.

        Get the raw energy values of the single samples as returned by the solver /
        algorithm. Will be None if the solver / algorithm did not provide a value.
        """
        ...

    @property
    def counts(self, /) -> NDArray:
        """Return how often each sample occurred in the solution."""
        ...

    @property
    def runtime(self, /) -> Timing | None:
        """Get the solver / algorithm runtime."""
        ...

    @property
    def best_sample_idx(self, /) -> int | None:
        """Get the index of the sample with the best objective value."""
        ...

    @property
    def variable_names(self, /) -> list[str]:
        """Get the names of all variables in the solution."""
        ...

    def expectation_value(self, /) -> float:
        """
        Compute the expectation value of the solution.

        Returns
        -------
        float
            The expectation value.

        Raises
        ------
        ComputationError
            If the computation fails for any reason.
        """

    @overload
    def encode(self, /) -> bytes: ...
    @overload
    def encode(self, /, *, compress: bool) -> bytes: ...
    @overload
    def encode(self, /, *, level: int) -> bytes: ...
    @overload
    def encode(self, /, *, compress: bool, level: int) -> bytes: ...
    def encode(self, /, *, compress: bool = True, level: int = 3) -> bytes:
        """
        Serialize the solution into a compact binary format.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the binary output. Default is True.
        level : int, optional
            Compression level (0-9). Default is 3.

        Returns
        -------
        bytes
            Encoded model representation.

        Raises
        ------
        IOError
            If serialization fails.
        """
        ...

    @overload
    def serialize(self, /) -> bytes: ...
    @overload
    def serialize(self, /, *, compress: bool) -> bytes: ...
    @overload
    def serialize(self, /, *, level: int) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool, level: int) -> bytes: ...
    def serialize(
        self, /, compress: bool | None = ..., level: int | None = ...
    ) -> bytes:
        """
        Alias for `encode()`.

        See `encode()` for details.
        """
        ...

    @classmethod
    def decode(cls, data: bytes) -> Solution:
        """
        Reconstruct a solution object from binary data.

        Parameters
        ----------
        data : bytes
            Serialized model blob created by `encode()`.

        Returns
        -------
        Solution
            The reconstructed solution.

        Raises
        ------
        DecodeError
            If decoding fails due to corruption or incompatibility.
        """
        ...

    @classmethod
    def deserialize(cls, data: bytes) -> Solution:
        """Alias for `decode()`."""
        ...

    @staticmethod
    def build(
        component_types: list[Vtype],
        *,
        variable_names: list[str] | None = ...,
        binary_cols: list[list[int]] | None = ...,
        spin_cols: list[list[int]] | None = ...,
        int_cols: list[list[int]] | None = ...,
        real_cols: list[list[float]] | None = ...,
        raw_energies: list[float | None] | None = ...,
        timing: Timing | None = ...,
        counts: list[int] | None = ...,
    ) -> Solution:
        """Build a `Solution` based on the provided input data.

        The solution is constructed based on a column layout of the solution.
        Let's take the following sample-set with three samples as an example:

        [ 0  1  -1  3  2.2  1 ]
        [ 1  0  -1  6  3.8  0 ]
        [ 1  1  +1  2  2.4  0 ]

        Each row encodes a single sample. However, the variable types vary, the first,
        second, and last columns all represent a Binary variable (index 0, 1, 5). The
        third column represents a variable of type Spin (index 2). The fourth column
        (index 3), a variable of type Integer and the fifth column (index 4), a
        real-valued variable.

        Thus, the `component_types` list is:

        >>> component_types = [
        ...     Vtype.Binary,
        ...     Vtype.Binary,
        ...     Vtype.Spin,
        ...     Vtype.Integer,
        ...     Vtype.Real,
        ...     Vtype.Binary,
        ... ]

        Now we can extract all columns for a binary-valued variable and append them to
        a new list:

        >>> binary_cols = [[0, 1, 1], [1, 0, 1], [1, 0, 0]]

        where the first element in the list represents the first column, the second
        element the/second column and the third element the fifth column.
        We do the same for the remaining variable types:

        >>> spin_cols = [[-1, -1, +1]]
        >>> int_cols = [[3, 6, 2]]
        >>> real_cols = [[2.2, 3.8, 2.4]]

        If we know the raw energies, we can construct them as well:

        >>> raw_energies = [-200, -100, +300]

        And finally call the `build` function:

        >>> sol = Solution.build(
        ...     component_types,
        ...     binary_cols,
        ...     spin_cols,
        ...     int_cols,
        ...     real_cols,
        ...     raw_energies,
        ...     timing,
        ...     counts=[1, 1, 1],
        ... )
        >>> sol

        In this example, we could also neglect the `counts` as it defaults to `1`
        for all samples if not set:

        >>> sol = Solution.build(
        ...     component_types,
        ...     binary_cols,
        ...     spin_cols,
        ...     int_cols,
        ...     real_cols,
        ...     raw_energies,
        ...     timing,
        ... )
        >>> sol


        Parameters
        ----------
        component_types : list[Vtype]
            The variable type each element in a sample encodes.
        variable_names : list[Vtype], optional
            The name of each variable in the solution.
        binary_cols : list[list[int]], optional
            The data of all binary valued columns.
            Each inner list encodes a single binary-valued column. Required if any
            element in the `component_types` is `Vtype.Binary`.
        spin_cols : list[list[int]], optional
            The data of all spin-valued columns.
            Each inner list encodes a single spin-valued column. Required if any
            element in the `component_types` is `Vtype.Spin`.
        int_cols : list[list[int]], optional
            The data of all integer-valued columns.
            Each inner list encodes a single integer valued column. Required if any
            element in the `component_types` is `Vtype.Integer`.
        real_cols : list[list[float]], optional
            The data of all real-valued columns.
            Each inner list encodes a single real-valued column. Required if any
            element in the `component_types` is `Vtype.Real`.
        raw_energies : list[float, optional], optional
            The data of all real valued columns.
            Each inner list encodes a single real-valued column.
        timing : Timing, optional
            The timing data.
        counts : list[int], optional
            The number how often each sample in the solution has occurred.
            By default, 1 for all samples.

        Returns
        -------
        Solution
            The constructed solution

        Raises
        ------
        RuntimeError
            If a sample column has an incorrect number of samples or if `counts` has
            a length different from the number of samples given.
        """
        ...

    @overload
    @staticmethod
    def from_dict(data: dict[Variable, int]) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable, float]) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[str, int]) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[str, float]) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable | str, int | float]) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable, int], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable, float], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[str, int], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[str, float], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable | str, int | float], *, env: Environment
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable, int], *, model: Model) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable, float], *, model: Model) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[str, int], *, model: Model) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[str, float], *, model: Model) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable | str, int | float], *, model: Model
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable, int], *, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[Variable, float], *, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[str, int], *, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(data: dict[str, float], *, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable | str, int | float], *, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable, int], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable, float], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[str, int], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[str, float], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable | str, int | float], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable, int], *, model: Model, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable, float], *, model: Model, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[str, int], *, model: Model, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[str, float], *, model: Model, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dict(
        data: dict[Variable | str, int | float], *, model: Model, timing: Timing
    ) -> Solution: ...
    @staticmethod
    def from_dict(
        data: dict[Variable | str, int | float],
        *,
        env: Environment | None = ...,
        model: Model | None = ...,
        timing: Timing | None = ...,
    ) -> Solution:
        """Create a `Solution` from a dict.

        Create a `Solution` from a dict that maps variables or variable names to their
        assigned values.

        If a Model is passed, the solution will be evaluated immediately. Otherwise,
        there has to be an environment present to determine the correct variable types.

        Parameters
        ----------
        data : dict[Variable | str, int | float]
            The sample that shall be part of the solution.
        env : Environment, optional
            The environment the variable types shall be determined from.
        model : Model, optional
            A model to evaluate the sample with.

        Returns
        -------
        Solution
            The solution object created from the sample dict.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment or model is passed to the method or available from the
            context.
        ValueError
            If `env` and `model` are both present. When this is the case, the user's
            intention is unclear as the model itself already contains an environment.
        SolutionTranslationError
            Generally if the sample translation fails. Might be specified by one of the
            three following errors.
        SampleIncorrectLengthErr
            If a sample has a different number of variables than the environment.
        SampleUnexpectedVariableError
            If a sample has a variable that is not present in the environment.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...

    @overload
    @staticmethod
    def from_dicts(data: list[dict[Variable, int]]) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[Variable, float]]) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[str, int]]) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[str, float]]) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[Variable | str, int | float]]) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable, int]], *, env: Environment
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable, float]], *, env: Environment
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[str, int]], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[str, float]], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable | str, int | float]], *, env: Environment
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[Variable, int]], *, model: Model) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[Variable, float]], *, model: Model) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[str, int]], *, model: Model) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[str, float]], *, model: Model) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable | str, int | float]], *, model: Model
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[Variable, int]], *, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable, float]], *, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[str, int]], *, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(data: list[dict[str, float]], *, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable | str, int | float]], *, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable, int]], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable, float]], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[str, int]], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[str, float]], *, env: Environment, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable | str, int | float]],
        *,
        env: Environment,
        timing: Timing,
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable, int]], *, model: Model, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable, float]], *, model: Model, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[str, int]], *, model: Model, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[str, float]], *, model: Model, timing: Timing
    ) -> Solution: ...
    @overload
    @staticmethod
    def from_dicts(
        data: list[dict[Variable | str, int | float]], *, model: Model, timing: Timing
    ) -> Solution: ...
    @staticmethod
    def from_dicts(
        data: list[dict[Variable | str, int | float]],
        *,
        env: Environment | None = ...,
        model: Model | None = ...,
        timing: Timing | None = ...,
    ) -> Solution:
        """Create a `Solution` from multiple dicts.

        Create a `Solution` from multiple dicts that map variables or variable names to
        their assigned values. Duplicate samples contained in the `data` list are
        aggregated to a single sample.

        If a Model is passed, the solution will be evaluated immediately. Otherwise,
        there has to be an environment present to determine the correct variable types.

        Parameters
        ----------
        data : list[dict[Variable | str, int | float]]
            The samples that shall be part of the solution.
        env : Environment, optional
            The environment the variable types shall be determined from.
        model : Model, optional
            A model to evaluate the sample with.

        Returns
        -------
        Solution
            The solution object created from the sample dict.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment or model is passed to the method or available from the
            context.
        ValueError
            If `env` and `model` are both present. When this is the case, the user's
            intention is unclear as the model itself already contains an environment.
        SolutionTranslationError
            Generally if the sample translation fails. Might be specified by one of the
            three following errors.
        SampleIncorrectLengthErr
            If a sample has a different number of variables than the environment.
        SampleUnexpectedVariableError
            If a sample has a variable that is not present in the environment.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...

    def print(
        self,
        /,
        layout: Literal["row", "column"] = "column",
        max_line_length: int = 80,
        max_column_length: int = 5,
        max_lines: int = 10,
        max_var_name_length: int = 10,
        show_metadata: Literal["before", "after", "hide"] = "after",
    ) -> None:
        """
        Show a solution object as a human-readable string.

        This method provides various ways to customize the way the solution is
        represented as a string.

        Parameters
        ----------
        layout : Literal["row", "column"]
            With `"row"` layout, all assignments to one variable across different
            samples are shown in the same *row*, and each sample is shown in one
            column.
            With `"column"` layout, all assignments to one variable across different
            samples are shown in the same *column*, and each sample is shown in one row.
        max_line_length : int
            The max number of chars shown in one line or, in other words, the max width
            of a row.
        max_column_length : int
            The maximal number of chars in one column. For both the row and column
            layout, this controls the max number of chars a single variable assignment
            may be shown with. For the column layout, this also controls the max number
            of chars that a variable name is shown with.
            Note: the max column length cannot always be adhered to. This is
            specifically the case when a variable assignment is so high that the max
            column length is not sufficient to show the number correctly.
        max_lines : int
            The max number of lines used for showing the samples. Note that this
             parameter does not influence how metadata are shown, s.t. the total number
             of lines may be higher than `max_lines`.
        max_var_name_length : int
            The max number of chars that a variable is shown with in row layout. This
            parameter is ignored in column layout.
        show_metadata : Literal["before", "after", "hide"]
            Whether and where to show sample-specific metadata such as feasibility and
            objective value. Note that this parameter only controls how sample-specific
            metadata are shown. Other metadata, like the solution timing will be shown
            after the samples regardless of the value of this parameter.

            - `"before"`: show metadata before the actual sample, i.e., above the
                sample in row layout, and left of the sample in column layout.
            - `"after"`: show metadata after the actual sample, i.e., below the
                sample in row layout, and right of the sample in column layout.
            - "hide": do not show sample-specific metadata.

        Returns
        -------
        str
            The solution represented as a string.

        Raises
        ------
        ValueError
            If at least one of the params has an invalid value.
        """
        ...

# _sample.pyi
class SamplesIterator:
    """
    An iterator over a solution's samples.

    Examples
    --------
    >>> from luna_quantum import Solution
    >>> solution: Solution = ...

    Note: ``solution.samples`` is automatically converted into a ``SamplesIterator``.

    >>> for sample in solution.samples:
    ...     sample
    [0, -5, 0.28]
    [1, -4, -0.42]
    """

    def __iter__(self, /) -> SamplesIterator: ...
    def __next__(self, /) -> Sample: ...

class SampleIterator:
    """
    An iterator over the variable assignments of a solution's sample.

    Examples
    --------
    >>> from luna_quantum import Solution
    >>> solution: Solution = ...
    >>> sample = solution.samples[0]

    Note: ``sample`` is automatically converted into a ``SampleIterator``.

    >>> for var in sample:
    ...     var
    0
    -5
    0.28
    """

    def __iter__(self, /) -> SampleIterator: ...
    def __next__(self, /) -> int | float: ...

class Samples:
    """A set-like object containing every different sample of a solution.

    A samples object is simply a set-like object that contains every different sample
    of a solution. The ``Samples`` class is readonly as it's merely a helper class for
    looking into a solution's different samples.

    Examples
    --------
    >>> from luna_quantum import Model, Sample, Solution
    >>> model: Model = ...
    >>> solution: Solution = ...
    >>> samples: Samples = solution.samples
    >>> samples
    [0, -5, 0.28]
    [1, -4, -0.42]
    """

    def __str__(self, /) -> str: ...
    @overload
    def __getitem__(self, item: int, /) -> Sample: ...
    @overload
    def __getitem__(self, item: tuple[int, int], /) -> int | float: ...
    def __getitem__(self, item: int | tuple[int, int], /) -> int | float:
        """Extract a sample or variable assignment from the ``Samples`` object.

        If ``item`` is an int, returns the sample in this row. If ``item`` is a tuple
        of ints `(i, j)`, returns the variable assignment in row `i` and column `j`.

        Returns
        -------
        Sample or int or float

        Raises
        ------
        TypeError
            If ``item`` has the wrong type.
        IndexError
            If the row or column index is out of bounds for the variable environment.
        """
        ...

    def __len__(self, /) -> int:
        """
        Get the number of samples present in this sample set.

        Returns
        -------
        int
        """
        ...

    def __iter__(self, /) -> SamplesIterator:
        """
        Iterate over all samples of this sample set.

        Returns
        -------
        SamplesIterator
        """
        ...

    def tolist(self, /) -> list[list[int | float]]:
        """Convert sample into a 2-dimensional list.

        Convert the sample into a 2-dimensional list where a row constitutes a single
        sample, and a column constitutes all assignments for a single variable.

        Returns
        -------
        list[list[int | float]]
            The samples object as a 2-dimensional list.
        """
        ...

class Sample:
    """Assignment of actual values to the model's variables.

    A sample object is an assignment of an actual value to each of the model's
    variables.

    The ``Sample`` class is readonly as it's merely a helper class for looking into a
    single sample of a solution.

    Note: a ``Sample`` can be converted to ``list[int | float]`` simply by calling
    ``list(sample)``.

    Examples
    --------
    >>> from luna_quantum import Model, Sample, Solution
    >>> model: Model = ...
    >>> solution: Solution = ...
    >>> sample: Sample = solution.samples[0]
    >>> sample
    [0, -5, 0.28]
    """

    def __str__(self, /) -> str: ...
    @overload
    def __getitem__(self, item: int, /) -> int | float: ...
    @overload
    def __getitem__(self, item: Variable, /) -> int | float: ...
    def __getitem__(self, item: int | Variable, /) -> int | float:
        """
        Extract a variable assignment from the ``Sample`` object.

        Returns
        -------
        Sample or int or float

        Raises
        ------
        TypeError
            If ``item`` has the wrong type.
        IndexError
            If the row or column index is out of bounds for the variable environment.
        """
        ...

    def __len__(self, /) -> int:
        """
        Get the number of variables present in this sample.

        Returns
        -------
        int
        """
        ...

    def __iter__(self, /) -> SampleIterator:
        """
        Iterate over all variable assignments of this sample.

        Returns
        -------
        SampleIterator
        """
        ...

# _result.pyi
class ResultIterator:
    """
    An iterator over a solution's results.

    Examples
    --------
    >>> from luna_quantum import ResultIterator, Solution
    >>> solution: Solution = ...
    >>> results: ResultIterator = solution.results
    >>> for result in results:
    ...     result.sample
    [0, -5, 0.28]
    [1, -4, -0.42]
    """

    def __iter__(self, /) -> ResultIterator: ...
    def __next__(self, /) -> ResultView: ...

class Result:
    """
    A result object can be understood as a solution with only one sample.

    It can be obtained by calling `model.evaluate_sample` for a single sample.

    Most properties available for the solution object are also available for a result,
    but in the singular form. For example, you can call `solution.obj_values`, but
    `result.obj_value`.

    Examples
    --------
    >>> from luna_quantum import Model, Result, Solution
    >>> model: Model = ...
    >>> solution: Solution = ...
    >>> sample = solution.samples[0]
    >>> result = model.evaluate_sample(sample)
    >>> result.obj_value
    -109.42
    >>> result.sample
    [0, -5, 0.28]
    >>> result.constraints
    [True, False]
    >>> result.feasible
    False
    """

    @property
    def sample(self, /) -> Sample:
        """Get the sample of the result."""
        ...

    @property
    def obj_value(self, /) -> float | None:
        """Get the objective value of the result."""
        ...

    @property
    def constraints(self, /) -> NDArray | None:
        """The result's feasibility of all constraints.

        Get this result's feasibility values of all constraints. Note that
        `results.constraints[i]` iff. `model.constraints[i]` is feasible for
        this result.
        """
        ...

    @property
    def variable_bounds(self, /) -> NDArray | None:
        """Get this result's feasibility values of all variable bounds."""
        ...

    @property
    def feasible(self, /) -> bool | None:
        """Return whether all constraint results are feasible for this result."""
        ...

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...

class ResultView:
    """
    A result view object serves as a view into one row of a solution object.

    The `Result` class is readonly as it's merely a helper class for looking into a
    solution's row, i.e., a single sample and this sample's metadata.

    Most properties available for the solution object are also available for a result,
    but in the singular form. For example, you can call `solution.obj_values`, but
    `result.obj_value`.

    Examples
    --------
    >>> from luna_quantum import ResultView, Solution
    >>> solution: Solution = ...
    >>> result: ResultView = solution[0]
    >>> result.obj_value
    -109.42
    >>> result.sample
    [0, -5, 0.28]
    >>> result.constraints
    [True, False]
    >>> result.feasible
    False
    """

    @property
    def sample(self, /) -> Sample:
        """Get the sample of the result."""
        ...

    @property
    def counts(self, /) -> int:
        """Return how often this result appears in the solution."""
        ...

    @property
    def obj_value(self, /) -> float | None:
        """
        Get the objective value of this sample if present.

        This is the value computed by the corresponding AqModel.
        """
        ...

    @property
    def raw_energy(self, /) -> float | None:
        """
        Get the raw energy returned by the algorithm if present.

        This value is not guaranteed to be accurate under consideration of the
        corresponding AqModel.
        """
        ...

    @property
    def constraints(self, /) -> NDArray | None:
        """
        Get this result's feasibility values of all constraints.

        Note that `results.constraints[i]` iff. `model.constraints[i]` is feasible for
        this result.
        """
        ...

    @property
    def variable_bounds(self, /) -> NDArray | None:
        """Get this result's feasibility values of all variable bounds."""
        ...

    @property
    def feasible(self, /) -> bool | None:
        """Return whether all constraint results are feasible for this result."""
        ...

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
    def __eq__(self, other: ResultView, /) -> bool: ...  # type: ignore[reportIncompatibleMethodOverride]

# _model.pyi
class Sense(Enum):
    """
    Enumeration of optimization senses supported by the optimization system.

    This enum defines the type of optimization used for a model. The type influences
    the domain and behavior of the model during optimization.
    """

    Min = ...
    """Indicate the objective function to be minimized."""

    Max = ...
    """Indicate the objective function to be maximized."""

class Model:
    """
    A symbolic optimization model consisting of an objective and constraints.

    The `Model` class represents a structured symbolic optimization problem. It
    combines a scalar objective `Expression`, a collection of `Constraints`, and
    a shared `Environment` that scopes all variables used in the model.

    Models can be constructed explicitly by passing an environment, or implicitly
    by allowing the model to create its own private environment. If constructed
    inside an active `Environment` context (via `with Environment()`), that context
    is used automatically.

    Parameters
    ----------
    env : Environment, optional
        The environment in which variables and expressions are created. If not
        provided, the model will either use the current context (if active), or
        create a new private environment.
    name : str, optional
        An optional name assigned to the model.

    Examples
    --------
    Basic usage:

    >>> from luna_quantum import Model, Variable
    >>> model = Model("MyModel")
    >>> with model.environment:
    ...     x = Variable("x")
    ...     y = Variable("y")
    >>> model.objective = x * y + x
    >>> model.constraints += x >= 0
    >>> model.constraints += y <= 5

    With explicit environment:

    >>> from luna_quantum import Environment
    >>> env = Environment()
    >>> model = Model("ScopedModel", env)
    >>> with env:
    ...     x = Variable("x")
    ...     model.objective = x * x

    Serialization:

    >>> blob = model.encode()
    >>> restored = Model.decode(blob)
    >>> restored.name
    'MyModel'

    Notes
    -----
    - The `Model` class does not solve the optimization problem.
    - Use `.objective`, `.constraints`, and `.environment` to access the symbolic
      content.
    - Use `encode()` and `decode()` to serialize and recover models.
    """

    @overload
    def __init__(self, /) -> None: ...
    @overload
    def __init__(self, /, name: str) -> None: ...
    @overload
    def __init__(self, /, name: str, *, sense: Sense) -> None: ...
    @overload
    def __init__(self, /, name: str, *, env: Environment) -> None: ...
    @overload
    def __init__(self, /, *, sense: Sense) -> None: ...
    @overload
    def __init__(self, /, *, env: Environment) -> None: ...
    @overload
    def __init__(self, /, *, sense: Sense, env: Environment) -> None: ...
    @overload
    def __init__(self, /, name: str, *, sense: Sense, env: Environment) -> None: ...
    def __init__(
        self,
        /,
        name: str | None = ...,
        *,
        sense: Sense | None = ...,
        env: Environment | None = ...,
    ) -> None:
        """
        Initialize a new symbolic model.

        Parameters
        ----------
        name : str, optional
            An optional name for the model.
        env : Environment, optional
            The environment in which the model operates. If not provided, a new
            environment will be created or inferred from context.
        """
        ...

    def set_sense(self, /, sense: Sense) -> None:
        """
        Set the optimization sense of a model.

        Parameters
        ----------
        sense : Sense
            The sense of the model (minimization, maximization)
        """
        ...

    @property
    def name(self, /) -> str:
        """Return the name of the model."""
        ...

    @property
    def sense(self, /) -> Sense:
        """
        Get the sense of the model.

        Returns
        -------
        Sense
            The sense of the model (Min or Max).
        """
        ...

    @property
    def objective(self, /) -> Expression:
        """Get the objective expression of the model."""
        ...

    @objective.setter
    def objective(self, value: Expression, /) -> None:
        """Set the objective expression of the model."""
        ...

    @property
    def constraints(self, /) -> Constraints:
        """Access the set of constraints associated with the model."""
        ...

    @constraints.setter
    def constraints(self, value: Constraints, /) -> None:
        """Replace the model's constraints with a new set."""
        ...

    @property
    def environment(self, /) -> Environment:
        """Get the environment in which this model is defined."""
        ...

    @overload
    def variables(self, /) -> list[Variable]: ...
    @overload
    def variables(self, /, *, active: bool) -> list[Variable]: ...
    def variables(self, /, active: bool | None = ...) -> list[Variable]:
        """
        Get all variables that are part of this model.

        Parameters
        ----------
        active : bool, optional
            Instead of all variables from the environment, return only those that are
            actually present in the model's objective.

        Returns
        -------
        The model's variables as a list.
        """
        ...

    @overload
    def add_constraint(self, /, constraint: Constraint) -> None: ...
    @overload
    def add_constraint(self, /, constraint: Constraint, name: str) -> None: ...
    def add_constraint(self, /, constraint: Constraint, name: str | None = ...) -> None:
        """
        Add a constraint to the model's constraint collection.

        Parameters
        ----------
        constraint : Constraint
            The constraint to be added.
        name : str, optional
            The name of the constraint to be added.
        """
        ...

    @overload
    def set_objective(self, /, expression: Expression) -> None: ...
    @overload
    def set_objective(self, /, expression: Expression, *, sense: Sense) -> None: ...
    def set_objective(
        self, /, expression: Expression, *, sense: Sense | None = ...
    ) -> None:
        """
        Set the model's objective to this expression.

        Parameters
        ----------
        expression : Expression
            The expression assigned to the model's objective.
        sense : Sense, optional
            The sense of the model for this objective, by default Sense.Min.
        """
        ...

    @property
    def num_constraints(self, /) -> int:
        """
        Return the number of constraints defined in the model.

        Returns
        -------
        int
            Total number of constraints.
        """
        ...

    def evaluate(self, /, solution: Solution) -> Solution:
        """
        Evaluate the model given a solution.

        Parameters
        ----------
        solution : Solution
            The solution used to evaluate the model with.

        Returns
        -------
        Solution
            A new solution object with filled-out information.
        """
        ...

    def evaluate_sample(self, /, sample: Sample) -> Result:
        """
        Evaluate the model given a single sample.

        Parameters
        ----------
        sample : Sample
            The sample used to evaluate the model with.

        Returns
        -------
        Result
            A result object containing the information from the evaluation process.
        """
        ...

    @overload
    def encode(self, /) -> bytes: ...
    @overload
    def encode(self, /, *, compress: bool) -> bytes: ...
    @overload
    def encode(self, /, *, level: int) -> bytes: ...
    @overload
    def encode(self, /, compress: bool, level: int) -> bytes: ...
    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """
        Serialize the model into a compact binary format.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the binary output. Default is True.
        level : int, optional
            Compression level (0-9). Default is 3.

        Returns
        -------
        bytes
            Encoded model representation.

        Raises
        ------
        IOError
            If serialization fails.
        """
        ...

    @overload
    def serialize(self, /) -> bytes: ...
    @overload
    def serialize(self, /, *, compress: bool) -> bytes: ...
    @overload
    def serialize(self, /, *, level: int) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool, level: int) -> bytes: ...
    def serialize(
        self, /, compress: bool | None = ..., level: int | None = ...
    ) -> bytes:
        """
        Alias for `encode()`.

        See `encode()` for full documentation.
        """
        ...

    @classmethod
    def decode(cls, data: bytes) -> Model:
        """
        Reconstruct a symbolic model from binary data.

        Parameters
        ----------
        data : bytes
            Serialized model blob created by `encode()`.

        Returns
        -------
        Model
            The reconstructed model.

        Raises
        ------
        DecodeError
            If decoding fails due to corruption or incompatibility.
        """
        ...

    @classmethod
    def deserialize(cls, data: bytes) -> Model:
        """
        Alias for `decode()`.

        See `decode()` for full documentation.
        """
        ...

    def __eq__(self, other: Model, /) -> bool:  # type: ignore[reportIncompatibleMethodOverride]
        """
        Check whether this model is equal to `other`.

        Parameters
        ----------
        other : Model

        Returns
        -------
        bool
        """
        ...

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
    def __hash__(self, /) -> int: ...

# _expression.pyi
class Expression:
    """
    Polynomial expression supporting symbolic arithmetic, constraint creation.

    An `Expression` represents a real-valued mathematical function composed of
    variables, scalars, and coefficients. Expressions may include constant, linear,
    quadratic, and higher-order terms (cubic and beyond). They are used to build
    objective functions and constraints in symbolic optimization models.

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
    def encode(self, /, *, compress: bool) -> bytes: ...
    @overload
    def encode(self, /, *, level: int) -> bytes: ...
    @overload
    def encode(self, /, compress: bool, level: int) -> bytes: ...
    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """
        Serialize the expression into a compact binary format.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the data. Default is True.
        level : int, optional
            Compression level (0-9). Default is 3.

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
    def serialize(self, /, *, compress: bool) -> bytes: ...
    @overload
    def serialize(self, /, *, level: int) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool, level: int) -> bytes: ...
    def serialize(
        self, /, compress: bool | None = ..., level: int | None = ...
    ) -> bytes:
        """
        Alias for `encode()`.

        See `encode()` for full documentation.
        """
        ...

    @classmethod
    def decode(cls, data: bytes) -> Expression:
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

    @classmethod
    def deserialize(cls, data: bytes) -> Expression:
        """
        Alias for `decode()`.

        See `decode()` for full documentation.
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
    def __iadd__(self, other: Expression, /) -> Self: ...
    @overload
    def __iadd__(self, other: Variable, /) -> Self: ...
    @overload
    def __iadd__(self, other: int, /) -> Self: ...
    @overload
    def __iadd__(self, other: float, /) -> Self: ...
    def __iadd__(self, other: Expression | Variable | int | float, /) -> Self:
        """
        In-place addition.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Self

        Raises
        ------
        VariablesFromDifferentEnvsError
            If operands are from different environments.
        TypeError
            If the operand type is unsupported.
        """
        ...

    @overload
    def __isub__(self, other: Expression, /) -> Self: ...
    @overload
    def __isub__(self, other: Variable, /) -> Self: ...
    @overload
    def __isub__(self, other: int, /) -> Self: ...
    @overload
    def __isub__(self, other: float, /) -> Self: ...
    def __isub__(self, other: Expression | Variable | int | float, /) -> Self:
        """
        In-place subtraction.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Self

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
    def __imul__(self, other: Expression, /) -> Self: ...
    @overload
    def __imul__(self, other: Variable, /) -> Self: ...
    @overload
    def __imul__(self, other: int, /) -> Self: ...
    @overload
    def __imul__(self, other: float, /) -> Self: ...
    def __imul__(self, other: Expression | Variable | int | float, /) -> Self:
        """
        In-place multiplication.

        Parameters
        ----------
        other : Expression, Variable, int, or float

        Returns
        -------
        Self

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
    def __eq__(self, rhs: int, /) -> Constraint: ...
    @overload
    def __eq__(self, rhs: float, /) -> Constraint: ...
    def __eq__(self, rhs: Expression | Variable | int | float, /) -> Constraint:  # type: ignore[reportIncompatibleMethodOverride]
        """
        Compare to a different expression or create a constraint `expression == scalar`.

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

    @property
    def _environment(self, /) -> Environment:
        """Get this expression's environment."""
        ...

    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...

# _environment.pyi
class Environment:
    """
    Execution context for variable creation and expression scoping.

    An `Environment` provides the symbolic scope in which `Variable`s are defined.
    It is required for constructing variables ensuring consistency across expressions.
    The environment does **not** store constraints or expressions — it only facilitates
    their creation by acting as a context manager and anchor for `Variable` instances.

    Environments are best used with `with` blocks, but can also be passed manually
    to models or variables.

    Examples
    --------
    Create variables inside an environment:

    >>> from luna_quantum import Environment, Variable
    >>> with Environment() as env:
    ...     x = Variable("x")
    ...     y = Variable("y")

    Serialize the environment state:

    >>> data = env.encode()
    >>> expr = Environment.decode(data)

    Notes
    -----
    - The environment is required to create `Variable` instances.
    - It does **not** own constraints or expressions — they merely reference variables
      tied to an environment.
    - Environments **cannot be nested**. Only one can be active at a time.
    - Use `encode()` / `decode()` to persist and recover expression trees.
    """

    def __init__(self, /) -> None:
        """
        Initialize a new environment for variable construction.

        It is recommended to use this in a `with` statement to ensure proper scoping.
        """
        ...

    def __enter__(self, /) -> Self:
        """
        Activate this environment for variable creation.

        Returns
        -------
        Environment
            The current environment (self).

        Raises
        ------
        MultipleActiveEnvironmentsError
            If another environment is already active.
        """
        ...

    def __exit__(
        self,
        /,
        exc_type: type[BaseException] | None = ...,
        exc_value: BaseException | None = ...,
        exc_traceback: TracebackType | None = ...,
    ) -> None:
        """
        Deactivate this environment.

        Called automatically at the end of a `with` block.
        """
        ...

    def get_variable(self, /, name: str) -> Variable:
        """
        Get a variable by its label (name).

        Parameters
        ----------
        name : str
            The name/label of the variable

        Returns
        -------
        Variable
            The variable with the specified label/name.

        Raises
        ------
        VariableNotExistingError
            If no variable with the specified name is registered.
        """
        ...

    @overload
    def encode(self, /) -> bytes: ...
    @overload
    def encode(self, /, *, compress: bool) -> bytes: ...
    @overload
    def encode(self, /, *, level: int) -> bytes: ...
    @overload
    def encode(self, /, compress: bool, level: int) -> bytes: ...
    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """
        Serialize the environment into a compact binary format.

        This is the preferred method for persisting an environment's state.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the binary output. Default is `True`.
        level : int, optional
            Compression level (e.g., from 0 to 9). Default is `3`.

        Returns
        -------
        bytes
            Encoded binary representation of the environment.

        Raises
        ------
        IOError
            If serialization fails.
        """
        ...

    @overload
    def serialize(self, /) -> bytes: ...
    @overload
    def serialize(self, /, *, compress: bool) -> bytes: ...
    @overload
    def serialize(self, /, *, level: int) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool, level: int) -> bytes: ...
    def serialize(
        self, /, compress: bool | None = ..., level: int | None = ...
    ) -> bytes:
        """
        Alias for `encode()`.

        See `encode()` for full usage details.
        """
        ...

    @classmethod
    def decode(cls, data: bytes) -> Environment:
        """
        Reconstruct an expression from a previously encoded binary blob.

        Parameters
        ----------
        data : bytes
            The binary data returned from `Environment.encode()`.

        Returns
        -------
        Expression
            The reconstructed symbolic expression.

        Raises
        ------
        DecodeError
            If decoding fails due to corruption or incompatibility.
        """
        ...

    @classmethod
    def deserialize(cls, data: bytes) -> Environment:
        """
        Alias for `decode()`.

        See `decode()` for full usage details.
        """
        ...

    def __eq__(self, other: Environment, /) -> bool: ...  # type: ignore[reportIncompatibleMethodOverride]
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...

# _constraints.pyi
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
        Get the left-hand side of the constraint.

        Returns
        -------
        Expression
            The left-hand side expression.
        """
        ...

    @property
    def rhs(self, /) -> float:
        """
        Get the right-hand side of the constraint.

        Returns
        -------
        float
            The right-hand side expression.
        """
        ...

    @property
    def comparator(self, /) -> Comparator:
        """
        Get the comparator of the constraint.

        Returns
        -------
        Comparator
            The comparator of the constraint.
        """
        ...

    def __eq__(self, other: Constraint, /) -> bool: ...  # type: ignore[reportIncompatibleMethodOverride]
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
    def add_constraint(self, /, constraint: Constraint) -> None: ...
    @overload
    def add_constraint(self, /, constraint: Constraint, name: str) -> None: ...
    def add_constraint(self, /, constraint: Constraint, name: str | None = ...) -> None:
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
    def encode(self, /, *, compress: bool) -> bytes: ...
    @overload
    def encode(self, /, *, level: int) -> bytes: ...
    @overload
    def encode(self, /, compress: bool, level: int) -> bytes: ...
    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """
        Serialize the constraint collection to a binary blob.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the result. Default is True.
        level : int, optional
            Compression level (0-9). Default is 3.

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
    def serialize(self, /, *, compress: bool) -> bytes: ...
    @overload
    def serialize(self, /, *, level: int) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool, level: int) -> bytes: ...
    def serialize(
        self, /, compress: bool | None = ..., level: int | None = ...
    ) -> bytes:
        """
        Alias for `encode()`.

        See `encode()` for details.
        """
        ...

    @classmethod
    def decode(cls, data: bytes, env: Environment) -> Expression:
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

    @classmethod
    def deserialize(cls, data: bytes, env: Environment) -> Expression:
        """
        Alias for `decode()`.

        See `decode()` for usage.
        """
        ...

    @overload
    def __iadd__(self, constraint: Constraint, /) -> Self: ...
    @overload
    def __iadd__(self, constraint: tuple[Constraint, str], /) -> Self: ...
    def __iadd__(self, constraint: Constraint | tuple[Constraint, str], /) -> Self:
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

    def __eq__(self, other: Constraints, /) -> bool: ...  # type: ignore[reportIncompatibleMethodOverride]
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
    def __getitem__(self, item: int, /) -> Constraint: ...
    def __len__(self, /) -> int:
        """
        Get the number of constraints.

        Returns
        -------
        int
            The number of constraints associated with this `Constraints` object.
        """
        ...

__all__ = [
    "Bounds",
    "Comparator",
    "Constraint",
    "Constraints",
    "Environment",
    "Expression",
    "Model",
    "Result",
    "ResultIterator",
    "ResultView",
    "Sample",
    "SampleIterator",
    "Samples",
    "SamplesIterator",
    "Sense",
    "Solution",
    "Timer",
    "Timing",
    "Variable",
    "Vtype",
    "errors",
    "translator",
]
