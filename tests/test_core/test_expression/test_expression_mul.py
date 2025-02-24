import pytest

from typing import Tuple

from aq_models import Variable
from aq_models import Vtype
from aq_models import Environment
from aq_models import Expression


@pytest.fixture
def variables(request) -> Tuple[Variable, ...]:
    with Environment():
        variables = [Variable(f"{i}", vtype=Vtype.Binary) for i in range(request.param)]
    return tuple(variables)


@pytest.fixture
def variable() -> Variable:
    with Environment():
        return Variable("variable")


@pytest.fixture
def expression() -> Expression:
    with Environment():
        a, b = Variable("expression_a"), Variable("expression_b")
    return a * b


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_mul_variable(variables):
    x, y, z = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert expr.get_linear(x) == 0
    assert expr.get_linear(y) == 0
    assert expr.get_quadratic(x, y) == 1
    assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)

    result = expr * z
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_linear(y) == 0
    assert result.get_linear(z) == 0
    assert result.get_quadratic(x, y) == 0
    assert result.get_quadratic(x, y) == result.get_quadratic(y, x)
    assert result.get_quadratic(x, z) == 0
    assert result.get_quadratic(x, z) == result.get_quadratic(z, x)
    assert result.get_quadratic(y, z) == 0
    assert result.get_quadratic(y, z) == result.get_quadratic(z, y)
    assert result.get_higher_order((x, y, z)) == 1
    assert result.get_higher_order((x, z, y)) == 1
    assert result.get_higher_order((y, x, z)) == 1
    assert result.get_higher_order((y, z, x)) == 1
    assert result.get_higher_order((z, x, y)) == 1
    assert result.get_higher_order((z, y, x)) == 1


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_mul_binary_variable_twice(variables):
    x, y, z = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert expr.get_linear(x) == 0
    assert expr.get_linear(y) == 0
    assert expr.get_quadratic(x, y) == 1
    assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)

    result = expr * z
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_linear(y) == 0
    assert result.get_linear(z) == 0
    assert result.get_quadratic(x, y) == 0
    assert result.get_quadratic(x, y) == result.get_quadratic(y, x)
    assert result.get_quadratic(x, z) == 0
    assert result.get_quadratic(x, z) == result.get_quadratic(z, x)
    assert result.get_quadratic(y, z) == 0
    assert result.get_quadratic(y, z) == result.get_quadratic(z, y)
    assert result.get_higher_order((x, y, z)) == 1
    assert result.get_higher_order((x, z, y)) == 1
    assert result.get_higher_order((y, x, z)) == 1
    assert result.get_higher_order((y, z, x)) == 1
    assert result.get_higher_order((z, x, y)) == 1
    assert result.get_higher_order((z, y, x)) == 1

    result = expr * z
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_linear(y) == 0
    assert result.get_linear(z) == 0
    assert result.get_quadratic(x, y) == 0
    assert result.get_quadratic(x, y) == result.get_quadratic(y, x)
    assert result.get_quadratic(x, z) == 0
    assert result.get_quadratic(x, z) == result.get_quadratic(z, x)
    assert result.get_quadratic(y, z) == 0
    assert result.get_quadratic(y, z) == result.get_quadratic(z, y)
    assert result.get_higher_order((x, y, z)) == 1
    assert result.get_higher_order((x, z, y)) == 1
    assert result.get_higher_order((y, x, z)) == 1
    assert result.get_higher_order((y, z, x)) == 1
    assert result.get_higher_order((z, x, y)) == 1
    assert result.get_higher_order((z, y, x)) == 1


@pytest.mark.expression
@pytest.mark.parametrize("variables", [2], indirect=True)
def test_expression_mul_number(variables):
    x, y = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert expr.get_linear(x) == 0
    assert expr.get_linear(y) == 0
    assert expr.get_quadratic(x, y) == 1
    assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)

    expr = expr * 2
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert expr.get_linear(x) == 0
    assert expr.get_linear(y) == 0
    assert expr.get_quadratic(x, y) == 2
    assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_instancemul_variable(variables):
    x, y, z = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert expr.get_linear(x) == 0
    assert expr.get_linear(y) == 0
    assert expr.get_quadratic(x, y) == 1
    assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)

    expr *= z
    assert type(expr) == Expression
    assert expr.num_variables() == 3
    assert expr.get_offset() == 0
    assert expr.get_linear(x) == 0
    assert expr.get_linear(y) == 0
    assert expr.get_linear(z) == 0
    assert expr.get_quadratic(x, y) == 0
    assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)
    assert expr.get_quadratic(x, z) == 0
    assert expr.get_quadratic(x, z) == expr.get_quadratic(z, x)
    assert expr.get_quadratic(y, z) == 0
    assert expr.get_quadratic(y, z) == expr.get_quadratic(z, y)
    assert expr.get_higher_order((x, y, z)) == 1
    assert expr.get_higher_order((x, z, y)) == 1
    assert expr.get_higher_order((y, x, z)) == 1
    assert expr.get_higher_order((y, z, x)) == 1
    assert expr.get_higher_order((z, x, y)) == 1
    assert expr.get_higher_order((z, y, x)) == 1


# @pytest.mark.expression
# @pytest.mark.parametrize("variables", [3], indirect=True)
# def test_expression_instancemul_variable_twice(variables):
#     x, y, z = variables
#
#     expr = x * y
#     assert type(expr) == Expression
#     assert expr.num_variables() == 2
#     assert expr.get_offset() == 0
#     assert expr.get_linear(x) == 0
#     assert expr.get_linear(y) == 0
#     assert expr.get_quadratic(x, y) == 1
#     assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)
#
#     expr *= z
#     expr *= z
#     assert type(expr) == Expression
#     assert expr.num_variables() == 3
#     assert expr.get_linear(x) == 1
#     assert expr.get_linear(y) == 1
#     assert expr.get_linear(z) == 2
#
#
# @pytest.mark.expression
# @pytest.mark.parametrize("variables", [2], indirect=True)
# def test_expression_instancemul_number(variables):
#     x, y = variables
#
#     expr = x * y
#     assert type(expr) == Expression
#     assert expr.num_variables() == 2
#     assert expr.get_offset() == 0
#     assert expr.get_linear(x) == 0
#     assert expr.get_linear(y) == 0
#     assert expr.get_quadratic(x, y) == 1
#     assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)
#
#     expr *= 2
#     assert type(expr) == Expression
#     assert expr.num_variables() == 2
#     assert expr.get_offset() == 2
#     assert expr.get_linear(x) == 1
