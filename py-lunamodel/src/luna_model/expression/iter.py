# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from __future__ import annotations

from typing import TYPE_CHECKING, TypeAlias

from luna_model._lm import (
    PyConstant,
    PyExpressionIterator,
    PyHigherOrder,
    PyLinear,
    PyQuadratic,
)
from luna_model._utils import wrap_var

if TYPE_CHECKING:
    from luna_model.variable.var import Variable


Constant: TypeAlias = PyConstant
"""Constant term in an expression.

Represents a term of the form: coefficient
"""


class Linear:
    """Linear term in an expression.

    Represents a term of the form: coefficient * variable

    Attributes
    ----------
    var : Variable
        The variable in this linear term.
    """

    _l: PyLinear

    __match_args__ = ("var",)

    @property
    def var(self) -> Variable:
        """Get the variable in this linear term.

        Returns
        -------
        Variable
            The variable.
        """
        return wrap_var(self._l.var)

    @classmethod
    def _from_pyl(cls, py_l: PyLinear) -> Linear:
        """Construct LunaModel Linear from FFI PyLinear object."""
        lin = cls.__new__(cls)
        lin._l = py_l
        return lin

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the linear term.
        """
        return self._l.__str__()


class Quadratic:
    """Quadratic term in an expression.

    Represents a term of the form: coefficient * var_a * var_b

    Attributes
    ----------
    var_a : Variable
        The first variable in the quadratic term.
    var_b : Variable
        The second variable in the quadratic term.
    """

    _q: PyQuadratic

    __match_args__ = ("var_a", "var_b")

    @property
    def var_a(self) -> Variable:
        """Get the first variable in the quadratic term.

        Returns
        -------
        Variable
            The first variable.
        """
        return wrap_var(self._q.var_a)

    @property
    def var_b(self) -> Variable:
        """Get the second variable in the quadratic term.

        Returns
        -------
        Variable
            The second variable.
        """
        return wrap_var(self._q.var_b)

    @classmethod
    def _from_pyq(cls, py_q: PyQuadratic) -> Quadratic:
        """Construct LunaModel Quadratic from FFI PyQuadratic object."""
        q = cls.__new__(cls)
        q._q = py_q
        return q

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the quadratic term.
        """
        return self._q.__str__()


class HigherOrder:
    """Higher-order term in an expression.

    Represents a term with degree > 2 of the form: coefficient * var1 * var2 * ...

    Attributes
    ----------
    vars : list[Variable]
        The list of variables in this higher-order term.
    """

    _h: PyHigherOrder

    __match_args__ = ("vars",)

    @property
    def vars(self) -> list[Variable]:
        """Get the variables in this higher-order term.

        Returns
        -------
        list[Variable]
            The list of variables.
        """
        return [wrap_var(v) for v in self._h.vars]

    @classmethod
    def _from_pyh(cls, py_h: PyHigherOrder) -> HigherOrder:
        """Construct LunaModel HigherOrder from FFI PyHigherOrder object."""
        h = cls.__new__(cls)
        h._h = py_h
        return h

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the higher-order term.
        """
        return self._h.__str__()


class ExprIter:
    """Iterator over terms in an expression.

    Iterates over all terms in an expression, yielding (term, coefficient)
    tuples where term is a Constant, Linear, Quadratic, or HigherOrder object.

    Examples
    --------
    >>> from luna_model import Variable, Environment
    >>> from luna_model import Constant, Linear, Quadratic, HigherOrder
    >>> with Environment():
    ...     x, y, z = Variable("x"), Variable("y"), Variable("z")
    ...     expr = 3 * x + 2 * x * y + 4 * x * y * z + 5
    ...     for term, coeff in expr.items():
    ...         match term:
    ...             case Constant():
    ...                 print(f"constant: {coeff}")
    ...             case Linear(var):
    ...                 print("linear:", coeff, var.name)
    ...             case Quadratic(var_a, var_b):
    ...                 print("quadratic:", coeff, var_a.name, var_b.name)
    ...             case HigherOrder(vars):
    ...                 print("higher-order:", coeff, *[v.name for v in vars])
    linear: 3.0 x
    quadratic: 2.0 x y
    higher-order: 4.0 x y z
    constant: 5.0
    """

    _i: PyExpressionIterator

    def __next__(self) -> tuple[Constant | Linear | Quadratic | HigherOrder, float]:
        """Get the next term and coefficient.

        Returns
        -------
        tuple[Constant or Linear or Quadratic or HigherOrder, float]
            The term and its coefficient.

        Raises
        ------
        StopIteration
            When there are no more terms.
        """
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
        msg = f"unknown element type: '{type(nxt)}'"
        raise RuntimeError(msg)

    def __iter__(self) -> ExprIter:
        """Return the iterator object itself."""
        return self

    @classmethod
    def _from_pyei(cls, py_ei: PyExpressionIterator) -> ExprIter:
        """Construct LunaModel ExprIter from FFI PyExpressionIterator object."""
        i = cls.__new__(cls)
        i._i = py_ei
        return i
