import pytest

from aqmodels import Expression, Vtype

from .common import *  # noqa: F403
from ...utils import (
    assert_offset,
    assert_linear,
    assert_quadratic,
    assert_higher_order,
    assert_higher_order_all,
)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [(3, Vtype.Binary)], indirect=True)
def test_expression_mul_binary_variables(variables):
    x, y, z = variables

    expr = x * y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    result = expr * z

    assert id(expr) != id(result)
    assert isinstance(result, Expression)
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert_linear(result, variables, 0)
    assert_quadratic(result, variables, 0)
    assert_higher_order(result, variables, 0, 2)
    assert_higher_order(result, variables, 1)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [(3, Vtype.Binary)], indirect=True)
def test_expression_rmul_binary_variables(variables):
    x, y, z = variables

    expr = x * y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    result = z * expr

    assert id(expr) != id(result)
    assert isinstance(result, Expression)
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert_linear(result, variables, 0)
    assert_quadratic(result, variables, 0)
    assert_higher_order(result, variables, 0, 2)
    assert_higher_order(result, variables, 1)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [(3, Vtype.Binary)], indirect=True)
def test_expression_mul_binary_variable_twice(variables):
    x, y, z = variables

    expr = x * y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    result = expr * z

    assert id(expr) != id(result)
    assert isinstance(result, Expression)
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert_linear(result, variables, 0)
    assert_quadratic(result, variables, 0)
    assert_higher_order(result, variables, 0, 2)
    assert_higher_order(result, variables, 1)

    result = result * z

    assert id(expr) != id(result)
    assert isinstance(result, Expression)
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert_linear(result, variables, 0)
    assert_quadratic(result, variables, 0)
    assert_higher_order(result, variables, 0, 2)
    assert_higher_order(result, variables, 1)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [(3, Vtype.Binary)], indirect=True)
def test_expression_instancemul_binary_variable_twice(variables):
    x, y, z = variables

    expr = x * y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    id_expr_before = id(expr)
    expr *= z
    expr *= z
    id_expr_after = id(expr)

    assert id_expr_before == id_expr_after
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 3
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 1)
