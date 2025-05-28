from typing import Tuple

import pytest

from aqmodels import Environment, Expression, Variable


@pytest.fixture
def variables(request) -> Tuple[Variable, ...]:
    n, vtype = request.param
    with Environment():
        variables = [Variable(f"x_{i}", vtype=vtype) for i in range(n)]
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
