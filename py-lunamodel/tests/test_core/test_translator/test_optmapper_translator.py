from luna_model.ttarget import TranslationTarget
import pytest

qiskit_opt_mapper = pytest.importorskip("qiskit_addon_opt_mapper")
pytest.importorskip("qiskit_addon_opt_mapper.problems")

from luna_model import Comparator, Model, Sense, Unbounded, Vtype
from luna_model.expression import Expression
from luna_model.translator.model import QiskitOptMapperTranslator

OptimizationProblem = qiskit_opt_mapper.OptimizationProblem


def test_optmapper_to_lm_to_optmapper_roundtrip():
    mod = OptimizationProblem("optmapper_model")
    mod.binary_var("x")
    mod.spin_var("s")
    mod.integer_var(-2, 5, "i")
    mod.continuous_var(-1.5, 3.5, "r")
    mod.maximize(
        constant=1.25,
        linear={"x": 2.0, "i": -3.0},
        quadratic={("x", "i"): 4.0},
        higher_order={3: {("x", "i", "r"): -1.5}},
    )
    mod.linear_constraint({"x": 1.0, "i": 2.0}, "<=", 3.0, "lin")
    mod.quadratic_constraint({"r": 1.0}, {("x", "i"): 2.0}, ">=", -1.0, "quad")
    mod.higher_order_constraint(
        {"s": -1.0},
        {("x", "i"): 1.5},
        {3: {("x", "i", "r"): 2.5}},
        "==",
        4.0,
        "higher",
    )

    model = QiskitOptMapperTranslator.to_lm(mod)
    mod_back = QiskitOptMapperTranslator.from_lm(model)

    assert mod_back.name == mod.name
    assert mod_back.objective.sense == mod.objective.sense
    assert _optmapper_variables(mod_back) == _optmapper_variables(mod)
    assert _optmapper_objective(mod_back) == _optmapper_objective(mod)
    assert _optmapper_constraints(mod_back) == _optmapper_constraints(mod)


def test_lm_to_optmapper_to_lm_roundtrip():
    model = Model("luna_model", sense=Sense.MAX)
    x = model.add_variable("x", vtype=Vtype.BINARY)
    s = model.add_variable("s", vtype=Vtype.SPIN)
    i = model.add_variable("i", vtype=Vtype.INTEGER, lower=-2, upper=5)
    r = model.add_variable("r", vtype=Vtype.REAL, lower=-1.5, upper=3.5)
    model.objective = 1.25 + 2.0 * x - 3.0 * i + 4.0 * x * i - 1.5 * x * i * r
    model.add_constraint(x + 2.0 * i <= 3.0, name="lin")
    model.add_constraint(2.0 * x * i + r >= -1.0, name="quad")
    model.add_constraint(x * i * r - s == 4.0, name="higher")

    mod = QiskitOptMapperTranslator.from_lm(model)
    model_back = QiskitOptMapperTranslator.to_lm(mod)

    assert model_back.name == model.name
    assert model_back.sense == model.sense
    assert _lm_variables(model_back) == _lm_variables(model)
    assert _lm_expr(model_back.objective) == _lm_expr(model.objective)
    assert _lm_constraints(model_back) == _lm_constraints(model)

def test_optmapper_to_lm_to_optmapper_roundtrip_fromto():
    mod = OptimizationProblem("optmapper_model")
    mod.binary_var("x")
    mod.spin_var("s")
    mod.integer_var(-2, 5, "i")
    mod.continuous_var(-1.5, 3.5, "r")
    mod.maximize(
        constant=1.25,
        linear={"x": 2.0, "i": -3.0},
        quadratic={("x", "i"): 4.0},
        higher_order={3: {("x", "i", "r"): -1.5}},
    )
    mod.linear_constraint({"x": 1.0, "i": 2.0}, "<=", 3.0, "lin")
    mod.quadratic_constraint({"r": 1.0}, {("x", "i"): 2.0}, ">=", -1.0, "quad")
    mod.higher_order_constraint(
        {"s": -1.0},
        {("x", "i"): 1.5},
        {3: {("x", "i", "r"): 2.5}},
        "==",
        4.0,
        "higher",
    )

    model = Model.from_(mod)
    mod_back = model.to(TranslationTarget.OPT_MAPPER)

    assert mod_back.name == mod.name
    assert mod_back.objective.sense == mod.objective.sense
    assert _optmapper_variables(mod_back) == _optmapper_variables(mod)
    assert _optmapper_objective(mod_back) == _optmapper_objective(mod)
    assert _optmapper_constraints(mod_back) == _optmapper_constraints(mod)

def test_lm_to_optmapper_to_lm_roundtrip_fromto():
    model = Model("luna_model", sense=Sense.MAX)
    x = model.add_variable("x", vtype=Vtype.BINARY)
    s = model.add_variable("s", vtype=Vtype.SPIN)
    i = model.add_variable("i", vtype=Vtype.INTEGER, lower=-2, upper=5)
    r = model.add_variable("r", vtype=Vtype.REAL, lower=-1.5, upper=3.5)
    model.objective = 1.25 + 2.0 * x - 3.0 * i + 4.0 * x * i - 1.5 * x * i * r
    model.add_constraint(x + 2.0 * i <= 3.0, name="lin")
    model.add_constraint(2.0 * x * i + r >= -1.0, name="quad")
    model.add_constraint(x * i * r - s == 4.0, name="higher")

    mod = model.to(TranslationTarget.OPT_MAPPER)
    model_back = Model.from_(mod)

    assert model_back.name == model.name
    assert model_back.sense == model.sense
    assert _lm_variables(model_back) == _lm_variables(model)
    assert _lm_expr(model_back.objective) == _lm_expr(model.objective)
    assert _lm_constraints(model_back) == _lm_constraints(model)



def _optmapper_variables(mod: OptimizationProblem) -> dict[str, tuple[object, float, float]]:
    return {var.name: (var.vartype, var.lowerbound, var.upperbound) for var in mod.variables}


def _optmapper_objective(mod: OptimizationProblem) -> tuple[object, float, dict[str, float], dict[tuple[str, ...], float]]:
    return (
        mod.objective.sense,
        mod.objective.constant,
        mod.objective.linear.to_dict(use_name=True),
        _optmapper_terms(mod.objective.quadratic.to_dict(use_name=True), mod.objective.higher_order),
    )


def _optmapper_constraints(
    mod: OptimizationProblem,
) -> dict[str, tuple[str, float, dict[str, float], dict[tuple[str, ...], float]]]:
    constraints = {}
    for constr in mod.linear_constraints:
        constraints[constr.name] = (
            constr.sense.name,
            constr.rhs,
            constr.linear.to_dict(use_name=True),
            {},
        )
    for constr in mod.quadratic_constraints:
        constraints[constr.name] = (
            constr.sense.name,
            constr.rhs,
            constr.linear.to_dict(use_name=True),
            _optmapper_terms(constr.quadratic.to_dict(use_name=True), {}),
        )
    for constr in mod.higher_order_constraints:
        constraints[constr.name] = (
            constr.sense.name,
            constr.rhs,
            constr.linear.to_dict(use_name=True),
            _optmapper_terms(constr.quadratic.to_dict(use_name=True), constr.higher_order),
        )
    return constraints


def _optmapper_terms(
    quadratic: dict[tuple[str, str], float],
    higher_order: dict[int, object],
) -> dict[tuple[str, ...], float]:
    terms = {_term_key(term): coeff for term, coeff in quadratic.items()}
    for expr in higher_order.values():
        terms.update({_term_key(term): coeff for term, coeff in expr.to_dict(use_name=True).items()})
    return terms


def _lm_variables(model: Model) -> dict[str, tuple[Vtype, float | type[Unbounded], float | type[Unbounded]]]:
    return {var.name: (var.vtype, var.bounds.lower, var.bounds.upper) for var in model.variables()}


def _lm_constraints(model: Model) -> dict[str, tuple[Comparator, float, tuple[float, dict[str, float], dict[tuple[str, ...], float]]]]:
    return {
        name: (constr.comparator, constr.rhs, _lm_expr(constr.lhs))
        for name, constr in model.constraints
    }


def _lm_expr(expr: Expression) -> tuple[float, dict[str, float], dict[tuple[str, ...], float]]:
    terms = {_term_key((u.name, v.name)): coeff for u, v, coeff in expr.quadratic_items()}
    terms.update({_term_key(tuple(var.name for var in variables)): coeff for variables, coeff in expr.higher_order_items()})
    return (
        expr.get_offset(),
        {var.name: coeff for var, coeff in expr.linear_items()},
        terms,
    )


def _term_key(term: tuple[str, ...]) -> tuple[str, ...]:
    return tuple(sorted(term))
