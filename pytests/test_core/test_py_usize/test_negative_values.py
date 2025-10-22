from contextlib import nullcontext as does_not_raise

import pytest

from luna_model import Environment, Solution, Variable


def test_negative_value_normal_method():
    with Environment():
        _ = Variable("x")
        sol = Solution.from_dict({"x": 1})
    with pytest.raises(TypeError):
        _ = sol.print(max_lines=1.0)  # noqa
    with pytest.raises(TypeError):
        _ = sol.print(max_lines="foo")  # noqa
    with pytest.raises(
        ValueError, match="Expected a non-negative number, received: -1"
    ):
        _ = sol.print(max_lines=-1)
    with pytest.raises(
        ValueError, match="`max_lines` needs to be at least 1; actual value: 0"
    ):
        _ = sol.print(max_lines=0)
    with does_not_raise():
        _ = sol.print(max_lines=1)


def test_negative_value_slot_method():
    with Environment():
        x = Variable("x")
    with pytest.raises(TypeError):
        _ = x**1.0  # noqa
    with pytest.raises(TypeError):
        _ = x ** "foo"  # noqa
    with pytest.raises(
        ValueError, match="Expected a non-negative number, received: -1"
    ):
        _ = x ** (-1)
    with does_not_raise():
        _ = (x**i for i in range(10))


def test_negative_values_getitem():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        sol = Solution.from_dicts([{x: 1, y: 0}, {x: 1, y: 1}])
    samples = sol.samples
    with pytest.raises(TypeError):
        _ = samples["foo"]  # noqa
    with pytest.raises(TypeError):
        _ = samples[1.0]  # noqa
    with pytest.raises(TypeError):
        _ = samples[(0, 1.0)]  # noqa
    with pytest.raises(TypeError):
        _ = samples[(0, 0, 0)]  # noqa
    with pytest.raises(
        ValueError, match="Expected a non-negative number, received: -1"
    ):
        _ = samples[-1]
    with pytest.raises(
        ValueError, match="Expected a non-negative number, received: -1"
    ):
        _ = samples[(-1, -2)]
    with pytest.raises(
        ValueError, match="Expected a non-negative number, received: -1"
    ):
        _ = samples[(0, -1)]
    with does_not_raise():
        _ = samples[0]
        _ = samples[(0, 0)]
