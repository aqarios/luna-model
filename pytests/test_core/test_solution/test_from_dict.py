import pytest

from aqmodels import Variable, Environment, Vtype, Solution, Model


def vars(n, vtype) -> tuple[tuple[Variable, ...], Environment]:
    env = Environment()
    with env:
        variables = [Variable(f"{i}", vtype=vtype) for i in range(n)]
    return tuple(variables), env


@pytest.fixture
def variables(request) -> tuple[tuple[Variable, ...], Environment]:
    return vars(*request.param)


@pytest.fixture
def model(request):
    (x, y, z), env = vars(*request.param)
    model = Model(env=env)
    model.objective = x - y - z
    return model, (x, y, z)


@pytest.mark.solution
@pytest.mark.parametrize("variables", [(3, Vtype.Binary)], indirect=True)
def test_from_dict_no_model(variables: tuple[tuple[Variable, ...], Environment]):
    (x, y, z), env = variables
    sample = {x: 0, y: 0, z: 1}
    sol = Solution.from_dict(sample, env=env)
    assert sol.samples.tolist() == [[0, 0, 1]]

    sample = {x: 0.0, y: 0.0, z: 1.0}
    sol = Solution.from_dict(sample, env=env)
    assert sol.samples.tolist() == [[0, 0, 1]]

    sample = {x: 0, y: 0.0, z: 1.0}
    sol = Solution.from_dict(sample, env=env)
    assert sol.samples.tolist() == [[0, 0, 1]]

    sample = {x.name: 0, y.name: 0, z.name: 1}
    sol = Solution.from_dict(sample, env=env)
    assert sol.samples.tolist() == [[0, 0, 1]]

    sample = {x.name: 0, y.name: 0.0, z: 1}
    sol = Solution.from_dict(sample, env=env)
    assert sol.samples.tolist() == [[0, 0, 1]]


@pytest.mark.solution
@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_from_dict_with_model(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    sample = {x: 0, y: 0, z: 1}
    sol = Solution.from_dict(sample, model=m)
    assert sol.samples.tolist() == [[0, 0, 1]]
    assert sol.obj_values.tolist() == [-1.0]
