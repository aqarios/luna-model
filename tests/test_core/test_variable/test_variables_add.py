import pytest

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression


@pytest.mark.variable
@pytest.mark.parametrize("scalar", [1, 2, 3])
def test_add_variable_and_number(scalar: int):
    with Environment():
        x = Variable("x")

    result = x + scalar
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_linear(x) == scalar


@pytest.mark.variable
def test_add_two_variables():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    result = x + y
    assert type(result) == Expression
    assert result.num_variables() == 2
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1


@pytest.mark.variable
def test_add_two_variables_unordered():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    result = y + x
    assert type(result) == Expression
    assert result.num_variables() == 2
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1


@pytest.mark.variable
def test_add_last_two_variables():
    with Environment():
        _ = Variable("x_ignore")
        _ = Variable("y_ignore")
        x = Variable("x")
        y = Variable("y")

    result = y + x
    assert type(result) == Expression
    assert result.num_variables() == 2
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1


@pytest.mark.variable
def test_add_any_and_last_variables():
    with Environment():
        _ = Variable("x_ignore")
        x = Variable("x")
        _ = Variable("y_ignore")
        y = Variable("y")

    result = y + x
    assert type(result) == Expression
    assert result.num_variables() == 2
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1


@pytest.mark.variable
def test_variable_add_expression():
    with Environment():
        x, y, z = Variable("x"), Variable("y"), Variable("z")
    expr = x + y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    result = z + expr
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1
    assert result.get_linear(z) == 1
