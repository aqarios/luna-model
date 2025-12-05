from __future__ import annotations
from enum import Enum

from luna_model._lm import PyConstraintType


class ConstraintType(Enum):
    UNCONSTRAINED = PyConstraintType.Unconstrained
    EQUALITY = PyConstraintType.Equality
    INEQUALITY = PyConstraintType.Inequality
    LESS_EQUAL = PyConstraintType.LessEqual
    GREATER_EQUAL = PyConstraintType.GreaterEqual

    # below is to be deprecated

    Unconstrained = PyConstraintType.Unconstrained
    Equality = PyConstraintType.Equality
    Inequality = PyConstraintType.Inequality
    LessEqual = PyConstraintType.LessEqual
    GreaterEqual = PyConstraintType.GreaterEqual
