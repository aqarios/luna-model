import pytest

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression
from aq_models import Vtype


@pytest.mark.variable
@pytest.mark.parametrize("scalar", [1, 2, 3, 1.0, 2.0, 3.0])
def test_mul_variable_and_number(scalar: int):
    with Environment():
        x = Variable("x")

    result = x * scalar
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_linear(x) == scalar
    assert result.get_offset() == 0


@pytest.mark.variable
@pytest.mark.parametrize("scalar", [1, 2, 3, 1.0, 2.0, 3.0])
def test_rmul_variable_and_number(scalar: int):
    with Environment():
        x = Variable("x")

    result = scalar * x
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_linear(x) == scalar
    assert result.get_offset() == 0


@pytest.mark.variable
def test_mul_variables():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    result = x * y
    assert type(result) == Expression
    assert result.num_variables() == 2
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_linear(y) == 0
    assert result.get_quadratic(x, y) == 1
    assert result.get_quadratic(x, y) == result.get_quadratic(y, x)


@pytest.mark.variable
def test_mul_same_variable_binary():
    with Environment():
        x = Variable("x", vtype=Vtype.Binary)

    result = x * x
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 1
    assert result.get_quadratic(x, x) == 0


@pytest.mark.variable
def test_mul_same_variable_spin():
    with Environment():
        x = Variable("x", vtype=Vtype.Spin)

    result = x * x
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_offset() == 1
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 0


@pytest.mark.variable
def test_mul_same_variable_real():
    with Environment():
        x = Variable("x", vtype=Vtype.Real)

    result = x * x
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 1


@pytest.mark.variable
def test_mul_same_variable_integer():
    with Environment():
        x = Variable("x", vtype=Vtype.Integer)

    result = x * x
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_quadratic(x, x) == 1
