from __future__ import annotations

from typing import Any as PyConstraint
# from luna_model._lm import PyConstraint


class Constraint:
    """ """

    _c: PyConstraint

    @classmethod
    def _from_pyc(cls, py_c: PyConstraint) -> Constraint:
        """Construct LunaModel Constraint from FFI PyConstraint object."""
        c = cls.__new__(cls)
        c._c = py_c
        return c
