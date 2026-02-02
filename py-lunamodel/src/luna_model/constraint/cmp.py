"""Comparator operators for constraints.

This module defines the comparison operators used in optimization constraints
to specify the relationship between the left-hand and right-hand sides.
"""

from __future__ import annotations

from enum import Enum

from luna_model._lm import PyComparator


class Comparator(Enum):
    """Comparison operators for constraints.

    Defines the type of comparison used in a constraint between the left-hand
    side expression and the right-hand side value.

    Attributes
    ----------
    EQ : str
        Equality constraint (==). Requires lhs to equal rhs.
    LE : str
        Less-than-or-equal constraint (<=). Requires lhs to be at most rhs.
    GE : str
        Greater-than-or-equal constraint (>=). Requires lhs to be at least rhs.

    Examples
    --------
    Comparators are typically created automatically through operator overloading:

    >>> from luna_model import Variable
    >>> x = Variable("x")
    >>> c1 = x == 1      # Creates EQ constraint
    >>> c2 = x <= 5      # Creates LE constraint
    >>> c3 = x >= 0      # Creates GE constraint

    Notes
    -----
    Strict inequality (<, >) is not supported in optimization constraints.

    See Also
    --------
    Constraint : Constraint class that uses these comparators.
    """

    EQ = "Eq"
    LE = "Le"
    GE = "Ge"

    @property
    def _val(self) -> PyComparator:
        """Convert Python Comparator to internal PyComparator representation."""
        match self:
            case Comparator.EQ:
                return PyComparator.Eq
            case Comparator.LE:
                return PyComparator.Le
            case Comparator.GE:
                return PyComparator.Ge

    @classmethod
    def _from_pycmp(cls, py_cmp: PyComparator) -> Comparator:
        """Convert internal PyComparator representation to Python Comparator."""
        match py_cmp:
            case PyComparator.Eq:
                return Comparator.EQ
            case PyComparator.Le:
                return Comparator.LE
            case PyComparator.Ge:
                return Comparator.GE
        msg = f"unknown comparator '{py_cmp}'"
        raise RuntimeError(msg)
