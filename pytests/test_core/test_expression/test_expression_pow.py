from itertools import product
from typing import Tuple

import pytest

from aqmodels import Environment, Variable, Vtype


@pytest.fixture
def variables(request) -> Tuple[Variable, ...]:
    n, vtype = request.param
    with Environment():
        variables = [Variable(f"x_{i}", vtype=vtype) for i in range(n)]
    return tuple(variables)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    product([3], [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real]),
    indirect=True,
)
def test_expression_pow(variables):
    x, y, z = variables

    expr = (x + y + z) ** 2
    expr_manual = (x + y + z) * (x + y + z)
    assert expr.is_equal(expr_manual)


@pytest.mark.expression
@pytest.mark.parametrize(
    "variables",
    product([3], [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real]),
    indirect=True,
)
def test_expression_pow_n1(variables):
    x, y, z = variables

    with pytest.raises(
        ValueError, match="Expected a non-negative number, received: -1"
    ):
        _ = (x + y + z) ** -1
