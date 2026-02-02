"""Constraints for optimization models.

This module provides classes for creating and managing constraints in
optimization problems. Constraints specify relationships that must be
satisfied by the solution.

Key classes:
    - Constraint: Individual constraint with lhs, comparator, and rhs
    - ConstraintCollection: Container for multiple named constraints
    - Comparator: Comparison operators (EQ, LE, GE)
    - ConstraintCollectionIter: Iterator for constraint collections
"""

from luna_model.constraint.cmp import Comparator
from luna_model.constraint.collection import ConstraintCollection
from luna_model.constraint.constr import Constraint
from luna_model.constraint.iter import ConstraintCollectionIter

__all__ = [
    "Comparator",
    "Constraint",
    "ConstraintCollection",
    "ConstraintCollectionIter",
]
