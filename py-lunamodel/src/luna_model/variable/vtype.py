from __future__ import annotations
from enum import Enum

from luna_model._lm import PyVtype


class Vtype(Enum):
    BINARY = "Binary"
    # todo: have this only be part of a specialization? So that it can be returned but not used
    # directly? I.e., forbid generation of a inverted binary variable by passing this vtype to
    # the variable's constructor?
    INVERTED_BINARY = "InvertedBinary"
    SPIN = "Spin"
    INTEGER = "Integer"
    REAL = "Real"

    @property
    def _val(self) -> PyVtype:
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
        raise RuntimeError("unknown sense")
