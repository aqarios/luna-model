import pytest

from luna_model import Solution, Model, Vtype
from luna_model.errors import LunaModelError


def test_cast_near_integral():
    m = Model("probe")
    x = m.add_variable("x", vtype=Vtype.INTEGER)
    b = m.add_variable("b", vtype=Vtype.BINARY)
    for x, b in [(2.9999999998, 0.9999999998), (2.0000000001, 1.0000000001), (1.9999999999, 0.0000000001)]:
        _ = Solution.from_dict({"x": x, "b": b}, model=m, tol=1e-8).samples[0].to_dict()


def test_cast_near_integral_failed():
    m = Model("probe")
    x = m.add_variable("x", vtype=Vtype.INTEGER)
    b = m.add_variable("b", vtype=Vtype.BINARY)
    with pytest.raises(LunaModelError):
        for x, b in [(2.98, 0.5998)]:
            _ = Solution.from_dict({"x": x, "b": b}, model=m).samples[0].to_dict()


def test_float_comparisons_allow_tolerance():
    m = Model("probe")
    x = m.add_variable("x", vtype=Vtype.REAL)
    m.add_constraint(x == 1.0, "eq")
    m.add_constraint(x <= 1.0, "le")
    m.add_constraint(x >= 1.0, "ge")

    upper_near = Solution.from_dict({"x": 1.0 + 1e-7}, model=m)[0]
    assert upper_near.constraints == {"eq": True, "le": True, "ge": True}

    lower_near = Solution.from_dict({"x": 1.0 - 1e-7}, model=m)[0]
    assert lower_near.constraints == {"eq": True, "le": True, "ge": True}

    upper_far = Solution.from_dict({"x": 1.0 + 1e-5}, model=m)[0]
    assert upper_far.constraints == {"eq": False, "le": False, "ge": True}

    lower_far = Solution.from_dict({"x": 1.0 - 1e-5}, model=m)[0]
    assert lower_far.constraints == {"eq": False, "le": True, "ge": False}
