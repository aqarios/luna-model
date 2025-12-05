from __future__ import annotations
from enum import Enum

from luna_model._lm import PySense


class Sense(Enum):
    MIN = PySense.Min
    MAX = PySense.Max

    # below is to be deprecated

    Min = PySense.Min
    Max = PySense.Max
