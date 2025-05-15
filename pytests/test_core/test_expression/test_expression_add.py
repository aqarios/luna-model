from itertools import product
from typing import Tuple

import pytest

from aqmodels import Environment, Expression, Variable, Vtype


@pytest.fixture
def variables(request) -> Tuple[Variable, ...]:
    n, vtype = request.param
    with Environment():
        variables = [Variable(f"{i}", vtype=vtype) for i in range(n)]
    return tuple(variables)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    product([3], [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real]),
    indirect=True,
)
def test_expression_add_variable(variables):
    x, y, z = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    result = expr + z
    assert isinstance(result, Expression)
    assert result.num_variables == 3
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1
    assert result.get_linear(z) == 1


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    product([2], [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real]),
    indirect=True,
)
def test_expression_add_number(variables):
    x, y = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr = expr + 2
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_offset() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    product([2], [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real]),
    indirect=True,
)
def test_expression_radd_number(variables):
    x, y = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr = 2 + expr
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_offset() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    product([3], [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real]),
    indirect=True,
)
def test_expression_instanceadd_variable(variables):
    x, y, z = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr += z
    assert isinstance(expr, Expression)
    assert expr.num_variables == 3
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1
    assert expr.get_linear(z) == 1


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    product([3], [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real]),
    indirect=True,
)
def test_expression_instanceadd_variable_twice(variables):
    x, y, z = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr += z
    expr += z
    assert isinstance(expr, Expression)
    assert expr.num_variables == 3
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1
    assert expr.get_linear(z) == 2


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    product([2], [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real]),
    indirect=True,
)
def test_expression_instanceadd_number(variables):
    x, y = variables

    expr = x + y
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr += 2
    assert isinstance(expr, Expression)
    assert expr.num_variables == 2
    assert expr.get_offset() == 2
    assert expr.get_linear(x) == 1
