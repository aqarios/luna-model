import pytest

from aqmodels import Solution, Variable, Vtype, Model, Bounds, Unbounded, Sense
from aqmodels.errors import EvaluationError


@pytest.fixture
def solution() -> Solution:
    return Solution._build( # type: ignore[reportAttributeAccessIssue]
        component_types=[
            Vtype.Binary,
            Vtype.Spin,
            Vtype.Integer,
            Vtype.Real,
        ],
        variable_names=["b", "s", "i", "r"],
        binary_cols=[[1, 0, 1]],
        spin_cols=[[+1, -1, +1]],
        int_cols=[[2, 3, -4]],
        real_cols=[[2.0, 3.0, 4.0]],
        raw_energies=[6.0, 5.0, 2.0],
    )


@pytest.fixture
def model_wo_constraint() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable("i", vtype=Vtype.Integer)
        r = Variable("r", vtype=Vtype.Real)
    model.objective = b + s + i + r
    return model


@pytest.fixture
def model_wo_constraint_maximize() -> Model:
    model = Model("test_model_maximize", sense=Sense.Max)
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable("i", vtype=Vtype.Integer)
        r = Variable("r", vtype=Vtype.Real)
    model.objective = b + s + i + r
    return model


@pytest.fixture
def model_wo_constraint_one_less_var() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable("i", vtype=Vtype.Integer)
    model.objective = b + s + i + b * s
    return model


@pytest.fixture
def model_wo_constraint_one_more_var() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable("i", vtype=Vtype.Integer)
        r = Variable("r", vtype=Vtype.Real)
        b2 = Variable("b2", vtype=Vtype.Binary)
    model.objective = b + s + i + r + b * b2 + i * r
    return model


@pytest.fixture
def model_w_constraint() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable(
            "i", vtype=Vtype.Integer, bounds=Bounds(lower=Unbounded, upper=Unbounded)
        )
        r = Variable("r", vtype=Vtype.Real)
    model.objective = b + s + i + r
    model.constraints += b + s + i + r <= 10.0
    model.constraints += b + s + i + r <= 10.0
    return model


@pytest.fixture
def model_w_constraint_infeasible() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable("i", vtype=Vtype.Integer)
        r = Variable("r", vtype=Vtype.Real)
    model.objective = b + s + i + r
    model.constraints += b + s + i + r <= 0.0
    model.constraints += b + s + i + r <= 0.0
    return model


def test_model_eval_wo_constraint(model_wo_constraint: Model, solution: Solution):
    new_sol = model_wo_constraint.evaluate(solution)
    assert all(new_sol.raw_energies == solution.raw_energies)
    assert all(new_sol.obj_values == solution.raw_energies)

    assert len(new_sol) == 3


def test_model_eval_wo_constraint_best(model_wo_constraint: Model, solution: Solution):
    new_sol = model_wo_constraint.evaluate(solution)
    assert all(new_sol.raw_energies == solution.raw_energies)
    assert all(new_sol.obj_values == solution.raw_energies)
    assert new_sol.best_sample_idx is not None
    assert new_sol.best_sample_idx == 1
    assert new_sol.best() == new_sol[new_sol.best_sample_idx]


def test_model_eval_wo_constraint_best_maximize(
    model_wo_constraint_maximize: Model, solution: Solution
):
    new_sol = model_wo_constraint_maximize.evaluate(solution)
    assert all(new_sol.raw_energies == solution.raw_energies)
    assert all(new_sol.obj_values == solution.raw_energies)
    assert new_sol.best_sample_idx is not None
    assert new_sol.best_sample_idx == 0
    assert new_sol.best() == new_sol[new_sol.best_sample_idx]


def test_model_eval_wo_constraint_one_less_var_in_model(
    model_wo_constraint_one_less_var: Model, solution: Solution
):
    with pytest.raises(EvaluationError):
        _ = model_wo_constraint_one_less_var.evaluate(solution)


def test_model_eval_wo_constraint_one_more_var_in_model(
    model_wo_constraint_one_more_var: Model, solution: Solution
):
    with pytest.raises(EvaluationError):
        _ = model_wo_constraint_one_more_var.evaluate(solution)


def test_model_eval_w_constraint(model_w_constraint: Model, solution: Solution):
    new_sol = model_w_constraint.evaluate(solution)
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


def test_model_eval_w_constraint_infeasible(
    model_w_constraint_infeasible: Model, solution: Solution
):
    new_sol = model_w_constraint_infeasible.evaluate(solution)
    assert all(new_sol.raw_energies == solution.raw_energies)
    assert all(new_sol.obj_values == solution.raw_energies)
    for res in new_sol.results:
        assert res.constraints is not None
        for constr in res.constraints:
            assert not constr
        assert res.variable_bounds is not None
        for varbounds in res.variable_bounds:
            assert varbounds or not varbounds
        assert res.feasible is not None
        assert not res.feasible


def test_model_eval_infeasible_bounds():
    m = Model("test_eval_bounds")
    with m.environment:
        x1 = Variable("x1", vtype=Vtype.Integer, bounds=Bounds(2, 3))
        x2 = Variable("x2", vtype=Vtype.Integer, bounds=Bounds(2, 3))
        x3 = Variable("x3", vtype=Vtype.Integer)

    m.objective = 5 * x1 + 3 * x2 + 2 * x3
    m.add_constraint(x1 + x2 == 6, "c1")

    sol_dict = {"x1": 5, "x2": 1, "x3": 10}
    sol = Solution.from_dict(sol_dict, model=m)
    assert len(sol.samples) == 1

    res = sol[0]
    assert res.feasible is False
    assert res.constraints is not None
    assert res.constraints.tolist() == [True]
    assert res.variable_bounds is not None
    assert res.variable_bounds.tolist() == [False, False, True]

    sample = m.evaluate_sample(sol.samples[0])
    assert sample.feasible is False
    assert sample.constraints is not None
    assert sample.constraints.tolist() == [True]
    assert sample.variable_bounds is not None
    assert sample.variable_bounds.tolist() == [False, False, True]
