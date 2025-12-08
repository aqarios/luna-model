from __future__ import annotations
from typing import TYPE_CHECKING


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
    from luna_model.variable.var import Variable
    return Variable._from_pyvar(py_var)  # type: ignore[attribute]


def wrap_b(py_b: PyBounds) -> Bounds:
    from luna_model.variable.bounds import Bounds
    return Bounds._from_pyb(py_b)  # type: ignore[attribute]


def wrap_s(py_s: PySolution) -> Solution:
    from luna_model.solution.sol import Solution
    return Solution._from_pys(py_s)  # type: ignore[attribute]


def wrap_env(py_env: PyEnvironment) -> Environment:
    from luna_model.environment.environment import Environment
    return Environment._from_pyenv(py_env)  # type: ignore[attribute]


def wrap_c(py_c: PyConstraint) -> Constraint:
    from luna_model.constraint.constr import Constraint
    return Constraint._from_pyc(py_c)  # type: ignore[attribute]


def wrap_cc(py_cc: PyConstraintCollection) -> ConstraintCollection:
    from luna_model.constraint.collection import ConstraintCollection
    return ConstraintCollection._from_pycc(py_cc)  # type: ignore[attribute]


def wrap_sp(py_sp: PyModelSpecs) -> ModelSpecs:
    from luna_model.model.specs import ModelSpecs
    return ModelSpecs._from_pysp(py_sp)  # type: ignore[attribute]


def wrap_t(py_t: PyTimer) -> Timer:
    from luna_model.solution.timer import Timer
    return Timer._from_pyt(py_t)  # type: ignore[attribute]


def wrap_m(py_m: PyModel) -> Model:
    from luna_model.model.model import Model
    return Model._from_pym(py_m)  # type: ignore[attribute]


def wrap_expr(py_expr: PyExpression) -> Expression:
    from luna_model.expression.expr import Expression
    return Expression._from_pyexpr(py_expr)  # type: ignore[attribute]
