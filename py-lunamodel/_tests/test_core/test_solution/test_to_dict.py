from random import Random

import pytest
from luna_model import Environment, Model, Solution, Variable, Vtype

from _tests.test_core.utils import make_seed, random_int


def vars(n, vtype) -> tuple[tuple[Variable, ...], Environment]:
    env = Environment()
    with env:
        variables = [Variable(f"x_{i}", vtype=vtype) for i in range(n)]
    return tuple(variables), env


@pytest.fixture()
def variables(request) -> tuple[tuple[Variable, ...], Environment]:
    return vars(*request.param)


@pytest.fixture()
def model(request):
    (x, y, z), env = vars(*request.param)
    model = Model(env=env)
    model.objective = x - y - z
    return model, (x, y, z)


@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_to_dict_with_model(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    sample = {x: 0, y: 0, z: 1}
    sol = Solution.from_dict(sample, model=m)
    assert sol.samples.tolist() == [[0, 0, 1]]
    assert sol.obj_values.tolist() == [-1.0]
    best = sol.best()
    assert best is not None
    assert best.sample.to_dict() == {v.name: a for v, a in sample.items()}


@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_to_dict_with_model_and_counts(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    sample = {x: 0, y: 0, z: 1}
    counts = random_int(rand=Random(make_seed()))
    sol = Solution.from_dict(sample, model=m, counts=counts)
    assert sol.samples.tolist() == [[0, 0, 1]]
    assert sol.obj_values.tolist() == [-1.0]
    assert sol.counts == counts
    best = sol.best()
    assert best is not None
    assert best.sample.to_dict() == {v.name: a for v, a in sample.items()}


@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_to_dicts_unique_with_model(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    best_sample = {x: 0, y: 1, z: 1}  # -2
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        best_sample,  # -2
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.samples.tolist() == [
        [0, 0, 1],
        [1, 1, 1],
        [1, 0, 0],
        [0, 1, 1],
    ]
    assert sol.obj_values.tolist() == [-1.0, -1.0, 1.0, -2.0]
    best = sol.best()
    assert best is not None
    assert best.sample.to_dict() == {v.name: a for v, a in best_sample.items()}


@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_to_dicts_duplicate_with_model(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    sample_a = {x: 0, y: 0, z: 1}  # -1
    sample_b = {x: 1, y: 1, z: 1}  # -1
    sample_c = {x: 1, y: 0, z: 0}  # 1
    sample_d = {x: 0, y: 1, z: 1}  # -2
    samples = [
        sample_a,
        sample_b,
        sample_c,
        sample_d,
        sample_b,
        sample_d,
        sample_a,
        sample_b,
        sample_d,
        sample_d,
    ]
    counts = [2, 3, 1, 4]
    sol = Solution.from_dicts(samples, model=m)
    assert len(list(sol.samples)) == 4
    assert len(sol.counts) == 4
    assert sol.samples.tolist() == [
        [0, 0, 1],  # -1
        [1, 1, 1],  # -1
        [1, 0, 0],  # 1
        [0, 1, 1],  # -2
    ]
    assert sol.counts.tolist() == counts
    assert sol.obj_values.tolist() == [-1, -1, 1, -2]
    best = sol.best()
    assert best is not None
    assert best.sample.to_dict() == {v.name: a for v, a in sample_d.items()}


@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_to_dicts_duplicate_with_model_and_counts(
    model: tuple[Model, tuple[Variable, ...]],
):
    m, (x, y, z) = model
    sample_a = {x: 0, y: 0, z: 1}  # -1
    sample_b = {x: 1, y: 1, z: 1}  # -1
    sample_c = {x: 1, y: 0, z: 0}  # 1
    sample_d = {x: 0, y: 1, z: 1}  # -2
    samples = [
        sample_a,
        sample_b,
        sample_c,
        sample_d,
    ]
    rand = Random(make_seed())
    counts = [random_int(rand), random_int(rand), random_int(rand), random_int(rand)]
    sol = Solution.from_dicts(samples, model=m, counts=counts)
    assert len(sol.samples) == 4
    assert len(sol.counts) == 4
    assert sol.samples.tolist() == [
        [0, 0, 1],  # -1
        [1, 1, 1],  # -1
        [1, 0, 0],  # 1
        [0, 1, 1],  # -2
    ]
    assert sol.counts.tolist() == counts
    assert sol.obj_values.tolist() == [-1, -1, 1, -2]
    best = sol.best()
    assert best is not None
    assert best.sample.to_dict() == {v.name: a for v, a in sample_d.items()}
