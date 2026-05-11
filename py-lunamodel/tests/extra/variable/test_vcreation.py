import itertools

import pytest

from luna_model import Bounds, Environment, Unbounded, Variable, Vtype

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


@pytest.mark.parametrize("vtype", [Vtype.INTEGER, Vtype.REAL])
@pytest.mark.parametrize("lower", [-2.0, 0.0, 3.0, Unbounded])
def test_creation_with_lower_only(vtype, lower):
    with Environment():
        v = Variable("v", vtype, lower=lower)
    assert v.bounds.lower == lower


@pytest.mark.parametrize("vtype", [Vtype.INTEGER, Vtype.REAL])
@pytest.mark.parametrize("upper", [-1.0, 0.0, 4.0, Unbounded])
def test_creation_with_upper_only(vtype, upper):
    with Environment():
        v = Variable("v", vtype, upper=upper)
    assert v.bounds.upper == upper


@pytest.mark.parametrize("vtype", [Vtype.INTEGER, Vtype.REAL])
@pytest.mark.parametrize("lower", [-3.0, 0.0, Unbounded])
@pytest.mark.parametrize("upper", [1.0, 5.0, Unbounded])
def test_creation_with_lower_and_upper(vtype, lower, upper):
    with Environment():
        v = Variable("v", vtype, lower=lower, upper=upper)
    assert v.bounds.lower == lower
    assert v.bounds.upper == upper


@pytest.mark.parametrize(
    "kwargs",
    [
        {"bounds": Bounds(0, 5), "lower": 0},
        {"bounds": Bounds(0, 5), "upper": 5},
        {"bounds": Bounds(0, 5), "lower": 0, "upper": 5},
        {"bounds": Bounds(0, 5), "lower": Unbounded},
        {"bounds": Bounds(0, 5), "upper": Unbounded},
        {"bounds": Bounds(Unbounded, Unbounded), "lower": 0},
    ],
)
def test_creation_bounds_with_lower_or_upper_raises(kwargs):
    with Environment(), pytest.raises(TypeError):
        _ = Variable("v", Vtype.INTEGER, **kwargs)


@pytest.mark.parametrize("vtype", [Vtype.INTEGER, Vtype.REAL])
@pytest.mark.parametrize(
    "kwargs",
    [
        {"bounds": None},
        {"lower": None},
        {"upper": None},
        {"lower": None, "upper": None},
        {"bounds": None, "lower": None},
        {"bounds": None, "upper": None},
        {"bounds": None, "lower": None, "upper": None},
    ],
)
def test_creation_explicit_none_matches_defaults(vtype, kwargs):
    """Explicit None for bounds/lower/upper must behave identically to omitting them."""
    with Environment():
        baseline = Variable("baseline", vtype)
        v = Variable("v", vtype, **kwargs)
    assert v.bounds.lower == baseline.bounds.lower
    assert v.bounds.upper == baseline.bounds.upper


@pytest.mark.parametrize("vtype", [Vtype.INTEGER, Vtype.REAL])
@pytest.mark.parametrize("lower", [-2.0, 0.0, 3.0, Unbounded])
def test_creation_bounds_none_with_lower(vtype, lower):
    """bounds=None alongside lower= must not trigger the mutual-exclusion check."""
    with Environment():
        v = Variable("v", vtype, bounds=None, lower=lower)
    assert v.bounds.lower == lower


@pytest.mark.parametrize("vtype", [Vtype.INTEGER, Vtype.REAL])
@pytest.mark.parametrize("upper", [-1.0, 0.0, 4.0, Unbounded])
def test_creation_bounds_none_with_upper(vtype, upper):
    """bounds=None alongside upper= must not trigger the mutual-exclusion check."""
    with Environment():
        v = Variable("v", vtype, bounds=None, upper=upper)
    assert v.bounds.upper == upper


@pytest.mark.parametrize("vtype", [Vtype.INTEGER, Vtype.REAL])
@pytest.mark.parametrize(
    "kwargs",
    [
        {"bounds": Bounds(0, 5), "lower": None},
        {"bounds": Bounds(0, 5), "upper": None},
        {"bounds": Bounds(0, 5), "lower": None, "upper": None},
    ],
)
def test_creation_bounds_with_explicit_none_lower_upper(vtype, kwargs):
    """bounds=<Bounds> with lower=None/upper=None must not raise."""
    with Environment():
        v = Variable("v", vtype, **kwargs)
    assert v.bounds.lower == 0
    assert v.bounds.upper == 5
