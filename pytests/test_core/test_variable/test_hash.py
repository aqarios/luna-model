import pytest

from aqmodels import Environment, Variable


@pytest.mark.variable
def test_variable_hash():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    _ = hash(x)
    _ = hash(y)
