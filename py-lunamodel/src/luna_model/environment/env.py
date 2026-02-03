"""Environment for managing optimization model variables.

This module provides the Environment class that manages variables and their
relationships in an optimization model. Environments ensure consistency
across variables and expressions.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from luna_model._lm import PyEnvironment
from luna_model._utils import wrap_var

if TYPE_CHECKING:
    from collections.abc import Callable

    from luna_model.variable import Variable


class Environment:
    """Environment for managing model variables and their relationships.

    An Environment is a container that manages all variables in an optimization
    model. It ensures that variables used together come from the same environment
    and maintains consistency across expressions and constraints.

    Environments can be used as context managers to automatically manage
    variable scoping.

    Attributes
    ----------
    num_variables : int
        The number of variables registered in this environment.
    id : int
        Unique identifier for this environment.

    Examples
    --------
    Use as a context manager:

    >>> from luna_model import Environment, Variable
    >>> with Environment() as env:
    ...     x = Variable("x", env=env)
    ...     y = Variable("y", env=env)

    Create and manage explicitly:

    >>> env = Environment()
    >>> x = Variable("x", env=env)
    >>> print(env.num_variables)  # 1
    >>> var = env.get_variable("x")

    Notes
    -----
    Variables from different environments cannot be combined in the same
    expression or constraint. This prevents accidental mixing of unrelated
    models.

    See Also
    --------
    Variable : Variables that belong to environments.
    Model : Models that use environments.
    """

    _env: PyEnvironment

    def __init__(self) -> None:
        """Initialize a new environment."""
        self._env = PyEnvironment()

    @classmethod
    def _from_pyenv(cls, py_env: PyEnvironment) -> Environment:
        env = cls.__new__(cls)
        env._env = py_env
        return env

    @classmethod
    def _from_ctx(cls) -> Environment | None:
        return Environment._from_pyenv(PyEnvironment._from_ctx())

    def __enter__(self) -> Environment:  # noqa: PYI034
        """Enter the environment context."""
        return Environment._from_pyenv(self._env.__enter__())

    def __exit__(self, exc_type, exc_value, exc_traceback) -> None:  # noqa: ANN001
        """Exit the environment context."""
        return self._env.__exit__(exc_type, exc_value, exc_traceback)

    @property
    def num_variables(self) -> int:
        """Get the number of variables in this environment.

        Returns
        -------
        int
            The number of registered variables.
        """
        return self._env.num_variables

    @property
    def id(self) -> int:
        """Get the unique identifier for this environment.

        Returns
        -------
        int
            The environment ID.
        """
        return self._env.id

    def get_variable(self, name: str) -> Variable:
        """Get a variable by name.

        Parameters
        ----------
        name : str
            The variable name.

        Returns
        -------
        Variable
            The variable with the given name.

        Raises
        ------
        VariableNotExistingError
            If no variable with the given name exists.
        """
        return wrap_var(self._env.get_variable(name))

    def variables(self) -> list[Variable]:
        """Get all variables in this environment.

        Returns
        -------
        list[Variable]
            List of all registered variables.
        """
        return [wrap_var(v) for v in self._env.variables()]

    def equal_contents(self, other: Environment) -> bool:
        """Check if two environments have equal content.

        Parameters
        ----------
        other : Environment
            The environment to compare with.

        Returns
        -------
        bool
            True if environments have the same variables.
        """
        return self._env.equal_contents(other._env)

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Encode the environment."""
        return self._env.encode(compress, level)

    def serialize(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Serliaze the environment. Same as encode."""
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes) -> Environment:
        """Decode the environment from its byte representation."""
        return cls._from_pyenv(PyEnvironment.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Environment:
        """Deserialize the environment from its byte representation. Same as decode."""
        return cls.decode(data)

    def __reduce__(self) -> tuple[Callable[[bytes], Environment], tuple[bytes]]:
        """Reduce environment to its byte representation. Used by pickle."""
        return (Environment.decode, (self.encode(),))

    def __eq__(self, other: Environment) -> bool:  # type: ignore[override]
        """Reduce environment to its byte representation. Used by pickle."""
        return self._env.__eq__(other._env)

    def __contains__(self, var: str) -> bool:
        """Check if a variable name exists in this environment.

        Parameters
        ----------
        var : str
            The variable name to check.

        Returns
        -------
        bool
            True if a variable with the given name exists.
        """
        return self._env.__contains__(var)

    def __str__(self) -> str:
        """Environment as a string."""
        return self._env.__str__()

    def __repr__(self) -> str:
        """Environment as a debug string."""
        return self._env.__repr__()
