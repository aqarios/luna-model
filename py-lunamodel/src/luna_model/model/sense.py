from __future__ import annotations
from enum import Enum

from luna_model._lm import PySense


class Sense(Enum):
    MIN = "Minimize"
    MAX = "Maximize"

    # below to be deprecated

    Min = "Minimize"
    Max = "Maximize"

    @property
    def name(self) -> str:
        match self:
            case Sense.MIN | Sense.Min:
                return "Min"
            case Sense.MAX | Sense.Max:
                return "Max"

    @property
    def _val(self) -> PySense:
        match self:
            case Sense.MIN | Sense.Min:
                return PySense.Min
            case Sense.MAX | Sense.Max:
                return PySense.Max

    @classmethod
    def _from_pysense(cls, py_sense: PySense) -> Sense:
        match py_sense:
            case PySense.Min:
                return Sense.MIN
            case PySense.Max:
                return Sense.MAX
        raise RuntimeError("unknown sense")
