import pytest

from luna_model import Model, Sense, Solution, Vtype
from luna_model.errors import ComputationError


@pytest.mark.parametrize(("sense", "expected"), [(Sense.MIN, 0.0), (Sense.MAX, 10.0)])
def test_cvar_takes_best_tail_for_sense(sense, expected):
    m = Model(sense=sense)
    x = m.add_variable("x")
    m.objective = 10 * x
    sol = Solution.from_dicts([{"x": 1}, {"x": 0}], model=m)
    assert sol.cvar(0.5) == expected


@pytest.mark.parametrize(("sense", "expected"), [(Sense.MIN, 0.0), (Sense.MAX, 3.0)])
def test_cvar_weights_by_counts(sense, expected):
    # 3 (x3) and 0 (x1): the best quarter is a single shot.
    m = Model(sense=sense)
    x = m.add_variable("x", vtype=Vtype.INTEGER, upper=10)
    m.objective = x
    sol = Solution.from_dicts([{"x": 3}, {"x": 3}, {"x": 3}, {"x": 0}], model=m)
    assert sol.cvar(0.25) == expected


def test_cvar_partial_count_is_split():
    # 1 (x2) and 5 (x2): the best three shots are 1, 1, 5.
    m = Model()
    x = m.add_variable("x", vtype=Vtype.INTEGER, upper=10)
    m.objective = x
    sol = Solution.from_dicts([{"x": 1}, {"x": 1}, {"x": 5}, {"x": 5}], model=m)
    assert sol.cvar(0.75) == pytest.approx(7 / 3)


@pytest.mark.parametrize("sense", [Sense.MIN, Sense.MAX])
def test_cvar_full_alpha_is_expectation_value(sense):
    m = Model(sense=sense)
    x = m.add_variable("x", vtype=Vtype.INTEGER, upper=10)
    m.objective = x
    sol = Solution.from_dicts([{"x": 2}, {"x": 2}, {"x": 7}], model=m)
    assert sol.cvar(1.0) == pytest.approx(sol.expectation_value())


def test_cvar_invalid_alpha_raises_computation_error():
    m = Model()
    x = m.add_variable("x")
    m.objective = x
    sol = Solution.from_dicts([{"x": 1}], model=m)
    with pytest.raises(ComputationError, match="alpha"):
        sol.cvar(0.0)
