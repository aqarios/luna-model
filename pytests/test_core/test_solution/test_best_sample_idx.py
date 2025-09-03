import pytest

from aqmodels import Environment, Model, Solution, Variable, Vtype


def vars(n, vtype) -> tuple[tuple[Variable, ...], Environment]:
    env = Environment()
    with env:
        variables = [Variable(f"x_{i}", vtype=vtype) for i in range(n)]
    return tuple(variables), env


@pytest.fixture
def model(request):
    (x, y, z), env = vars(*request.param)
    model = Model(env=env)
    model.objective = x - y - z
    model.constraints += x <= 4
    model.constraints += y <= 4
    model.constraints += z <= 4
    return model, (x, y, z)


@pytest.mark.parametrize("model", [(3, Vtype.Integer)], indirect=True)
def test_model_bsi_all_feasible(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    # feasible when value smaller than 4.
    samples = [
        {x: 1, y: 2, z: 3},  # = -4
        {x: 2, y: 3, z: 2},  # = -3
        {x: 2, y: 2, z: 0},  # = 0
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.best_sample_idx == 0


@pytest.mark.parametrize("model", [(3, Vtype.Integer)], indirect=True)
def test_model_bsi_no_feasible(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    # feasible when value smaller than 4.
    samples = [
        {x: 5, y: 0, z: 8},  # = -3
        {x: 3, y: 7, z: 2},  # = -6
        {x: 5, y: 21, z: 8},  # = -24
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.best_sample_idx is None


@pytest.mark.parametrize("model", [(3, Vtype.Integer)], indirect=True)
def test_model_bsi_max_feasible(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    # feasible when value smaller than 4.
    samples = [
        {x: 3, y: 7, z: 2},  # = -6
        {x: 4, y: 0, z: 0},  # = +4
        {x: 5, y: 21, z: 8},  # = -24
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.best_sample_idx == 1


@pytest.mark.parametrize("model", [(3, Vtype.Integer)], indirect=True)
def test_model_bsi_min_feasible(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    m, (x, y, z) = model
    # feasible when value smaller than 4.
    samples = [
        {x: 0, y: 5, z: 2},  # = -7
        {x: 0, y: 0, z: 6},  # = -6
        {x: 0, y: 4, z: 4},  # = -8
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.best_sample_idx == 2
