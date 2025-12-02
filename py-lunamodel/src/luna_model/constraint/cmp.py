from enum import Enum

from luna_model._lm import PyComparator


class Comparator(Enum):
    EQ = PyComparator.Eq
    LE = PyComparator.Le
    GE = PyComparator.Ge

    Eq = PyComparator.Eq
    Le = PyComparator.Le
    Ge = PyComparator.Ge
