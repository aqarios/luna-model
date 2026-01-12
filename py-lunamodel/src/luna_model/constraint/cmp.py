from __future__ import annotations
from enum import Enum

from luna_model._lm import PyComparator


class Comparator(Enum):
    EQ = "Eq"
    LE = "Le"
    GE = "Ge"

    Eq = "Eq"
    Le = "Le"
    Ge = "Ge"

    @property
    def name(self) -> str:
        match self:
            case Comparator.EQ | Comparator.Eq:
                return "Eq"
            case Comparator.LE | Comparator.Le:
                return "Le"
            case Comparator.GE | Comparator.Ge:
                return "Ge"

    @property
    def _val(self) -> PyComparator:
        match self:
            case Comparator.EQ | Comparator.Eq:
                return PyComparator.Eq
            case Comparator.LE | Comparator.Le:
                return PyComparator.Le
            case Comparator.GE | Comparator.Ge:
                return PyComparator.Ge

    @classmethod
    def _from_pycmp(cls, py_cmp: PyComparator) -> Comparator:
        match py_cmp:
            case PyComparator.Eq:
                return Comparator.EQ
            case PyComparator.Le:
                return Comparator.LE
            case PyComparator.Ge:
                return Comparator.GE
        raise RuntimeError("unknown sense")
