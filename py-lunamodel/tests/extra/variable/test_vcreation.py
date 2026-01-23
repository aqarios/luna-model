import pytest
import itertools

from luna_model import Variable, Vtype, Bounds, Unbounded, Environment


bound_value = [-1.0, 0.0, 1.0, Unbounded, None]
bounds = [Bounds(lo, up) for (lo, up) in itertools.product(bound_value, bound_value)]
vtypes = [Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL]


@pytest.mark.parametrize("vtype", vtypes)
def test_creation(vtype):
    with Environment():
        v = Variable("v", vtype)
    assert v.id == 0
    assert v.name == "v"
    assert vtype == v.vtype
    default = Bounds.default(vtype)
    assert default.lower == v.bounds.lower
    assert default.upper == v.bounds.upper


@pytest.mark.parametrize("vtype", [Vtype.INTEGER, Vtype.REAL])
@pytest.mark.parametrize("bounds", bounds)
def test_creation_with_bounds(vtype, bounds):
    with Environment():
        v = Variable("v", vtype, bounds)
    assert v.id == 0
    assert v.name == "v"
    assert vtype == v.vtype

    upper = Unbounded if bounds.upper is None else bounds.upper

    lower: float | Unbounded
    if vtype in [Vtype.INTEGER, Vtype.REAL] and bounds.lower is None:
        lower = 0
    else:
        lower = Unbounded if bounds.lower is None else bounds.lower

    assert lower == v.bounds.lower
    assert upper == v.bounds.upper
