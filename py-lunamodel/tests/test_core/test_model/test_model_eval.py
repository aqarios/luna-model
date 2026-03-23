from luna_model.solution import sol
import pytest

from luna_model import (
    Bounds,
    ConstraintCollection,
    Model,
    Sense,
    Solution,
    Unbounded,
    Variable,
    Vtype,
)
from luna_model.errors import EvaluationError, VariableNotExistingError


@pytest.fixture()
def solution() -> Solution:
    return Solution(
        samples=[
            {"b": 1, "s": +1, "i": +2, "r": 2.0},
            {"b": 0, "s": -1, "i": +3, "r": 3.0},
            {"b": 1, "s": +1, "i": -4, "r": 4.0},
        ],
        vtypes=[Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL],
        raw_energies=[6.0, 5.0, 2.0],
    )


@pytest.fixture()
def solution_max() -> Solution:
    return Solution(
        samples=[
            {"b": 1, "s": +1, "i": +2, "r": 2.0},
            {"b": 0, "s": -1, "i": +3, "r": 3.0},
            {"b": 1, "s": +1, "i": -4, "r": 4.0},
        ],
        vtypes=[
            Vtype.BINARY,
            Vtype.SPIN,
            Vtype.INTEGER,
            Vtype.REAL,
        ],
        raw_energies=[6.0, 5.0, 2.0],
        sense=Sense.MAX,
    )


@pytest.fixture()
def model_wo_constraint() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.BINARY)
        s = Variable("s", vtype=Vtype.SPIN)
        i = Variable("i", vtype=Vtype.INTEGER)
        r = Variable("r", vtype=Vtype.REAL)
    model.objective = b + s + i + r
    return model


@pytest.fixture()
def model_wo_constraint_maximize() -> Model:
    model = Model("test_model_maximize", sense=Sense.MAX)
    with model.environment:
        b = Variable("b", vtype=Vtype.BINARY)
        s = Variable("s", vtype=Vtype.SPIN)
        i = Variable("i", vtype=Vtype.INTEGER)
        r = Variable("r", vtype=Vtype.REAL)
    model.objective = b + s + i + r + 3.0
    return model


@pytest.fixture()
def model_wo_constraint_one_less_var() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.BINARY)
        s = Variable("s", vtype=Vtype.SPIN)
        i = Variable("i", vtype=Vtype.INTEGER)
    model.objective = b + s + i + b * s + 3.0
    return model


@pytest.fixture()
def model_wo_constraint_one_more_var() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.BINARY)
        s = Variable("s", vtype=Vtype.SPIN)
        i = Variable("i", vtype=Vtype.INTEGER)
        r = Variable("r", vtype=Vtype.REAL)
        b2 = Variable("b2", vtype=Vtype.BINARY)
    model.objective = b + s + i + r + b * b2 + i * r + 5.12
    return model


@pytest.fixture()
def model_w_constraint() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.BINARY)
        s = Variable("s", vtype=Vtype.SPIN)
        i = Variable("i", vtype=Vtype.INTEGER, bounds=Bounds(lower=Unbounded, upper=Unbounded))
        r = Variable("r", vtype=Vtype.REAL)
    model.objective = b + s + i + r
    model.constraints += b + s + i + r <= 10.0
    model.constraints += b + s + i + r <= 10.0
    return model


@pytest.fixture()
def model_w_constraint_infeasible() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.BINARY)
        s = Variable("s", vtype=Vtype.SPIN)
        i = Variable("i", vtype=Vtype.INTEGER)
        r = Variable("r", vtype=Vtype.REAL)
    model.objective = b + s + i + r
    model.constraints += b + s + i + r <= 0.0
    model.constraints += b + s + i + r <= 0.0
    return model


def test_model_eval_wo_constraint(model_wo_constraint: Model, solution: Solution):
    new_sol = model_wo_constraint.evaluate(solution)
    assert all(new_sol.raw_energies == solution.raw_energies)
    assert all(new_sol.obj_values == solution.raw_energies)

    assert len(new_sol) == 3



def test_model_eval_wo_constraint_one_less_var_in_model(model_wo_constraint_one_less_var: Model, solution: Solution):
    with pytest.raises(VariableNotExistingError):
        _ = model_wo_constraint_one_less_var.evaluate(solution)


def test_model_eval_wo_constraint_one_more_var_in_model(model_wo_constraint_one_more_var: Model, solution: Solution):
    with pytest.raises(EvaluationError):
        _ = model_wo_constraint_one_more_var.evaluate(solution)


def test_model_eval_w_constraint(model_w_constraint: Model, solution: Solution):
    new_sol = model_w_constraint.evaluate(solution)
    print(solution)
    print(new_sol)
    print(new_sol.raw_energies)
    print(solution.raw_energies)
    assert all(new_sol.raw_energies == solution.raw_energies)
    assert all(new_sol.obj_values == solution.raw_energies)
    for res in new_sol.results:
        assert res.constraints is not None
        for constr in res.constraints:
            assert constr
        assert res.variable_bounds is not None
        for varbound in res.variable_bounds:
            assert varbound
        assert res.feasible is not None
        assert res.feasible


def test_model_eval_w_constraint_infeasible(model_w_constraint_infeasible: Model, solution: Solution):
    new_sol = model_w_constraint_infeasible.evaluate(solution)
    assert all(new_sol.raw_energies == solution.raw_energies)
    assert all(new_sol.obj_values == solution.raw_energies)
    for res in new_sol.results:
        assert res.constraints is not None
        for _, constr in res.constraints.items():
            assert not constr
        assert res.variable_bounds is not None
        for _, varbounds in res.variable_bounds.items():
            assert varbounds or not varbounds
        assert res.feasible is not None
        assert not res.feasible


def test_model_eval_infeasible_bounds():
    m = Model("test_eval_bounds")
    with m.environment:
        x1 = Variable("x1", vtype=Vtype.INTEGER, bounds=Bounds(2, 3))
        x2 = Variable("x2", vtype=Vtype.INTEGER, bounds=Bounds(2, 3))
        x3 = Variable("x3", vtype=Vtype.INTEGER)

    m.objective = 5 * x1 + 3 * x2 + 2 * x3
    m.add_constraint(x1 + x2 == 6, "c1")

    sol_dict = {"x1": 5, "x2": 1, "x3": 10}
    sol = Solution.from_dict(sol_dict, model=m)
    assert len(sol.samples) == 1

    res = sol[0]
    assert res.feasible is False
    assert res.constraints is not None
    assert list(res.constraints.values()) == [True]
    assert res.variable_bounds is not None
    assert res.variable_bounds == {"x1": False, "x2": False, "x3": True}

    sample = m.evaluate_sample(sol.samples[0])
    assert sample.feasible is False
    assert sample.constraints is not None
    assert list(sample.constraints.values()) == [True]
    assert sample.variable_bounds is not None
    assert res.variable_bounds == {"x1": False, "x2": False, "x3": True}


def test_model_violated_constraints():
    m = Model("test_eval_bounds")
    with m.environment:
        x1 = Variable("x1")
        x2 = Variable("x2")
        x3 = Variable("x3")

    m.objective = x1 - 3 * x2 + 2 * x3

    sol_dict = {"x1": 1, "x2": 1, "x3": 0}

    sol_no_constr = Solution.from_dict(sol_dict, model=m)
    assert len(sol_no_constr.samples) == 1
    sample_no_constr = sol_no_constr.samples[0]
    assert m.violated_constraints(sample_no_constr) == ConstraintCollection()

    c1 = x1 + x2 <= 1
    c2 = x1 + x2 + x3 <= 2
    c3 = x1 + x3 >= 1

    m.add_constraint(c1)
    m.add_constraint(c2)
    m.add_constraint(c3)

    sol = Solution.from_dict(sol_dict, model=m)
    assert len(sol.samples) == 1
    sample = sol.samples[0]
    violated = ConstraintCollection()
    violated.add_constraint(c1)
    assert m.violated_constraints(sample) == violated
