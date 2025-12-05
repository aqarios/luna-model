from __future__ import annotations
from typing import TYPE_CHECKING


import luna_model._reexport as lm
from luna_model._lm import (
    PyEnvironment,
    PyVariable,
    PyBounds,
    PySolution,
    PyConstraint,
    PyConstraintCollection,
    PyModelSpecs,
    PyTimer,
    PyModel,
    PyExpression,
)

if TYPE_CHECKING:
    from luna_model import (
        Environment,
        Variable,
        Bounds,
        Solution,
        Constraint,
        ConstraintCollection,
        ModelSpecs,
        Timer,
        Model,
        Expression,
    )


def wrap_var(py_var: PyVariable) -> Variable:
    return lm.v.Variable._from_pyvar(py_var)  # type: ignore[attribute]


def wrap_b(py_b: PyBounds) -> Bounds:
    return lm.b.Bounds._from_pyb(py_b)  # type: ignore[attribute]


def wrap_s(py_s: PySolution) -> Solution:
    return lm.s.Solution._from_pys(py_s)  # type: ignore[attribute]


def wrap_env(py_env: PyEnvironment) -> Environment:
    return lm.ev.Environment._from_pyenv(py_env)  # type: ignore[attribute]


def wrap_c(py_c: PyConstraint) -> Constraint:
    return lm.c.Constraint._from_pyc(py_c)  # type: ignore[attribute]


def wrap_cc(py_cc: PyConstraintCollection) -> ConstraintCollection:
    return lm.cc.ConstraintCollection._from_pycc(py_cc)  # type: ignore[attribute]


def wrap_sp(py_sp: PyModelSpecs) -> ModelSpecs:
    return lm.ms.ModelSpecs._from_pysp(py_sp)  # type: ignore[attribute]


def wrap_t(py_t: PyTimer) -> Timer:
    return lm.t.Timer._from_pyt(py_t)  # type: ignore[attribute]


def wrap_m(py_m: PyModel) -> Model:
    return lm.m.Model._from_pym(py_m)  # type: ignore[attribute]


def wrap_expr(py_expr: PyExpression) -> Expression:
    return lm.e.Expression._from_pyexpr(py_expr)  # type: ignore[attribute]
