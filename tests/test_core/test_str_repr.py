import string
from contextlib import nullcontext as does_not_raise

import pytest

from aq_models import Variable, Environment, Vtype, Bounds, Expression


@pytest.fixture
def variables(request) -> tuple[Variable, ...]:
    with Environment():
        variables = [Variable(f"{string.ascii_lowercase[i]}") for i in range(request.param)]
    return tuple(variables)


@pytest.mark.str_repr
def test_vtype():
    assert str(Vtype.Real) == "real"
    assert str(Vtype.Binary) == "binary"
    assert str(Vtype.Spin) == "spin"
    assert str(Vtype.Integer) == "int"

    with does_not_raise():
        repr(Vtype.Real)
        repr(Vtype.Binary)
        repr(Vtype.Spin)
        repr(Vtype.Integer)


@pytest.mark.str_repr
def test_variable():
    with Environment():
        # TODO: test cases where only one bound is specified once the functionality is implemented
        a = Variable("a")
        assert str(a) == "a: binary"
        b = Variable("b", vtype=Vtype.Spin)
        assert str(b) == "b: spin"
        c = Variable("c", vtype=Vtype.Integer)
        assert str(c) == "c: int"
        d = Variable("d", vtype=Vtype.Integer, bounds=Bounds(lower=0, upper=10))
        assert str(d) == "d: int { lower: 0, upper: 10 }"
        e = Variable("e", vtype=Vtype.Real)
        assert str(e) == "e: real"
        f = Variable("f", vtype=Vtype.Real, bounds=Bounds(lower=-1.5, upper=1))
        assert str(f) == "f: real { lower: -1.5, upper: 1 }"

        with does_not_raise():
            repr(a)
            repr(b)
            repr(c)
            repr(d)
            repr(e)
            repr(f)


@pytest.mark.str_repr
def test_bounds():
    bounds_1 = Bounds(lower=0, upper=1.5)
    assert str(bounds_1) == "{ lower: 0, upper: 1.5 }"
    bounds_2 = Bounds(lower=-1, upper=10)
    assert str(bounds_2) == "{ lower: -1, upper: 10 }"

    with does_not_raise():
        repr(bounds_1)
        repr(bounds_2)


@pytest.mark.str_repr
def test_model():
    ...


@pytest.mark.str_repr
def test_environment():
    ...


@pytest.mark.str_repr
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression(variables: tuple[Variable, ...]):
    a, b, c = variables

    expressions: list[Expression] = []

    # linear
    expressions.append(a + b)
    assert str(expressions[-1]) == "a + b"
    expressions.append(a + b * -1)
    assert str(expressions[-1]) == "a - b"
    expressions.append(b + a)
    assert str(expressions[-1]) == "a + b"
    expressions.append(a * -1 + b)
    assert str(expressions[-1]) == "-a + b"
    expressions.append(a * 2 + b)
    assert str(expressions[-1]) == "2 * a + b"
    expressions.append(a * 1.5 + b)
    assert str(expressions[-1]) == "1.5 * a + b"
    expressions.append(a * 2 + b + -1)
    assert str(expressions[-1]) == "2 * a + b - 1"
    expressions.append(a * 2 + b + 1)
    assert str(expressions[-1]) == "2 * a + b + 1"
    expressions.append(a * 2 + b + 1.5)
    assert str(expressions[-1]) == "2 * a + b + 1.5"
    expressions.append(a * 2 + b + 0)
    assert str(expressions[-1]) == "2 * a + b"

    # quadratic
    expressions.append(a * b)
    assert str(expressions[-1]) == "a * b"
    expressions.append(a * b * -1)
    assert str(expressions[-1]) == "-a * b"
    expressions.append(a * b * 2 + a)
    assert str(expressions[-1]) == "2 * a * b + a"
    expressions.append(a * b * -2 + a * -1)
    assert str(expressions[-1]) == "-2 * a * b - a"
    expressions.append(a * b * -2 + 5)
    assert str(expressions[-1]) == "-2 * a * b + 5"
    expressions.append(a * c + a * b)
    assert str(expressions[-1]) == "a * b + a * c"

    # higher order
    expressions.append(a * b * c)
    assert str(expressions[-1]) == "a * b * c"
    expressions.append(a * b * 2 * c)
    assert str(expressions[-1]) == "2 * a * b * c"
    expressions.append(a * b * c * -1)
    assert str(expressions[-1]) == "-a * b * c"
    expressions.append(a * b * c + a * b + c + 1)
    assert str(expressions[-1]) == "a * b * c + a * b + c + 1"

    with does_not_raise():
        for expr in expressions:
            repr(expr)
