from __future__ import annotations

from enum import Enum

from luna_model._lm import PyComparator


class Comparator(Enum):
    """Comparator emum."""

    EQ = "Eq"
    LE = "Le"
    GE = "Ge"

    @property
    def _val(self) -> PyComparator:
        match self:
            case Comparator.EQ:
                return PyComparator.Eq
            case Comparator.LE:
                return PyComparator.Le
            case Comparator.GE:
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
        msg = f"unknown comparator '{py_cmp}'"
        raise RuntimeError(msg)
