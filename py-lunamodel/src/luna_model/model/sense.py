from __future__ import annotations
from enum import Enum

from luna_model._lm import PySense


class Sense(Enum):
    """
    Enumeration of optimization senses supported by the optimization system.

    This enum defines the type of optimization used for a model. The type influences
    the domain and behavior of the model during optimization.
    """

    MIN = PySense.Min
    """Indicate the objective function to be minimized."""

    MAX = PySense.Max
    """Indicate the objective function to be maximized."""

    # below is to be deprecated

    Min = PySense.Min
    """Indicate the objective function to be minimized."""

    Max = PySense.Max
    """Indicate the objective function to be maximized."""
