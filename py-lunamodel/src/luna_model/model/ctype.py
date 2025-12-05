from __future__ import annotations
from enum import Enum

from luna_model._lm import PyCtype


class Ctype(Enum):
    UNCONSTRAINED = PyCtype.Unconstrained
    EQUALITY = PyCtype.Equality
    INEQUALITY = PyCtype.Inequality
    LESS_EQUAL = PyCtype.LessEqual
    GREATER_EQUAL = PyCtype.GreaterEqual

    # below is to be deprecated

    Unconstrained = PyCtype.Unconstrained
    Equality = PyCtype.Equality
    Inequality = PyCtype.Inequality
    LessEqual = PyCtype.LessEqual
    GreaterEqual = PyCtype.GreaterEqual
