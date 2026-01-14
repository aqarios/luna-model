import pytest
from luna_model import Environment, Model, Solution, Timer, Variable, Vtype


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
def test_set_runtime(model: tuple[Model, tuple[Variable, ...]]):
    timer = Timer.start()
    m, (x, y, z) = model
    sample = {x: 0, y: 0, z: 1}
    sol = Solution.from_dict(sample, model=m)
    timing = timer.stop()
    sol.runtime = timing
    assert timing == sol.runtime
