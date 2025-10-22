import pytest

from luna_model import Environment, Expression, Variable, Vtype


@pytest.mark.variable
def test_variable_pow_n1():
    with Environment():
        x = Variable("x", vtype=Vtype.Integer)

    with pytest.raises(
        ValueError, match="Expected a non-negative number, received: -1"
    ):
        _ = x**-1


@pytest.mark.variable
def test_variable_pow_0():
    with Environment():
        x = Variable("x", vtype=Vtype.Integer)

    result = x**0
    assert isinstance(result, Expression)
    assert result.num_variables == 0
    # assert result.get_linear(x) == 0 # TODO: this is a panic...
    assert result.get_offset() == 1


@pytest.mark.variable
def test_variable_pow_1():
    with Environment():
        x = Variable("x", vtype=Vtype.Integer)

    result = x**1
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_linear(x) == 1
    assert result.get_offset() == 0


@pytest.mark.variable
def test_variable_pow_2():
    with Environment():
        x = Variable("x", vtype=Vtype.Integer)

    result = x**2
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 1


@pytest.mark.variable
def test_variable_pow_3():
    with Environment():
        x = Variable("x", vtype=Vtype.Integer)

    result = x**3
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 0
    assert result.get_higher_order((x, x, x)) == 1


@pytest.mark.variable
@pytest.mark.parametrize("scalar", list(range(4, 10)))
def test_variable_pow_n(scalar: int):
    with Environment():
        x = Variable("x", vtype=Vtype.Integer)

    result = x**scalar
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 0
    for r in range(3, scalar):
        key = tuple([x for _ in range(r)])
        assert result.get_higher_order(key) == 0
    assert result.get_higher_order(tuple([x for _ in range(scalar)])) == 1
