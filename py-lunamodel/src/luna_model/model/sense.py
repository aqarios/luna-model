"""Optimization sense enumeration.

This module defines whether an optimization problem seeks to minimize
or maximize the objective function.
"""

from __future__ import annotations

from enum import Enum

from luna_model._lm import PySense


class Sense(Enum):
    """Optimization direction for the objective function.

    Specifies whether the goal is to minimize or maximize the objective.

    Attributes
    ----------
    MIN : str
        Minimize the objective function.
    MAX : str
        Maximize the objective function.

    Examples
    --------
    >>> from luna_model import Model, Sense
    >>> model = Model(sense=Sense.MIN)  # Minimization problem

    See Also
    --------
    Model : Model class that uses this sense.
    """

    MIN = "Minimize"
    MAX = "Maximize"

    @property
    def _val(self) -> PySense:
        match self:
            case Sense.MIN:
                return PySense.Min
            case Sense.MAX:
                return PySense.Max

    @classmethod
    def _from_pysense(cls, py_sense: PySense) -> Sense:
        match py_sense:
            case PySense.Min:
                return Sense.MIN
            case PySense.Max:
                return Sense.MAX
        msg = f"unknown sense: {py_sense}"
        raise RuntimeError(msg)
