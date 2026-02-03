import pytest

from luna_model import Solution, Model, Vtype, Sense

@pytest.fixture
def model() -> Model:
    model = Model()
    x = model.add_variable("x", Vtype.BINARY)
    y = model.add_variable("y", Vtype.BINARY)
    model.set_objective(x + y)
    model.sense = Sense.MAX
    return model

@pytest.fixture
def model_min() -> Model:
    model = Model()
    x = model.add_variable("x", Vtype.BINARY)
    y = model.add_variable("y", Vtype.BINARY)
    model.set_objective(x + y)
    model.sense = Sense.MIN
    return model

@pytest.fixture
def constr_model() -> Model:
    model = Model()
    x = model.add_variable("x", Vtype.BINARY)
    y = model.add_variable("y", Vtype.BINARY)
    model.set_objective(x + y)
    model.sense = Sense.MAX
    model.add_constraint(x + y <= 1)
    return model

@pytest.fixture
def constr_model_min() -> Model:
    model = Model()
    x = model.add_variable("x", Vtype.BINARY)
    y = model.add_variable("y", Vtype.BINARY)
    model.set_objective(x + y)
    model.sense = Sense.MIN
    model.add_constraint(x + y <= 1)
    return model

def build_sol(model: Model) -> Solution:
    return Solution.from_dicts([
        {'x': 1, 'y': 0},
        {'x': 1, 'y': 1},
    ], model=model)

def build_sol_eq(model: Model) -> Solution:
    return Solution.from_dicts([
        {'x': 1, 'y': 0},
        {'x': 0, 'y': 1},
    ], model=model)

def test_solution_best(model: Model):
    solution = build_sol(model)
    best_solutions = solution.best()
    assert best_solutions is not None
    assert len(best_solutions) == 1
    assert 2 == best_solutions[0].obj_value

def test_solution_best_constr(constr_model: Model):
    solution = build_sol(constr_model)
    best_solutions = solution.best()
    assert best_solutions is not None
    assert len(best_solutions) == 1
    assert 1 == best_solutions[0].obj_value

def test_solution_best_constr_eq(constr_model: Model):
    solution = build_sol_eq(constr_model)
    best_solutions = solution.best()
    assert best_solutions is not None
    assert len(best_solutions) == 2
    assert 1 == best_solutions[0].obj_value
    assert 1 == best_solutions[1].obj_value

def test_solution_best_min(model_min: Model):
    solution = build_sol(model_min)
    best_solutions = solution.best()
    assert best_solutions is not None
    assert len(best_solutions) == 1
    assert 1 == best_solutions[0].obj_value

def test_solution_best_constr_min(constr_model_min: Model):
    solution = build_sol(constr_model_min)
    best_solutions = solution.best()
    assert best_solutions is not None
    assert len(best_solutions) == 1
    assert 1 == best_solutions[0].obj_value

def test_solution_best_constr_eq_min(constr_model_min: Model):
    solution = build_sol_eq(constr_model_min)
    best_solutions = solution.best()
    assert best_solutions is not None
    assert len(best_solutions) == 2
    assert 1 == best_solutions[0].obj_value
    assert 1 == best_solutions[1].obj_value
