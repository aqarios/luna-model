from __future__ import annotations
from luna_model._lm import PyLogElement


class LogElement:
    _le: PyLogElement

    @classmethod
    def _from_pyle(cls, py_le: PyLogElement) -> LogElement:
        le = cls.__new__(cls)
        le._le = py_le
        return le
