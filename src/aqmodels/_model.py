from enum import Enum
from aqmodels._api_utils import export, dispatched


@export
class Sense(Enum):
    """
    Enumeration of optimization senses supported by the optimization system.

    This enum defines the type of a optimization used for a model. The type influences
    the domain and behavior of the model during optimization.
    """

    Min = ...
    """Indicate the objective function to be minimized."""

    Max = ...
    """Indicate the objective function to be maximized."""


@export
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

    >>> from aqmodels import Model, Variable
    >>> model = Model("MyModel")
    >>> with model.environment:
    ...     x = Variable("x")
    ...     y = Variable("y")
    >>> model.objective = x * y + x
    >>> model.constraints += x >= 0
    >>> model.constraints += y <= 5

    With explicit environment:

    >>> from aqmodels import Environment
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

    @dispatched
    def __init__(self, name, env):
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
        return env, name

    @dispatched
    def set_sense(self, sense):
        """
        Set the optimization sense of a model.

        Parameters
        ----------
        sense : Sense
            The sense of the model (minimization, maximization)
        """
        return sense

    @dispatched
    def evaluate(self, solution):
        """
        Evaluate the model given a solution.

        Parameters
        ----------
        solution : Solution
            The solution used to evaluate the model with.

        Returns
        -------
        Solution
            A new solution object with filled out information.
        """
        return solution

    @dispatched
    def evaluate_sample(self, sample):
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
        return sample

    @property
    @dispatched
    def name(self):
        """Return the name of the model."""
        return

    @property
    def sense(self):
        """
        Get the sense of the model

        Returns
        -------
        Sense
            The sense of the model (Min or Max).
        """
        return

    @property
    @dispatched
    def objective(self):
        """Get the objective expression of the model."""
        return

    @objective.setter
    @dispatched
    def objective(self, value):
        """Set the objective expression of the model."""
        return value

    @property
    @dispatched
    def constraints(self):
        """Access the set of constraints associated with the model."""
        return

    @constraints.setter
    @dispatched
    def constraints(self, value):
        """Replace the model's constraints with a new set."""
        return value

    @property
    @dispatched
    def environment(self):
        """Get the environment in which this model is defined."""
        return

    @dispatched
    def num_constraints(self):
        """
        Return the number of constraints defined in the model.

        Returns
        -------
        int
            Total number of constraints.
        """
        return

    @dispatched
    def encode(self, compress=True, level=3):
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
        return compress, level

    @dispatched
    def serialize(self, compress=True, level=3):
        """Alias for `encode()`."""
        return compress, level

    @dispatched
    @staticmethod
    def decode(data):
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
        return data

    @dispatched
    @staticmethod
    def deserialize(data):
        """Alias for `decode()`."""
        return data
