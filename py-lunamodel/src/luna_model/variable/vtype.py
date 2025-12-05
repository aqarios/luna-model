from __future__ import annotations
from enum import Enum

from luna_model._lm import PyVtype


class Vtype(Enum):

    BINARY = PyVtype.Binary
    # todo: have this only be part of a specialization? So that it can be returned but not used
    # directly? I.e., forbid generation of a inverted binary variable by passing this vtype to
    # the variable's constructor?
    INVERTED_BINARY = PyVtype.InvertedBinary
    SPIN = PyVtype.Spin
    INTEGER = PyVtype.Integer
    REAL = PyVtype.Real

    # todo: deprecate the below

    Binary = PyVtype.Binary
    InvertedBinary = PyVtype.InvertedBinary
    Spin = PyVtype.Spin
    Integer = PyVtype.Integer
    Real = PyVtype.Real
