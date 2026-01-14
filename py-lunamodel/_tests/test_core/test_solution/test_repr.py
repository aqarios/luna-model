import pytest
from luna_model import Environment, Model, Solution, Variable, Vtype


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
def test_from_dict_with_model(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    sample = {x: 0, y: 0, z: 1}
    sol = Solution.from_dict(sample, model=m)
    sol_repr = repr(sol)
    assert (
        sol_repr
        == "Solution(samples=[[0, 0, 1]], obj_values=[-1], raw_energies=None, counts=[1], constraints=[[]], variable_bounds=[[True, True, True]], feasible=[True], best_sample_idx=0, runtime=None, n_samples=1, variable_names=[x_0, x_1, x_2], sense=Minimize)"
    )


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_dicts_unique_with_model(model: tuple[Model, tuple[Variable, ...]]):
    m, (x, y, z) = model
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        {x: 0, y: 1, z: 1},  # -2
    ]
    sol = Solution.from_dicts(samples, model=m)
    assert sol.samples.tolist() == [
        [0, 0, 1],
        [1, 1, 1],
        [1, 0, 0],
        [0, 1, 1],
    ]
    assert sol.obj_values is not None
    assert sol.obj_values.tolist() == [-1.0, -1.0, 1.0, -2.0]
    sol_repr = repr(sol)
    assert (
        sol_repr
        == "Solution(samples=[[0, 0, 1], [1, 1, 1], [1, 0, 0], [0, 1, 1]], obj_values=[-1, -1, 1, -2], raw_energies=None, counts=[1, 1, 1, 1], constraints=[[], [], [], []], variable_bounds=[[True, True, True], [True, True, True], [True, True, True], [True, True, True]], feasible=[True, True, True, True], best_sample_idx=3, runtime=None, n_samples=4, variable_names=[x_0, x_1, x_2], sense=Minimize)"
    )


@pytest.mark.parametrize("model", [(3, Vtype.BINARY)], indirect=True)
def test_from_dicts_duplicate_with_model(model: tuple[Model, tuple[Variable, ...]]):
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
    assert len(sol.samples) == 4
    assert len(sol.counts) == 4
    assert sol.samples.tolist() == [
        [0, 0, 1],  # -1
        [1, 1, 1],  # -1
        [1, 0, 0],  # 1
        [0, 1, 1],  # -2
    ]
    assert sol.counts.tolist() == counts
    assert sol.obj_values is not None
    assert sol.obj_values.tolist() == [-1, -1, 1, -2]
    sol_repr = repr(sol)
    assert (
        sol_repr
        == "Solution(samples=[[0, 0, 1], [1, 1, 1], [1, 0, 0], [0, 1, 1]], obj_values=[-1, -1, 1, -2], raw_energies=None, counts=[2, 3, 1, 4], constraints=[[], [], [], []], variable_bounds=[[True, True, True], [True, True, True], [True, True, True], [True, True, True]], feasible=[True, True, True, True], best_sample_idx=3, runtime=None, n_samples=4, variable_names=[x_0, x_1, x_2], sense=Minimize)"
    )
