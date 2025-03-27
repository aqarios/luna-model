from itertools import product
import pytest

from typing import Tuple

from aqmodels import Variable, Environment, Vtype


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
def test_expression_negate(variables):
    x, y, z = variables

    expr = x + y + z
    expr_neg = -x - y - z
    assert -expr == expr_neg
