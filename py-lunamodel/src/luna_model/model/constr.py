from __future__ import annotations
from enum import Enum

from luna_model._lm import PyConstraintType


class ConstraintType(Enum):
    """
    Enumeration of constraint types supported by the optimization system.

    This enum defines the type of constraint used within a model.
    """

    UNCONSTRAINED = PyConstraintType.Unconstrained
    """The model contains no constraints, i.e., is unconstrained."""

    EQUALITY = PyConstraintType.Equality
    """The model contains equality constraints (`Comparator.Eq`)."""

    INEQUALITY = PyConstraintType.Inequality
    """The model contains inequality constraints (`Comparator.Le`, `Comparator.Ge`).

    implicitly includes the `ConstraintType.LessEqual` and `ConstraintType.GreaterEqual`
    options.
    """

    LESS_EQUAL = PyConstraintType.LessEqual
    """The model contains less-equal-inequality constraints (`Comparator.Le`)."""

    GREATER_EQUAL = PyConstraintType.GreaterEqual
    """The model contains greater-equal-inequality constraints (`Comparator.Ge`)."""

    # below is to be deprecated

    Unconstrained = PyConstraintType.Unconstrained
    """The model contains no constraints, i.e., is unconstrained."""

    Equality = PyConstraintType.Equality
    """The model contains equality constraints (`Comparator.Eq`)."""

    Inequality = PyConstraintType.Inequality
    """The model contains inequality constraints (`Comparator.Le`, `Comparator.Ge`).

    implicitly includes the `ConstraintType.LessEqual` and `ConstraintType.GreaterEqual`
    options.
    """

    LessEqual = PyConstraintType.LessEqual
    """The model contains less-equal-inequality constraints (`Comparator.Le`)."""

    GreaterEqual = PyConstraintType.GreaterEqual
    """The model contains greater-equal-inequality constraints (`Comparator.Ge`)."""
