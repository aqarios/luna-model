from __future__ import annotations
from typing import TYPE_CHECKING, TypeAlias

from luna_model._utils import wrap_var
from luna_model._lm import (
    PyConstant,
    PyLinear,
    PyQuadratic,
    PyHigherOrder,
    PyExpressionIterator,
)

if TYPE_CHECKING:
    from luna_model.variable.var import Variable


Constant: TypeAlias = PyConstant


class Linear:
    _l: PyLinear

    __match_args__ = ("var",)

    @property
    def var(self) -> Variable:
        return wrap_var(self._l.var)

    @classmethod
    def _from_pyl(cls, py_l: PyLinear) -> Linear:
        """Construct LunaModel Linear from FFI PyLinear object."""
        lin = cls.__new__(cls)
        lin._l = py_l
        return lin


class Quadratic:
    _q: PyQuadratic

    __match_args__ = ("var_a", "var_b")

    @property
    def var_a(self) -> Variable:
        return wrap_var(self._q.var_a)

    @property
    def var_b(self) -> Variable:
        return wrap_var(self._q.var_b)

    @classmethod
    def _from_pyq(cls, py_q: PyQuadratic) -> Quadratic:
        """Construct LunaModel Quadratic from FFI PyQuadratic object."""
        q = cls.__new__(cls)
        q._q = py_q
        return q


class HigherOrder:
    _h: PyHigherOrder

    __match_args__ = ("vars",)

    @property
    def vars(self) -> list[Variable]:
        return [wrap_var(v) for v in self._h.vars]

    @classmethod
    def _from_pyh(cls, py_h: PyHigherOrder) -> HigherOrder:
        """Construct LunaModel HigherOrder from FFI PyHigherOrder object."""
        h = cls.__new__(cls)
        h._h = py_h
        return h


class ExprIter:
    _i: PyExpressionIterator

    def __next__(self) -> tuple[Constant | Linear | Quadratic | HigherOrder, float]:
        nxt, b = self._i.__next__()
        match nxt:
            case PyLinear(_):
                return Linear._from_pyl(nxt), b
            case PyQuadratic(_):
                return Quadratic._from_pyq(nxt), b
            case PyHigherOrder(_):
                return HigherOrder._from_pyh(nxt), b
            case PyConstant():
                return nxt, b
        raise RuntimeError(f"unknown element type: '{type(nxt)}'")

    def __iter__(self) -> ExprIter:
        return self

    @classmethod
    def _from_pyei(cls, py_ei: PyExpressionIterator) -> ExprIter:
        """Construct LunaModel ExprIter from FFI PyExpressionIterator object."""
        i = cls.__new__(cls)
        i._i = py_ei
        return i
