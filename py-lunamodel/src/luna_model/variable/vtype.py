from __future__ import annotations
from enum import Enum

from luna_model._lm import PyVtype


class Vtype(Enum):
    """
    Enumeration of variable types supported by the optimization system.

    This enum defines the type of a variable used in a model. The type influences
    the domain and behavior of the variable during optimization. It is often passed
    when defining variables to specify how they should behave.

    Attributes
    ----------
    REAL : Vtype
        Continuous real-valued variable. Can take any value within given bounds.
    INTEGER : Vtype
        Discrete integer-valued variable. Takes integer values within bounds.
    BINARY : Vtype
        Binary variable. Can only take values 0 or 1.
    SPIN : Vtype
        Spin variable. Can only take values -1 or +1.

    Examples
    --------
    >>> from luna_model import Vtype
    >>> Vtype.REAL
    Real

    >>> str(Vtype.BINARY)
    'Binary'

    Notes
    -----
    Older versions of the Vtype enum used PascalCase naming for the enum members.
    This is still possible, but support for this will be removed in a future version.
    """

    BINARY = PyVtype.Binary
    """Binary variable. Can only take values 0 or 1."""
    # todo: have this only be part of a specialization? So that it can be returned but not used
    # directly? I.e., forbid generation of a inverted binary variable by passing this vtype to
    # the variable's constructor?
    INVERTED_BINARY = PyVtype.InvertedBinary
    """Inverted binary variable. Can only take values 0 or 1. Always corresponds to another Binary variable"""
    SPIN = PyVtype.Spin
    """Spin variable. Can only take values -1 or +1."""
    INTEGER = PyVtype.Integer
    """Discrete integer-valued variable. Takes integer values within bounds."""
    REAL = PyVtype.Real
    """Continuous real-valued variable. Can take any value within given bounds."""

    Binary = PyVtype.Binary
    """Binary variable. Can only take values 0 or 1."""
    InvertedBinary = PyVtype.InvertedBinary
    """Inverted binary variable. Can only take values 0 or 1. Always corresponds to another Binary variable"""
    Spin = PyVtype.Spin
    """Spin variable. Can only take values -1 or +1."""
    Integer = PyVtype.Integer
    """Discrete integer-valued variable. Takes integer values within bounds."""
    Real = PyVtype.Real
    """Continuous real-valued variable. Can take any value within given bounds."""
