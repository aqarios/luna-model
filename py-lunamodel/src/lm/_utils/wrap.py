from __future__ import annotations

from typing import TYPE_CHECKING

import lm._reexport as lm

if TYPE_CHECKING:
    from lm import Expression, Environment, Variable
    from lm._core import PyExpression, PyEnvironment, PyVariable


def wrap_expr(expr: PyExpression) -> Expression:
    return lm.Expression._from_pyexpr(expr)


def wrap_env(env: PyEnvironment) -> Environment:
    return lm.Environment._from_pyenv(env)


def wrap_var(var: PyVariable) -> Variable:
    return lm.Variable._from_pyvar(var)
