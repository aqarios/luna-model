import pytest

from typing import Tuple

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression


@pytest.fixture
def variables(request) -> Tuple[Variable, ...]:
    with Environment():
        variables = [Variable(f"{i}") for i in range(request.param)]
    return tuple(variables)


@pytest.fixture
def variable() -> Variable:
    with Environment():
        return Variable("variable")


@pytest.fixture
def expression() -> Expression:
    with Environment():
        a, b = Variable("expression_a"), Variable("expression_b")
    return a + b


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_add_variable(variables):
    x, y, z = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    result = expr + z
    assert isinstance(result, Expression)
    assert result.num_variables() == 3
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1
    assert result.get_linear(z) == 1


@pytest.mark.expression
@pytest.mark.parametrize("variables", [2], indirect=True)
def test_expression_add_number(variables):
    x, y = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr = expr + 2
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_offset() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1


@pytest.mark.expression
@pytest.mark.parametrize("variables", [2], indirect=True)
def test_expression_radd_number(variables):
    x, y = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr = 2 + expr
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_offset() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_instanceadd_variable(variables):
    x, y, z = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr += z
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 3
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1
    assert expr.get_linear(z) == 1


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_instanceadd_variable_twice(variables):
    x, y, z = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr += z
    expr += z
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 3
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1
    assert expr.get_linear(z) == 2


@pytest.mark.expression
@pytest.mark.parametrize("variables", [2], indirect=True)
def test_expression_instanceadd_number(variables):
    x, y = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr += 2
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert expr.get_offset() == 2
    assert expr.get_linear(x) == 1
