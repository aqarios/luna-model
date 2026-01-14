import pytest
from luna_model import Environment, Model, Solution, Variable, Vtype


def to_str_dict(d: dict[Variable, int | float]) -> dict[str, int | float]:
    return {v.name: a for v, a in d.items()}


def vars(n, vtype) -> tuple[tuple[Variable, ...], Environment]:
    env = Environment()
    with env:
        variables = [Variable(f"x_{i}", vtype=vtype) for i in range(n)]
    return tuple(variables), env


@pytest.fixture()
def model(request):
    (x, y, z), env = vars(*request.param)
    model = Model(env=env)
    model.objective = x - y - z
    return model, (x, y, z)


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_index_sample_var(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    sample = {x: 0, y: 0, z: 1}
    sol = Solution.from_dict(sample, model=m)
    best = sol.best()
    assert best is not None
    best_sample = best[0].sample
    assert best_sample[x] == sample[x]
    assert best_sample[y] == sample[y]
    assert best_sample[z] == sample[z]


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_index_sample_index(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    sample = {x: 0, y: 0, z: 1}
    sol = Solution.from_dict(sample, model=m)
    best = sol.best()
    assert best is not None
    best_sample = best.sample
    assert best_sample[0] == sample[x]
    assert best_sample[1] == sample[y]
    assert best_sample[2] == sample[z]


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_index_sample_name(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    sample = {x: 0, y: 0, z: 1}
    sol = Solution.from_dict(sample, model=m)
    best = sol.best()
    assert best is not None
    best_sample = best.sample
    assert best_sample[f"x_{0}"] == sample[x]
    assert best_sample[f"x_{1}"] == sample[y]
    assert best_sample[f"x_{2}"] == sample[z]


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_index_sample_on_sol(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # +1
        {x: 0, y: 1, z: 1},  # -2
    ]
    sol = Solution.from_dicts(samples, model=m)
    for i in range(len(sol)):
        assert sol[i].sample[0] == samples[i][x]
        assert sol[i].sample[1] == samples[i][y]
        assert sol[i].sample[2] == samples[i][z]
        assert sol[i].sample[x] == samples[i][x]
        assert sol[i].sample[y] == samples[i][y]
        assert sol[i].sample[z] == samples[i][z]
        assert sol[i].sample[f"x_{0}"] == samples[i][x]
        assert sol[i].sample[f"x_{1}"] == samples[i][y]
        assert sol[i].sample[f"x_{2}"] == samples[i][z]
