from __future__ import annotations
from enum import Enum

from luna_model._lm import PyCtype


class Ctype(Enum):
    UNCONSTRAINED = "Unconstrained"
    EQUALITY = "Equality"
    INEQUALITY = "Inequality"
    LESS_EQUAL = "LessEqual"
    GREATER_EQUAL = "GreaterEqual"

    # below is to be deprecated

    Unconstrained = "Unconstrained"
    Equality = "Equality"
    Inequality = "Inequality"
    LessEqual = "LessEqual"
    GreaterEqual = "GreaterEqual"

    @property
    def name(self) -> str:
        match self:
            case Ctype.UNCONSTRAINED | Ctype.Unconstrained:
                return "Unconstrained"
            case Ctype.EQUALITY | Ctype.Equality:
                return "Equality"
            case Ctype.INEQUALITY | Ctype.Inequality:
                return "Inequality"
            case Ctype.LESS_EQUAL | Ctype.LessEqual:
                return "LessEqual"
            case Ctype.GREATER_EQUAL | Ctype.GreaterEqual:
                return "GreaterEqual"

    @property
    def _val(self) -> PyCtype:
        match self:
            case Ctype.UNCONSTRAINED | Ctype.Unconstrained:
                return PyCtype.Unconstrained
            case Ctype.EQUALITY | Ctype.Equality:
                return PyCtype.Equality
            case Ctype.INEQUALITY | Ctype.Inequality:
                return PyCtype.Inequality
            case Ctype.LESS_EQUAL | Ctype.LessEqual:
                return PyCtype.LessEqual
            case Ctype.GREATER_EQUAL | Ctype.GreaterEqual:
                return PyCtype.GreaterEqual

    @classmethod
    def _from_pyctype(cls, py_vtype: PyCtype) -> Ctype:
        match py_vtype:
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
        raise RuntimeError("unknown sense")
