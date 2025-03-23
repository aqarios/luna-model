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
@pytest.mark.parametrize(
    "variables",
    [
        (2, Vtype.Binary),
        (2, Vtype.Spin),
        (2, Vtype.Integer),
        (2, Vtype.Real),
    ],
    indirect=True,
)
def test_expression_mul_number(variables):
    x, y = variables

    expr = x * y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 1)
    assert_higher_order_all(expr, variables, 0)

    id_expr_before = id(expr)
    expr = expr * 2
    id_expr_after = id(expr)

    assert id_expr_before != id_expr_after
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 2)
    assert_higher_order_all(expr, variables, 0)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    [
        (2, Vtype.Binary),
        (2, Vtype.Spin),
        (2, Vtype.Integer),
        (2, Vtype.Real),
    ],
    indirect=True,
)
def test_expression_rmul_number(variables):
    x, y = variables

    expr = x * y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 1)
    assert_higher_order_all(expr, variables, 0)

    id_expr_before = id(expr)
    expr = 2 * expr
    id_expr_after = id(expr)

    assert id_expr_before != id_expr_after
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 2)
    assert_higher_order_all(expr, variables, 0)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    [
        (3, Vtype.Binary),
        (3, Vtype.Spin),
        (3, Vtype.Integer),
        (3, Vtype.Real),
    ],
    indirect=True,
)
def test_expression_instancemul_variable(variables):
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
    id_expr_after = id(expr)

    assert id_expr_before == id_expr_after
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 3
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 1)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    [
        (2, Vtype.Binary),
        (2, Vtype.Spin),
        (2, Vtype.Integer),
        (2, Vtype.Real),
    ],
    indirect=True,
)
def test_expression_instancemul_number(variables):
    x, y = variables

    expr = x * y
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 1)
    assert_higher_order_all(expr, variables, 0)

    id_expr_before = id(expr)
    expr *= 2
    id_expr_after = id(expr)

    assert id_expr_before == id_expr_after
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 2)
    assert_higher_order_all(expr, variables, 0)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    [
        (4, Vtype.Binary),
        (4, Vtype.Spin),
        (4, Vtype.Integer),
        (4, Vtype.Real),
    ],
    indirect=True,
)
def test_expression_mul_expression(variables):
    w, x, y, z = variables
    lhs_variables = (w, x)
    rhs_variables = (y, z)

    expr_lhs = w * x
    id_expr_lhs = id(expr_lhs)

    assert isinstance(expr_lhs, Expression)
    assert expr_lhs.num_variables() == 2
    assert expr_lhs.get_offset() == 0
    assert_linear(expr_lhs, lhs_variables, 0)
    assert_quadratic(expr_lhs, lhs_variables, 1)
    assert_higher_order_all(expr_lhs, lhs_variables, 0)

    expr_rhs = y * z
    id_expr_rhs = id(expr_rhs)

    assert isinstance(expr_rhs, Expression)
    assert expr_rhs.num_variables() == 2
    assert_linear(expr_rhs, rhs_variables, 0)
    assert_quadratic(expr_rhs, rhs_variables, 1)
    assert_higher_order_all(expr_rhs, rhs_variables, 0)

    assert id_expr_lhs != id_expr_rhs

    # ACTUAL TEST
    expr = expr_lhs * expr_rhs
    id_expr = id(expr)

    assert id_expr != id_expr_lhs
    assert id_expr != id_expr_rhs

    assert isinstance(expr, Expression)
    assert expr.num_variables() == 4
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 0, 3)
    assert_higher_order(expr, variables, 1, 4)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    [
        (4, Vtype.Binary),
        (4, Vtype.Spin),
        (4, Vtype.Integer),
        (4, Vtype.Real),
    ],
    indirect=True,
)
def test_expression_instancemul_expression(variables):
    w, x, y, z = variables
    lhs_variables = (w, x)
    rhs_variables = (y, z)

    expr_lhs = w * x
    id_expr_lhs = id(expr_lhs)

    assert isinstance(expr_lhs, Expression)
    assert expr_lhs.num_variables() == 2
    assert expr_lhs.get_offset() == 0
    assert_linear(expr_lhs, lhs_variables, 0)
    assert_quadratic(expr_lhs, lhs_variables, 1)
    assert_higher_order_all(expr_lhs, lhs_variables, 0)

    expr = y * z
    id_expr_rhs = id(expr)

    assert isinstance(expr, Expression)
    assert expr.num_variables() == 2
    assert_linear(expr, rhs_variables, 0)
    assert_quadratic(expr, rhs_variables, 1)
    assert_higher_order_all(expr, rhs_variables, 0)

    assert id_expr_lhs != id_expr_rhs

    # ACTUAL TEST
    expr *= expr_lhs
    id_expr = id(expr)

    assert id_expr == id_expr_rhs
    assert isinstance(expr, Expression)
    assert expr.num_variables() == 4
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 0, 3)
    assert_higher_order(expr, variables, 1, 4)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    [
        (3, Vtype.Binary),
        (3, Vtype.Spin),
        (3, Vtype.Integer),
        (3, Vtype.Real),
    ],
    indirect=True,
)
def test_unordered_mul_to_expression(variables):
    x, y, z = variables
    expr = x * z * y

    assert isinstance(expr, Expression)
    assert expr.num_variables() == 3
    assert_offset(expr, 0)
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 1, 3)
