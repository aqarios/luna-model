import itertools

import pytest

from luna_model import Bounds, Unbounded, Vtype

bound_value = [-1.0, 0.0, 1.0, Unbounded, None]


@pytest.mark.parametrize("lower, upper", itertools.product(bound_value, bound_value))
def test_creation(lower, upper):
    _ = Bounds(lower, upper)


@pytest.mark.parametrize("lower, upper", itertools.product(bound_value, bound_value))
def test_access(lower, upper):
    b = Bounds(lower, upper)
    assert lower == b.lower
    assert upper == b.upper


@pytest.mark.parametrize("vtype", [Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL])
def test_default(vtype):
    default: Bounds
    match vtype:
        case Vtype.BINARY:
            default = Bounds.binary()
        case Vtype.SPIN:
            default = Bounds.spin()
        case Vtype.INTEGER:
            default = Bounds.integer()
        case Vtype.REAL:
            default = Bounds.real()
        case _:
            raise TypeError(f"unexpected vtype: {vtype}")

    assert default == Bounds.default(vtype)
