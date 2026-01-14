import pytest
from luna_model import Environment, Expression, Variable, Vtype


@pytest.mark.parametrize("scalar", [1, 2, 3, 1.0, 2.0, 3.0])
def test_mul_variable_and_number(scalar: int):
    with Environment():
        x = Variable("x")

    result = x * scalar
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_linear(x) == scalar
    assert result.get_offset() == 0


@pytest.mark.parametrize("scalar", [1, 2, 3, 1.0, 2.0, 3.0])
def test_rmul_variable_and_number(scalar: int):
    with Environment():
        x = Variable("x")

    result = scalar * x
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_linear(x) == scalar
    assert result.get_offset() == 0


def test_mul_variables():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    result = x * y
    assert isinstance(result, Expression)
    assert result.num_variables == 2
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_linear(y) == 0
    assert result.get_quadratic(x, y) == 1
    assert result.get_quadratic(x, y) == result.get_quadratic(y, x)


def test_mul_same_variable_binary():
    with Environment():
        x = Variable("x", vtype=Vtype.BINARY)

    result = x * x
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 1
    assert result.get_quadratic(x, x) == 0


def test_mul_same_variable_spin():
    with Environment():
        x = Variable("x", vtype=Vtype.SPIN)

    result = x * x
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_offset() == 1
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 0


def test_mul_same_variable_real():
    with Environment():
        x = Variable("x", vtype=Vtype.REAL)

    result = x * x
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 1


def test_mul_same_variable_integer():
    with Environment():
        x = Variable("x", vtype=Vtype.INTEGER)

    result = x * x
    assert isinstance(result, Expression)
    assert result.num_variables == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 1
