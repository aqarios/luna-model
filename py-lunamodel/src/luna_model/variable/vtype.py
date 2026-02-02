"""Variable type enumeration for optimization variables.

This module defines the types of variables that can be used in optimization
models. Each type determines the domain of values the variable can take.
"""

from __future__ import annotations

from enum import Enum

from luna_model._lm import PyVtype


class Vtype(Enum):
    """Enumeration of variable types for optimization models.

    The variable type determines the domain of values a variable can take
    during optimization. Different problem types and solvers support different
    variable types.

    Attributes
    ----------
    BINARY : str
        Binary variable that can be 0 or 1. Used for yes/no decisions.
    INVERTED_BINARY : str
        Inverted binary variable. Internal representation that maps 0→1 and 1→0.
        Not typically used directly by users.
    SPIN : str
        Spin variable that can be -1 or +1. Common in quantum computing formulations.
    INTEGER : str
        Integer variable that can be any integer value within bounds.
    REAL : str
        Real-valued (continuous) variable that can be any floating-point value within bounds.

    Examples
    --------
    Create different types of variables:

    >>> from luna_model import Variable, Vtype
    >>> x = Variable("x", vtype=Vtype.BINARY)
    >>> y = Variable("y", vtype=Vtype.INTEGER, bounds=(0, 10))
    >>> z = Variable("z", vtype=Vtype.REAL, bounds=(0.0, 1.0))

    Notes
    -----
    The default variable type is ``BINARY`` when not specified.

    See Also
    --------
    Variable : Decision variable class that uses this type enumeration.
    Bounds : Bounds class for constraining variable values.
    """

    BINARY = "Binary"
    INVERTED_BINARY = "InvertedBinary"
    SPIN = "Spin"
    INTEGER = "Integer"
    REAL = "Real"

    @property
    def _val(self) -> PyVtype:
        """Convert Python Vtype to internal PyVtype representation."""
        match self:
            case Vtype.BINARY:
                return PyVtype.Binary
            case Vtype.INVERTED_BINARY:
                return PyVtype.InvertedBinary
            case Vtype.SPIN:
                return PyVtype.Spin
            case Vtype.INTEGER:
                return PyVtype.Integer
            case Vtype.REAL:
                return PyVtype.Real

    @classmethod
    def _from_pyvtype(cls, py_vtype: PyVtype) -> Vtype:
        """Convert internal PyVtype representation to Python Vtype."""
        match py_vtype:
            case PyVtype.Binary:
                return Vtype.BINARY
            case PyVtype.InvertedBinary:
                return Vtype.INVERTED_BINARY
            case PyVtype.Spin:
                return Vtype.SPIN
            case PyVtype.Integer:
                return Vtype.INTEGER
            case PyVtype.Real:
                return Vtype.REAL
        msg = f"unknown vtype '{py_vtype}'"
        raise RuntimeError(msg)
