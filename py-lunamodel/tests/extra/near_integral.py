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
        for x, b in [(2.9999999998, 0.5999999998)]:
            _ = Solution.from_dict({"x": x, "b": b}, model=m).samples[0].to_dict()
