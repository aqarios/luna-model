from enum import Enum
from aqmodels._api_utils import dispatched, export


@export
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


@export
class Bounds:
    """"""

    @dispatched
    def __init__(self, lower, upper):
        """
        Create the bounds for a variable. (only integer and real).
        """
        return lower, upper


@export
class Variable:
    """
    This is a variable
    """

    @dispatched
    def __init__(self, name, env, vtype, bounds):
        """
        Create a new variable
        """
        return name, env, vtype, bounds
