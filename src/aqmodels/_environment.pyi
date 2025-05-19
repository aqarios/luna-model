from typing import Any
from typing import overload

from aqmodels._variable import Variable

class Environment:
    """
    Execution context for variable creation and expression scoping.

    An `Environment` provides the symbolic scope in which `Variable` objects are defined.
    It is required for variable construction, and ensures consistency across expressions.
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
    - It does **not** own constraints or expressions — they merely reference variables tied to an environment.
    - Environments **cannot be nested**. Only one can be active at a time.
    - Use `encode()` / `decode()` to persist and recover expression trees.
    """

    def __init__(self, /) -> None:
        """
        Initialize a new environment for variable construction.

        It is recommended to use this in a `with` statement to ensure proper scoping.
        """
        ...

    def __enter__(self, /) -> Any:
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

    def __exit__(self, /, exc_type, exc_value, exc_traceback) -> None:
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
        label : str
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
    def encode(self, /, compress: bool) -> bytes: ...
    @overload
    def encode(self, /, *, level: int) -> bytes: ...
    @overload
    def encode(self, /, compress: bool, level: int) -> bytes: ...
    def encode(self, /, compress: bool | None = ..., level: int | None = ...) -> bytes:
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
    def serialize(self, /, compress: bool) -> bytes: ...
    @overload
    def serialize(self, /, *, level: int) -> bytes: ...
    @overload
    def serialize(self, /, compress: bool, level: int) -> bytes: ...
    def serialize(self, /, compress: bool | None = ..., level: int | None = ...) -> bytes:
        """
        Alias for `encode()`.

        See `encode()` for full usage details.
        """
        ...

    @staticmethod
    def decode(data: bytes) -> Environment:
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

    @staticmethod
    def deserialize(data: bytes) -> Environment:
        """
        Alias for `decode()`.

        See `decode()` for full usage details.
        """
        ...

    def __eq__(self, other: Environment, /) -> bool: ... # type: ignore
    def __str__(self, /) -> str: ...
    def __repr__(self, /) -> str: ...
