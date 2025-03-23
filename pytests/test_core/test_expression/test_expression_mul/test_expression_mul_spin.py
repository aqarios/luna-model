import pytest

from aq_models import Expression, Vtype

from ...utils import (
    assert_higher_order,
    assert_higher_order_all,
    assert_linear,
    assert_offset,
    assert_quadratic,
)
from .common import *  # noqa: F403


@pytest.mark.expression
@pytest.mark.parametrize("variables", [(3, Vtype.Spin)], indirect=True)
def test_expression_mul_spin_variables(variables):
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
@pytest.mark.parametrize("variables", [(3, Vtype.Spin)], indirect=True)
def test_expression_rmul_spin_variables(variables):
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
@pytest.mark.parametrize("variables", [(1, Vtype.Spin)], indirect=True)
def test_expression_mul_same_spin_variable(variables):
    x = variables[0]

    expr = x * x
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 1
    assert expr.get_offset() == 1
    assert expr.get_linear(x) == 0
    assert expr.get_quadratic(x, x) == 0

    result = expr * x

    assert id(expr) != id(result)
    assert isinstance(result, Expression)
    assert result.num_variables() == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 1
    assert result.get_quadratic(x, x) == 0
    assert result.get_higher_order((x, x, x)) == 0


@pytest.mark.expression
@pytest.mark.parametrize("variables", [(2, Vtype.Spin)], indirect=True)
def test_expression_mul_same_spin_variable_larger_index(variables):
    x, y = variables

    expr = y * y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 1
    assert expr.get_offset() == 1
    assert expr.get_linear(x) == 0
    assert expr.get_linear(y) == 0
    assert expr.get_quadratic(x, x) == 0
    assert expr.get_quadratic(y, y) == 0

    assert expr.get_quadratic(x, y) == 0
    assert expr.get_quadratic(x, y) == expr.get_quadratic(y, x)

    result = expr * y

    assert id(expr) != id(result)
    assert isinstance(result, Expression)
    assert result.num_variables() == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_linear(y) == 1
    assert result.get_quadratic(x, x) == 0
    assert result.get_quadratic(y, y) == 0
    assert result.get_quadratic(x, y) == 0
    assert result.get_quadratic(x, y) == expr.get_quadratic(y, x)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [(1, Vtype.Spin)], indirect=True)
def test_mul_multispins(variables):
    x = variables[0]
    expr = x * x * x

    assert isinstance(expr, Expression)
    assert expr.num_variables() == 1
    assert_offset(expr, 0)
    assert_linear(expr, variables, 1)
