import pytest

from typing import Tuple

from aq_models import Variable, Environment, Expression


@pytest.fixture
def variables(request) -> Tuple[Variable, ...]:
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
