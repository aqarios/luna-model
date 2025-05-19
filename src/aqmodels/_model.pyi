from enum import Enum
from typing import overload

from aqmodels._constraints import Constraint
from aqmodels._constraints import Constraints
from aqmodels._variable import Variable
from aqmodels._environment import Environment
from aqmodels._expression import Expression
from aqmodels._result import Result
from aqmodels._sample import Sample
from aqmodels._solution import Solution

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
    - Use `.objective`, `.constraints`, and `.environment` to access the symbolic content.
    - Use `encode()` and `decode()` to serialize and recover models.
    """

    @overload
    def __init__(self, /) -> None: ...
    @overload
    def __init__(self, /, name: str) -> None: ...
    @overload
    def __init__(self, /, *, env: Environment) -> None: ...
    @overload
    def __init__(self, /, name: str, env: Environment) -> None: ...
    def __init__(
        self,
        /,
        name: str | None = ...,
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
        Get the sense of the model

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
    def objective(self, value: Expression, /):
        """Set the objective expression of the model."""
        ...

    @property
    def constraints(self, /) -> Constraints:
        """Access the set of constraints associated with the model."""
        ...

    @constraints.setter
    def constraints(self, value: Constraints, /):
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
    def add_constraint(self, /, constraint: Constraint): ...
    @overload
    def add_constraint(self, /, constraint: Constraint, name: str): ...
    def add_constraint(self, /, constraint: Constraint, name: str | None = ...):
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
    def set_objective(self, /, expression: Expression): ...
    @overload
    def set_objective(self, /, expression: Expression, *, sense: Sense): ...
    def set_objective(self, /, expression: Expression, *, sense: Sense | None = ...):
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
    def encode(self, /, compress: bool | None = ..., level: int | None = ...) -> bytes:
        """
        Serialize the model into a compact binary format.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the binary output. Default is True.
        level : int, optional
            Compression level (0–9). Default is 3.

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

    @staticmethod
    def decode(data: bytes) -> Model:
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

    @staticmethod
    def deserialize(data: bytes) -> Model:
        """
        Alias for `decode()`.

        See `decode()` for full documentation.
        """
        ...

    def __eq__(self, other: Model, /) -> bool:  # type: ignore
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
