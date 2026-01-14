import math
import random

import pytest
from luna_model import Environment, Model, Solution, Variable, Vtype

TOL = 1e-12


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


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_cvar_properties_min_mean_monotone(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 0, y: 0, z: 1},
        {x: 1, y: 1, z: 1},
        {x: 1, y: 0, z: 0},
        {x: 0, y: 1, z: 1},
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.obj_values is not None

    vals = sorted(sol.obj_values)
    mean = sum(vals) / len(vals)
    minv = vals[0]

    # Limits: α→0+ gives min; α=1 gives mean
    assert sol.cvar(alpha=1.0) == pytest.approx(mean, abs=TOL)
    assert sol.cvar(alpha=1e-9) == pytest.approx(minv, abs=TOL)
    assert sol.cvar(alpha=0.25) == pytest.approx(minv, abs=TOL)  # K=4 ⇒ m=ceil(αK)=1

    # Monotonic in α (nondecreasing from min to mean)
    cvar_025 = sol.cvar(alpha=0.25)
    cvar_050 = sol.cvar(alpha=0.50)
    cvar_100 = sol.cvar(alpha=1.00)
    assert cvar_025 <= cvar_050 <= cvar_100


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_cvar_matches_empirical_formula(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 0, y: 0, z: 1},
        {x: 1, y: 1, z: 1},
        {x: 1, y: 0, z: 0},
        {x: 0, y: 1, z: 1},
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.obj_values is not None

    vals = sorted(sol.obj_values)
    K = len(vals)

    def empirical(alpha: float) -> float:
        m = max(1, math.ceil(alpha * K))
        return sum(vals[:m]) / m

    for alpha in [0.10, 0.25, 0.50, 0.51, 0.75, 1.0]:
        assert sol.cvar(alpha=alpha) == pytest.approx(empirical(alpha), abs=TOL)


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_cvar_order_invariance(model: tuple[Model, tuple[Variable, ...]]):
    """CVaR should not depend on the order the samples are given."""
    m, (x, y, z) = model
    base = [
        {x: 0, y: 0, z: 1},
        {x: 1, y: 1, z: 1},
        {x: 1, y: 0, z: 0},
        {x: 0, y: 1, z: 1},
    ]
    shuffled = base[:]
    random.Random(123).shuffle(shuffled)

    sol_a = Solution.from_dicts(base, model=m)
    sol_b = Solution.from_dicts(shuffled, model=m)

    for alpha in [0.1, 0.25, 0.5, 0.9, 1.0]:
        assert sol_a.cvar(alpha=alpha) == pytest.approx(
            sol_b.cvar(alpha=alpha), abs=TOL
        )


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_cvar_invalid_alpha_raises(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 0, y: 0, z: 1},
        {x: 1, y: 1, z: 1},
        {x: 1, y: 0, z: 0},
        {x: 0, y: 1, z: 1},
    ]
    sol = Solution.from_dicts(samples, model=m)

    for bad in [0.0, -0.1, float("nan"), float("inf")]:
        with pytest.raises(Exception):
            _ = sol.cvar(alpha=bad)
