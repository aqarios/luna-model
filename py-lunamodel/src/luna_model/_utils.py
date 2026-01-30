from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from luna_model import (
        Bounds,
        Constraint,
        ConstraintCollection,
        Environment,
        Expression,
        Model,
        ModelSpecs,
        Solution,
        Timer,
        Variable,
    )
    from luna_model._lm import (
        PyBounds,
        PyConstraint,
        PyConstraintCollection,
        PyEnvironment,
        PyExpression,
        PyModel,
        PyModelSpecs,
        PySolution,
        PyTimer,
        PyVariable,
    )


def wrap_var(py_var: PyVariable) -> Variable:
    from luna_model.variable.var import Variable  # noqa: PLC0415

    return Variable._from_pyvar(py_var)


def wrap_b(py_b: PyBounds) -> Bounds:
    from luna_model.variable.bounds import Bounds  # noqa: PLC0415

    return Bounds._from_pyb(py_b)


def wrap_s(py_s: PySolution) -> Solution:
    from luna_model.solution.sol import Solution  # noqa: PLC0415

    return Solution._from_pys(py_s)


def wrap_env(py_env: PyEnvironment) -> Environment:
    from luna_model.environment.env import Environment  # noqa: PLC0415

    return Environment._from_pyenv(py_env)


def wrap_c(py_c: PyConstraint) -> Constraint:
    from luna_model.constraint.constr import Constraint  # noqa: PLC0415

    return Constraint._from_pyc(py_c)


def wrap_cc(py_cc: PyConstraintCollection) -> ConstraintCollection:
    from luna_model.constraint.collection import ConstraintCollection  # noqa: PLC0415

    return ConstraintCollection._from_pycc(py_cc)


def wrap_sp(py_sp: PyModelSpecs) -> ModelSpecs:
    from luna_model.model.specs import ModelSpecs  # noqa: PLC0415

    return ModelSpecs._from_pysp(py_sp)


def wrap_t(py_t: PyTimer) -> Timer:
    from luna_model.timer import Timer  # noqa: PLC0415

    return Timer._from_pyt(py_t)


def wrap_m(py_m: PyModel) -> Model:
    from luna_model.model.model import Model  # noqa: PLC0415

    return Model._from_pym(py_m)


def wrap_expr(py_expr: PyExpression) -> Expression:
    from luna_model.expression.expr import Expression  # noqa: PLC0415

    return Expression._from_pyexpr(py_expr)
