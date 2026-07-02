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

# type: ignore[reportPossiblyUnboundVariable]
from typing import Protocol

from luna_model.constraint.cmp import Comparator
from luna_model.constraint.constr import Constraint
from luna_model.environment.env import Environment
from luna_model.errors import TranslationError
from luna_model.expression.expr import Expression
from luna_model.model.model import Model
from luna_model.model.sense import Sense
from luna_model.variable.bounds import Unbounded
from luna_model.variable.var import Variable
from luna_model.variable.vtype import Vtype

_OPT_MAPPER_AVAILABLE: bool = False
try:
    from qiskit_addon_opt_mapper import INFINITY, OptimizationProblem
    from qiskit_addon_opt_mapper.problems import ObjSense, VarType

    _OPT_MAPPER_AVAILABLE = True
except ImportError:
    _OPT_MAPPER_AVAILABLE = False


class _NamedSense(Protocol):
    name: str


class _OptMapperVariable(Protocol):
    name: str
    vartype: object
    lowerbound: float
    upperbound: float


class _OptMapperConstraint(Protocol):
    name: str
    sense: _NamedSense
    rhs: float


class QiskitOptMapperTranslator:
    r"""Translator for OptimizationProblem format.

    Converts between LunaModel and OptimizationProblem format.

    Requires the ``qiskit`` extra.

    Examples
    --------
    >>> from qiskit_addon_opt_mapper import OptimizationProblem
    >>> from luna_model.translator import QiskitOptMapperTranslator
    >>> mod = OptimizationProblem("my problem")
    >>> model = QiskitOptMapperTranslator.to_lm(mod)

    >>> from luna_model import Model
    >>> model = Model()
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> model.objective = -x - y + 2 * x * y
    >>> mod = QiskitOptMapperTranslator.from_lm(model)
    """

    @staticmethod
    def to_lm(mod: "OptimizationProblem", *, name: str | None = None) -> Model:
        """Convert Qiskit OptimizationProblem to LunaModel.

        Converts a Qiskit OptimizationProblem to a LunaModel Model.

        Parameters
        ----------
        mod : OptimizationProblem
            Qiskit OptimizationProblem to convert. All variable names must be strings.
        name : str
            Name for the resulting model. If None, uses the OptimizationProblem's name.

        Returns
        -------
        Model
            LunaModel representation with variables, objective and constraints matching the OptimizationProblem.

        Raises
        ------
        RuntimeError
            If ``qiskit-addon-opt-mapper`` package is not installed.
        TypeError
            If ``mod`` is not a OptimizationProblem.

        Examples
        --------
        >>> from qiskit_addon_opt_mapper import OptimizationProblem
        >>> from luna_model.translator import QiskitOptMapperTranslator
        >>> mod = OptimizationProblem("my model")
        >>> x = mod.binary_var("x")
        >>> y = mod.binary_var("y")
        >>> mod.minimize(constant=0.5, linear={"x": -1, "y": 2}, quadratic={("x", "y"): 1})
        >>> model = QiskitOptMapperTranslator.to_lm(mod)
        >>> print(model.objective)
        x y - x + 2 y + 0.5
        """
        if not _OPT_MAPPER_AVAILABLE:
            msg = (
                "qiskit-addon-opt-mapper is required for the QiskitOptMapperTranslator. "
                "You can install it using the 'qiskit' extra."
            )
            raise RuntimeError(msg)
        if not isinstance(mod, OptimizationProblem):
            msg = f"Expected mod to be of type OptimizationProblem, received: {type(mod)}"
            raise TypeError(msg)

        sense = Sense.MAX if mod.objective.sense == ObjSense.MAXIMIZE else Sense.MIN
        model = Model(name=name if name is not None else mod.name, sense=sense)

        for var in mod.variables:
            _add_lm_variable(model, var)

        model.objective = _expr_from_coefs(
            model.environment,
            constant=mod.objective.constant,
            linear=mod.objective.linear.to_dict(use_name=True),
            quadratic=mod.objective.quadratic.to_dict(use_name=True),
            higher_order={order: expr.to_dict(use_name=True) for order, expr in mod.objective.higher_order.items()},
        )

        _add_lm_constraints(model, mod)

        return model

    @staticmethod
    def from_lm(model: Model) -> "OptimizationProblem":
        """Convert LunaModel to Qiskit OptimizationProblem.

        Converts a LunaModel Model to a Qiskit OptimizationProblem.

        Parameters
        ----------
        model : Model
            The model to convert.

        Returns
        -------
        OptimizationProblem
            The Qiskit OptimizationProblem.

        Raises
        ------
        RuntimeError
            If ``qiskit-addon-opt-mapper`` package is not installed.
        TranslationError
            If the model contains constructs not supported by the
            OptimizationProblem format, e.g. inverted binary variables.

        Examples
        --------
        >>> from luna_model import Model
        >>> from luna_model.translator import QiskitOptMapperTranslator
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> model.objective = x * y - 2 * x + y
        >>> mod = QiskitOptMapperTranslator.from_lm(model)
        """
        if not _OPT_MAPPER_AVAILABLE:
            msg = (
                "qiskit-addon-opt-mapper is required for the QiskitOptMapperTranslator. "
                "You can install it using the 'qiskit' extra."
            )
            raise RuntimeError(msg)

        mod = OptimizationProblem(model.name)

        for var in model.variables():
            _add_optmapper_variable(mod, var)

        constant, linear, quadratic, higher_order = _expr_to_coeffs(model.objective)
        if model.sense == Sense.MAX:
            mod.maximize(constant, linear, quadratic, higher_order or None)
        else:
            mod.minimize(constant, linear, quadratic, higher_order or None)

        for cname, constr in model.constraints:
            _add_optmapper_constraint(mod, cname, constr)

        return mod


def _add_lm_variable(model: Model, var: _OptMapperVariable) -> None:
    match var.vartype:
        case VarType.BINARY:
            model.add_variable(var.name, vtype=Vtype.BINARY)
        case VarType.SPIN:
            model.add_variable(var.name, vtype=Vtype.SPIN)
        case VarType.INTEGER:
            model.add_variable(
                var.name,
                vtype=Vtype.INTEGER,
                lower=_to_bound(var.lowerbound),
                upper=_to_bound(var.upperbound),
            )
        case VarType.CONTINUOUS:
            model.add_variable(
                var.name,
                vtype=Vtype.REAL,
                lower=_to_bound(var.lowerbound),
                upper=_to_bound(var.upperbound),
            )
        case _:
            msg = f"unsupported variable type: {var.vartype}"
            raise TranslationError(msg)


def _add_lm_constraints(model: Model, mod: "OptimizationProblem") -> None:
    for constr in mod.linear_constraints:
        _add_lm_constraint(
            model,
            constr,
            linear=constr.linear.to_dict(use_name=True),
            quadratic={},
            higher_order={},
        )

    for constr in mod.quadratic_constraints:
        _add_lm_constraint(
            model,
            constr,
            linear=constr.linear.to_dict(use_name=True),
            quadratic=constr.quadratic.to_dict(use_name=True),
            higher_order={},
        )

    for constr in mod.higher_order_constraints:
        _add_lm_constraint(
            model,
            constr,
            linear=constr.linear.to_dict(use_name=True),
            quadratic=constr.quadratic.to_dict(use_name=True),
            higher_order={order: expr.to_dict(use_name=True) for order, expr in constr.higher_order.items()},
        )


def _add_lm_constraint(
    model: Model,
    constr: _OptMapperConstraint,
    linear: dict[str, float],
    quadratic: dict[tuple[str, str], float],
    higher_order: dict[int, dict[tuple[str, ...], float]],
) -> None:
    lhs = _expr_from_coefs(
        model.environment,
        constant=0.0,
        linear=linear,
        quadratic=quadratic,
        higher_order=higher_order,
    )
    cmp = Comparator(constr.sense.name.title())
    model.add_constraint(_build_constr(lhs, cmp, constr.rhs), name=constr.name)


def _add_optmapper_variable(mod: "OptimizationProblem", var: Variable) -> None:
    match var.vtype:
        case Vtype.BINARY:
            mod.binary_var(var.name)
        case Vtype.SPIN:
            mod.spin_var(var.name)
        case Vtype.INTEGER:
            lower, upper = _from_bounds(var)
            mod.integer_var(lower, upper, var.name)
        case Vtype.REAL:
            lower, upper = _from_bounds(var)
            mod.continuous_var(lower, upper, var.name)
        case _:
            msg = f"variable type not supported by OptimizationProblem: {var.vtype}"
            raise TranslationError(msg)


def _add_optmapper_constraint(mod: "OptimizationProblem", cname: str, constr: Constraint) -> None:
    lhs_const, linear, quadratic, higher_order = _expr_to_coeffs(constr.lhs)
    cmp = _CMP_TO_SENSE[constr.comparator]
    rhs = constr.rhs - lhs_const
    if higher_order:
        mod.higher_order_constraint(linear, quadratic, higher_order, cmp, rhs, cname)
    elif quadratic:
        mod.quadratic_constraint(linear, quadratic, cmp, rhs, cname)
    else:
        mod.linear_constraint(linear, cmp, rhs, cname)


def _to_bound(bound: float) -> float | type[Unbounded]:
    if bound <= -INFINITY or bound >= INFINITY:
        return Unbounded
    return bound


def _from_bounds(var: Variable) -> tuple[float, float]:
    lower = var.bounds.lower
    upper = var.bounds.upper
    return (
        -INFINITY if lower is Unbounded else lower,
        INFINITY if upper is Unbounded else upper,
    )


def _expr_from_coefs(
    env: Environment,
    constant: float,
    linear: dict[str, float],
    quadratic: dict[tuple[str, str], float],
    higher_order: dict[int, dict[tuple[str, ...], float]],
) -> Expression:
    expr = Expression.const(constant, env=env)
    for vname, bias in linear.items():
        expr += bias * env.get_variable(vname)
    for (vname, uname), bias in quadratic.items():
        expr += bias * env.get_variable(vname) * env.get_variable(uname)
    for coeffs in higher_order.values():
        for vnames, bias in coeffs.items():
            term = Expression.const(bias, env=env)
            for vname in vnames:
                term *= env.get_variable(vname)
            expr += term
    return expr


def _expr_to_coeffs(
    expression: Expression,
) -> tuple[float, dict[str, float], dict[tuple[str, str], float], dict[int, dict[tuple[str, ...], float]]]:
    constant = expression.get_offset()
    linear = {var.name: coeff for var, coeff in expression.linear_items()}
    quadratic = {(u.name, v.name): coeff for u, v, coeff in expression.quadratic_items()}
    higher_order: dict[int, dict[tuple[str, ...], float]] = {}
    for variables, coeff in expression.higher_order_items():
        order = len(variables)
        higher_order.setdefault(order, {})[tuple(var.name for var in variables)] = coeff
    return constant, linear, quadratic, higher_order


def _build_constr(lhs: Expression, comparator: Comparator, rhs: float) -> Constraint:
    match comparator:
        case Comparator.EQ:
            return lhs == rhs
        case Comparator.LE:
            return lhs <= rhs
        case Comparator.GE:
            return lhs >= rhs


_CMP_TO_SENSE: dict[Comparator, str] = {
    Comparator.EQ: "==",
    Comparator.LE: "<=",
    Comparator.GE: ">=",
}
