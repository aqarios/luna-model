"""Constraint type enumeration.

This module defines the types of constraints that can appear in optimization
models, indicating the nature of constraint satisfaction requirements.
"""

from __future__ import annotations

from enum import Enum

from luna_model._lm import PyCtype


class Ctype(Enum):
    """Types of constraints in optimization models.

    Categorizes constraints by their comparison operators and structure.

    Attributes
    ----------
    UNCONSTRAINED : str
        No constraints (unconstrained optimization).
    EQUALITY : str
        Equality constraints (==).
    INEQUALITY : str
        Inequality constraints (<= or >=).
    LESS_EQUAL : str
        Less-than-or-equal constraints (<=).
    GREATER_EQUAL : str
        Greater-than-or-equal constraints (>=).

    Examples
    --------
    >>> from luna_model import Ctype, ModelSpecs
    >>> specs = ModelSpecs(constraints={Ctype.LESS_EQUAL})

    See Also
    --------
    ModelSpecs : Model specifications that use constraint types.
    Comparator : Comparator enum for individual constraints.
    """

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
