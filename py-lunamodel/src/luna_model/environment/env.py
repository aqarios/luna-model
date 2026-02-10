# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
from __future__ import annotations

from typing import TYPE_CHECKING

from luna_model._lm import PyEnvironment
from luna_model._utils import wrap_var

if TYPE_CHECKING:
    from collections.abc import Callable

    from luna_model.variable import Variable


class Environment:
    """Environment for managing model variables and their relationships.

    An Environment is a container that manages all variables created by the user.
    It ensures that variables used together come from the same environment
    and maintains consistency across multiple expressions.

    Environments serve as context managers to automatically manage
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

    def encode(self) -> bytes:
        """Serialize the environment into a compact binary format.

        Returns
        -------
        bytes
            Encoded environment representation.

        Raises
        ------
        CompressionError
            If compression fails.
        """
        return self._env.encode()

    def serialize(self) -> bytes:
        """Serialize the environment into a compact binary format.

        This is an alias for :meth:`encode`.

        Returns
        -------
        bytes
            Encoded environment representation.
        """
        return self.encode()

    @classmethod
    def decode(cls, data: bytes) -> Environment:
        """Reconstruct an environment from encoded bytes.

        Parameters
        ----------
        data : bytes
            Binary blob returned by :meth:`encode` or :meth:`serialize`.

        Returns
        -------
        Environment
            Deserialized environment object.

        Raises
        ------
        DecodingError
            If decoding fails due to corruption or incompatibility.

        Examples
        --------
        >>> original = Environment()
        >>> ...
        >>> blob = original.encode()
        >>> restored = Environment.decode(blob)
        """
        return cls._from_pyenv(PyEnvironment.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Environment:
        """Reconstruct an environment from encoded bytes.

        This is an alias for :meth:`decode`.

        Parameters
        ----------
        data : bytes
            Binary blob returned by encode().

        Returns
        -------
        Model
            Deserialized environment object.
        """
        return cls.decode(data)

    def __reduce__(self) -> tuple[Callable[[bytes], Environment], tuple[bytes]]:
        """Support for pickle serialization.

        Returns
        -------
        tuple
            A tuple of (decoder_function, encoded_data) for pickle.

        Notes
        -----
        This method is called automatically by Python's pickle module.
        """
        return (Environment.decode, (self.encode(),))

    def __eq__(self, other: Environment) -> bool:  # type: ignore[override]
        """Check if two environments are exactly equal."""
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
