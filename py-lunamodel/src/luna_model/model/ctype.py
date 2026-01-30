from __future__ import annotations

from enum import Enum

from luna_model._lm import PyCtype


class Ctype(Enum):
    """Constrained types."""

    UNCONSTRAINED = "Unconstrained"
    EQUALITY = "Equality"
    INEQUALITY = "Inequality"
    LESS_EQUAL = "LessEqual"
    GREATER_EQUAL = "GreaterEqual"

    @property
    def _val(self) -> PyCtype:
        match self:
            case Ctype.UNCONSTRAINED:
                return PyCtype.Unconstrained
            case Ctype.EQUALITY:
                return PyCtype.Equality
            case Ctype.INEQUALITY:
                return PyCtype.Inequality
            case Ctype.LESS_EQUAL:
                return PyCtype.LessEqual
            case Ctype.GREATER_EQUAL:
                return PyCtype.GreaterEqual

    @classmethod
    def _from_pyctype(cls, py_ctype: PyCtype) -> Ctype:
        match py_ctype:
            case PyCtype.Unconstrained:
                return Ctype.UNCONSTRAINED
            case PyCtype.Equality:
                return Ctype.EQUALITY
            case PyCtype.Inequality:
                return Ctype.INEQUALITY
            case PyCtype.LessEqual:
                return Ctype.LESS_EQUAL
            case PyCtype.GreaterEqual:
                return Ctype.GREATER_EQUAL
        msg = f"unknown ctype: {py_ctype}"
        raise RuntimeError(msg)
