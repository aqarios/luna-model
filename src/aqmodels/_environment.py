from __future__ import annotations
from typing import Any
from aqmodels._expression import Expression


# @export
class Environment:
    """
    Documentation of Environment
    """

    def __init__(self) -> None:
        """
        Documentation of init
        """
        ...

    def encode(self, compress, level) -> bytes:
        """
        Documentation of encode
        """
        ...

    def serialize(self, compress, level) -> bytes:
        """
        Documentation of serialize
        """
        ...

    def __enter__(self) -> Any:
        """
        Documentation of __enter__
        """
        ...

    def __exit__(self, exc_type, exc_value, exc_traceback) -> None:
        """
        Documentation of __exit__
        """
        ...

    def __str__(self) -> str:
        """
        Description of `__str__`
        """
        ...

    def __repr__(self) -> str:
        """
        Description of `__repr__`
        """
        ...

    @staticmethod
    def decode(data) -> Expression:
        """
        Documentation of add_constraint
        """
        ...

    @staticmethod
    def deserialize(data) -> Expression:
        """
        Documentation of deserialize
        """
        ...
