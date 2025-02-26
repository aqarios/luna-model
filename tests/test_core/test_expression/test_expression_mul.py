import pytest

from typing import Tuple
from itertools import permutations

from aq_models import Variable
from aq_models import Vtype
from aq_models import Environment
from aq_models import Expression


@pytest.fixture
def variables(request) -> Tuple[Variable, ...]:
    n: int
    vtype: Vtype
    if isinstance(request.param, int):
        n = request.param
        vtype = Vtype.Binary
    else:
        n, vtype = request.param
    with Environment():
        variables = [Variable(f"{i}", vtype=vtype) for i in range(n)]
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


def check_equality(variables, p, f, value):
    permuts = permutations(variables, p)
    base = next(permuts)
    base_value = f(base)
    assert base_value == value
    for permut in permuts:
        assert f(permut) == base_value


def assert_linear(expr, variables, value):
    check_equality(variables, 1, lambda v: expr.get_linear(v[0]), value)


def assert_quadratic(expr, variables, value):
    check_equality(variables, 2, lambda v: expr.get_quadratic(*v), value)


def assert_higher_order(expr, variables, value, p_size=None):
    if not p_size:
        check_equality(variables, len(variables), expr.get_higher_order, value)
    else:
        check_equality(variables, p_size, expr.get_higher_order, value)


def assert_higher_order_all(expr, variables, value):
    for p_size in range(3, len(variables) + 1):
        check_equality(variables, p_size, expr.get_higher_order, value)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_mul_binary_variables(variables):
    x, y, z = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    result = expr * z

    assert id(expr) != id(result)
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert_linear(result, variables, 0)
    assert_quadratic(result, variables, 0)
    assert_higher_order(result, variables, 0, 2)
    assert_higher_order(result, variables, 1)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [(3, Vtype.Spin)], indirect=True)
def test_expression_mul_spin_variables(variables):
    x, y, z = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    result = expr * z

    assert id(expr) != id(result)
    assert type(result) == Expression
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
    assert type(expr) == Expression
    assert expr.num_variables() == 1
    assert expr.get_offset() == 1
    assert expr.get_linear(x) == 0
    assert expr.get_quadratic(x, x) == 0

    result = expr * x

    assert id(expr) != id(result)
    assert type(result) == Expression
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
    assert type(expr) == Expression
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
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_offset() == 0
    assert result.get_linear(x) == 0
    assert result.get_linear(y) == 1
    assert result.get_quadratic(x, x) == 0
    assert result.get_quadratic(y, y) == 0
    assert result.get_quadratic(x, y) == 0
    assert result.get_quadratic(x, y) == expr.get_quadratic(y, x)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_mul_binary_variable_twice(variables):
    x, y, z = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    result = expr * z

    assert id(expr) != id(result)
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert_linear(result, variables, 0)
    assert_quadratic(result, variables, 0)
    assert_higher_order(result, variables, 0, 2)
    assert_higher_order(result, variables, 1)

    result = result * z

    assert id(expr) != id(result)
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_offset() == 0
    assert_linear(result, variables, 0)
    assert_quadratic(result, variables, 0)
    assert_higher_order(result, variables, 0, 2)
    assert_higher_order(result, variables, 1)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [2], indirect=True)
def test_expression_mul_number(variables):
    x, y = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 1)
    assert_higher_order_all(expr, variables, 0)

    id_expr_before = id(expr)
    expr = expr * 2
    id_expr_after = id(expr)

    assert id_expr_before != id_expr_after
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 2)
    assert_higher_order_all(expr, variables, 0)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_instancemul_variable(variables):
    x, y, z = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    id_expr_before = id(expr)
    expr *= z
    id_expr_after = id(expr)

    assert id_expr_before == id_expr_after
    assert type(expr) == Expression
    assert expr.num_variables() == 3
    assert expr.get_offset() == 0
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 1)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [2], indirect=True)
def test_expression_instancemul_number(variables):
    x, y = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 1)
    assert_higher_order_all(expr, variables, 0)

    id_expr_before = id(expr)
    expr *= 2
    id_expr_after = id(expr)

    assert id_expr_before == id_expr_after
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 2)
    assert_higher_order_all(expr, variables, 0)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression_instancemul_binary_variable_twice(variables):
    x, y, z = variables

    expr = x * y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_offset() == 0
    assert_linear(expr, (x, y), 0)
    assert_quadratic(expr, (x, y), 1)
    assert_higher_order_all(expr, (x, y), 0)

    id_expr_before = id(expr)
    expr *= z
    expr *= z
    id_expr_after = id(expr)

    assert id_expr_before == id_expr_after
    assert type(expr) == Expression
    assert expr.num_variables() == 3
    assert expr.get_offset() == 0
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 1)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [4], indirect=True)
def test_expression_mul_expression(variables):
    w, x, y, z = variables
    lhs_variables = (w, x)
    rhs_variables = (y, z)

    expr_lhs = w * x
    id_expr_lhs = id(expr_lhs)

    assert type(expr_lhs) == Expression
    assert expr_lhs.num_variables() == 2
    assert expr_lhs.get_offset() == 0
    assert_linear(expr_lhs, lhs_variables, 0)
    assert_quadratic(expr_lhs, lhs_variables, 1)
    assert_higher_order_all(expr_lhs, lhs_variables, 0)

    expr_rhs = y * z
    id_expr_rhs = id(expr_rhs)

    assert type(expr_rhs) == Expression
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

    assert type(expr) == Expression
    assert expr.num_variables() == 4
    assert expr.get_offset() == 0
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 0, 3)
    assert_higher_order(expr, variables, 1, 4)


@pytest.mark.expression
@pytest.mark.parametrize("variables", [4], indirect=True)
def test_expression_instancemul_expression(variables):
    w, x, y, z = variables
    lhs_variables = (w, x)
    rhs_variables = (y, z)

    expr_lhs = w * x
    id_expr_lhs = id(expr_lhs)

    assert type(expr_lhs) == Expression
    assert expr_lhs.num_variables() == 2
    assert expr_lhs.get_offset() == 0
    assert_linear(expr_lhs, lhs_variables, 0)
    assert_quadratic(expr_lhs, lhs_variables, 1)
    assert_higher_order_all(expr_lhs, lhs_variables, 0)

    expr = y * z
    id_expr_rhs = id(expr)

    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert_linear(expr, rhs_variables, 0)
    assert_quadratic(expr, rhs_variables, 1)
    assert_higher_order_all(expr, rhs_variables, 0)

    assert id_expr_lhs != id_expr_rhs

    # ACTUAL TEST
    expr *= expr_lhs
    id_expr = id(expr)

    assert id_expr == id_expr_rhs
    assert type(expr) == Expression
    assert expr.num_variables() == 4
    assert expr.get_offset() == 0
    assert_linear(expr, variables, 0)
    assert_quadratic(expr, variables, 0)
    assert_higher_order(expr, variables, 0, 2)
    assert_higher_order(expr, variables, 0, 3)
    assert_higher_order(expr, variables, 1, 4)
