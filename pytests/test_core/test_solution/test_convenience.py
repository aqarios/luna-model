import numpy as np
import pytest

from luna_model import Environment, Model, Solution, Variable, Vtype


def vars(n, vtype) -> tuple[tuple[Variable, ...], Environment]:
    env = Environment()
    with env:
        variables = [Variable(f"x_{i}", vtype=vtype) for i in range(n)]
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
@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_expectation_value(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        {x: 0, y: 1, z: 1},  # -2
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.obj_values is not None
    gt_expval = np.average(sol.obj_values, weights=sol.counts)
    gt_manual_expval = float(
        sum(w * o for w, o in zip(sol.counts, sol.obj_values)) / sum(sol.counts)
    )
    assert gt_expval == gt_manual_expval

    assert sol.samples.tolist() == [
        [0, 0, 1],
        [1, 1, 1],
        [1, 0, 0],
        [0, 1, 1],
    ]
    assert sol.obj_values is not None
    assert sol.obj_values.tolist() == [-1.0, -1.0, 1.0, -2.0]
    assert sol.expectation_value() == gt_expval


@pytest.mark.solution
@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_cvar(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        {x: 0, y: 1, z: 1},  # -2
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.obj_values is not None
    assert sol.cvar(alpha=1.0) == sol.expectation_value()


@pytest.mark.solution
@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_feasibility_ratio(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    m.add_constraint(x + y + z <= 1)
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        {x: 0, y: 1, z: 1},  # -2
    ]
    sol = Solution.from_dicts(samples, model=m, counts=[1, 2, 3, 4])

    assert sol.samples.tolist() == [
        [0, 0, 1],
        [1, 1, 1],
        [1, 0, 0],
        [0, 1, 1],
    ]
    assert sol.obj_values is not None
    assert sol.obj_values.tolist() == [-1.0, -1.0, 1.0, -2.0]
    assert sol.feasibility_ratio() == 0.4


@pytest.mark.solution
@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_filter_feasible(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    m.add_constraint(x + y + z <= 1)
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        {x: 0, y: 1, z: 1},  # -2
    ]
    sol = Solution.from_dicts(samples, model=m, counts=[1, 2, 3, 4])

    assert sol.samples.tolist() == [
        [0, 0, 1],
        [1, 1, 1],
        [1, 0, 0],
        [0, 1, 1],
    ]
    assert sol.obj_values is not None
    assert sol.obj_values.tolist() == [-1.0, -1.0, 1.0, -2.0]

    sol_feasible = sol.filter_feasible()
    expected = Solution.from_dicts(
        [{x: 0, y: 0, z: 1}, {x: 1, y: 0, z: 0}], model=m, counts=[1, 3]
    )
    assert sol_feasible == expected


@pytest.mark.solution
@pytest.mark.parametrize("model", [(3, Vtype.Binary)], indirect=True)
def test_filter_feasible_callback(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    m.add_constraint(x + y + z <= 1)
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        {x: 0, y: 1, z: 1},  # -2
    ]
    sol = Solution.from_dicts(samples, model=m, counts=[1, 2, 3, 4])

    assert sol.samples.tolist() == [
        [0, 0, 1],
        [1, 1, 1],
        [1, 0, 0],
        [0, 1, 1],
    ]
    assert sol.obj_values is not None
    assert sol.obj_values.tolist() == [-1.0, -1.0, 1.0, -2.0]

    expected_sol_feasible = sol.filter_feasible()
    sol_feasible_with_ffn = sol.filter(lambda res: bool(res.feasible))
    assert expected_sol_feasible == sol_feasible_with_ffn
