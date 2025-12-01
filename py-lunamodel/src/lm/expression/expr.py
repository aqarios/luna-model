from __future__ import annotations
from typing import TYPE_CHECKING

from lm._utils.wrap import wrap_env, wrap_var
from lm._core import PyExpression
from .iter import ExprIter

if TYPE_CHECKING:
    from lm import Environment
    from lm import Variable


class Expression:
    """
    todo
    """

    _expr: PyExpression

    # todo: we will need many more inits here, to also support the
    # creation of an expression from within a plugin. This also has to
    # be done via this method, at least allow the PyExpression as input.
    def __init__(self, data: PyExpression | Environment | None = None) -> None:
        if data is None:
            self._expr = PyExpression()
        elif isinstance(data, Environment):
            self._expr = PyExpression(data._env)
        elif isinstance(data, PyExpression):
            self._expr = data

    @classmethod
    def _from_pyexpr(cls, py_expr: PyExpression) -> Expression:
        expr = cls.__new__(cls)
        expr._expr = py_expr
        return expr

    @classmethod
    def const(cls, value: float, /, env: Environment | None = None) -> Expression:
        """ """
        return PyExpression.const(value, env)

    @property
    def environment(self) -> Environment:
        return wrap_env(self._expr.env)

    @property
    def num_variables(self) -> int:
        return self._expr.num_variables

    def get_offset(self) -> float:
        return self._expr.get_offset()

    def get_linear(self, /, variable: Variable) -> float:
        return self._expr.get_linear(variable._v)

    def get_quadratic(self, /, u: Variable, v: Variable) -> float:
        return self._expr.get_quadratic(u._v, v._v)

    def get_higher_order(self, /, *variables: tuple[Variable, ...]) -> float:
        return self._expr.get_higher_order(variables)

    def items(self) -> ExprIter:
        return self._expr.items()

    def variables(self) -> list[Variable]:
        return [wrap_var(v) for v in self._expr.variables()]
