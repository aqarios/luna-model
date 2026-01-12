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

    # todo: deprecate the below

    Binary = "Binary"
    InvertedBinary = "InvertedBinary"
    Spin = "Spin"
    Integer = "Integer"
    Real = "Real"

    @property
    def name(self) -> str:
        match self:
            case Vtype.BINARY | Vtype.Binary:
                return "Binary"
            case Vtype.INVERTED_BINARY | Vtype.InvertedBinary:
                return "InvertedBinary"
            case Vtype.SPIN | Vtype.Spin:
                return "Spin"
            case Vtype.INTEGER | Vtype.Integer:
                return "Integer"
            case Vtype.REAL | Vtype.Real:
                return "Real"

    @property
    def _val(self) -> PyVtype:
        match self:
            case Vtype.BINARY | Vtype.Binary:
                return PyVtype.Binary
            case Vtype.INVERTED_BINARY | Vtype.InvertedBinary:
                return PyVtype.InvertedBinary
            case Vtype.SPIN | Vtype.Spin:
                return PyVtype.Spin
            case Vtype.INTEGER | Vtype.Integer:
                return PyVtype.Integer
            case Vtype.REAL | Vtype.Real:
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
