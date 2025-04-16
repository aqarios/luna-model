import pytest
from aqmodels import Solution, Variable, Vtype, Model


@pytest.fixture
def solution() -> Solution:
    return Solution.build(
        component_types=[
            Vtype.Binary,
            Vtype.Spin,
            Vtype.Integer,
            Vtype.Real,
        ],
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
def model_w_constraint() -> Model:
    model = Model("test_model")
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable("i", vtype=Vtype.Integer)
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


def test_model_eval_w_constraint(model_w_constraint: Model, solution: Solution):
    new_sol = model_w_constraint.evaluate(solution)
    assert all(new_sol.raw_energies == solution.raw_energies)
    assert all(new_sol.obj_values == solution.raw_energies)
    for res in new_sol.results:
        assert res.constraints is not None
        for constr in res.constraints:
            assert constr
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
        assert res.feasible is not None
        assert not res.feasible
