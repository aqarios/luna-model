from __future__ import annotations
from enum import Enum

from luna_model._lm import PyVtype


class Vtype(Enum):
    BINARY = PyVtype.Binary
    INVERTED_BINARY = PyVtype.InvertedBinary
    SPIN = PyVtype.Spin
    INTEGER = PyVtype.Integer
    REAL = PyVtype.Real

    Binary = PyVtype.Binary
    InvertedBinary = PyVtype.InvertedBinary
    Spin = PyVtype.Spin
    Integer = PyVtype.Integer
    Real = PyVtype.Real
