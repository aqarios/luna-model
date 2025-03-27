from enum import Enum
from aqmodels._api_utils import dispatched, export


@export
class Comparator(Enum):
    """ """

    Eq = ...
    """ """
    Leq = ...
    """ """
    Geq = ...
    """ """


@export
class Constraint:
    """ """

    @dispatched
    def __init__(self, lhs, rhs, comparator):
        """ """
        return lhs, rhs, comparator


@export
class Constraints:
    """ """

    @dispatched
    def add_constraint(self, constraint):
        """ """
        return constraint

    @dispatched
    def encode(self, compress, level):
        """ """
        return compress, level

    @dispatched
    def serialize(self, compress, level):
        """ """
        return compress, level

    @dispatched
    @staticmethod
    def decode(data):
        """ """
        return data

    @dispatched
    @staticmethod
    def deserialize(data):
        """ """
        return data
