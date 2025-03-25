from __future__ import annotations
from enum import Enum
from aqmodels._expression import Expression


# @export
class Vtype(Enum):
    """This is the vtype"""

    Real = ...
    """Real documentation"""
    Integer = ...
    """Integer documentation"""
    Binary = ...
    """Binary documentation"""
    Spin = ...
    """Spin documentation"""

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


# @export
class Variable:
    """
    This is a variable
    """

    def __init__(self, name, env, vtype, bounds) -> None:
        """
        Create a new variable
        """
        ...

    def __new__(cls, name, env, vtype, bounds) -> Variable:
        """
        Create a new variable
        """
        ...

    def __add__(self, other) -> Expression:
        """
        Description of __add__
        """
        return other

    def __radd__(self, other) -> Expression:
        """
        Description of `__radd__`
        """
        ...

    def __sub__(self, other) -> Expression:
        """
        Description of __sub__
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __rsub__(self, other) -> Expression:
        """
        Description of `__rsub__`
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __mul__(self, other) -> Expression:
        """
        Description of `__mul__`
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __rmul__(self, other) -> Expression:
        """
        Description of `__rmul__`
        """
        raise TypeError(f"Invalid input type '{type(other)}' for other")

    def __str__(self) -> str:
        """
        Description of `__str__`
        """
        raise TypeError()

    def __repr__(self) -> str:
        """
        Description of `__repr__`
        """
        raise TypeError()
