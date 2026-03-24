from random import Random
from luna_model import Solution, Model, Variable, Environment, Vtype, errors

import pytest
import numpy as np

from tests.test_core.utils import make_seed, random_int

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
def test_from_dicts_illegal_too_few(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0},  # 1
        {x: 0, y: 1},  # -2
    ]
    with pytest.raises(errors.SampleIncorrectLengthError):
        _ = Solution.from_dicts(samples, model=m)

@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_dicts_illegal_too_many(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 1, y: 0},  # 1
        {x: 0, y: 1},  # -2
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
    ]
    with pytest.raises(errors.SampleIncorrectLengthError):
        _ = Solution.from_dicts(samples, model=m)

@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_counts_illegal_too_many(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    rand = Random(make_seed())
    counts = {
        "100": random_int(rand),
        "111": random_int(rand),
        "0011": random_int(rand),
        "1101": random_int(rand),
    }
    with pytest.raises(errors.SampleIncorrectLengthError):
        _ = Solution.from_counts(counts, model=m)

@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_counts_illegal_too_few(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    rand = Random(make_seed())
    counts = {
        "1001": random_int(rand),
        "1111": random_int(rand),
        "001": random_int(rand),
        "110": random_int(rand),
    }
    with pytest.raises(errors.SampleIncorrectLengthError):
        _ = Solution.from_counts(counts, model=m)

@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_arrays_illegal_too_many(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    with pytest.raises(ValueError):
        arrs = np.array([[0, 0, 1], [1, 1, 0], [1, 0, 0, 1], [1, 1, 1, 1]])
        _ = Solution.from_arrays(arrs, model=m)

@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_arrays_illegal_too_few(model: tuple[Model, tuple[Variable, ...]]):
    m, _ = model
    with pytest.raises(ValueError):
        arrs = np.array([[1, 0, 0, 1], [1, 1, 1, 1], [0, 0, 1], [1, 1, 0]])
        _ = Solution.from_arrays(arrs, model=m)


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_many_illegal_too_few(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples_a = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
    ]
    samples_b = [
        {x: 1, y: 0},  # 1
        {x: 0, y: 1},  # -2
    ]
    sol_a = Solution.from_dicts(samples_a, env=m.environment)
    sol_b = Solution.from_dicts(samples_b, env=m.environment)

    with pytest.raises(errors.UnsupportedOperationError):
        _ = Solution.from_many([sol_a, sol_b])

@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_many_illegal_too_many(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples_a = [
        {x: 1, y: 0},  # 1
        {x: 0, y: 1},  # -2
    ]
    samples_b = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
    ]
    sol_a = Solution.from_dicts(samples_a, env=m.environment)
    sol_b = Solution.from_dicts(samples_b, env=m.environment)

    with pytest.raises(errors.UnsupportedOperationError):
        _ = Solution.from_many([sol_a, sol_b])
