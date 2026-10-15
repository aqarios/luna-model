import operator

import pytest

from luna_model import Model


@pytest.fixture()
def model():
    return Model()


@pytest.fixture()
def x(model):
    return model.add_variable("x")


BINARY = [operator.add, operator.sub, operator.mul, operator.le, operator.ge]
INPLACE = [operator.iadd, operator.isub, operator.imul, operator.ipow]


@pytest.mark.parametrize("op", BINARY)
def test_expression_binary_unsupported_raises(x, op):
    with pytest.raises(TypeError):
        op(2 * x, "garbage")


@pytest.mark.parametrize("op", BINARY)
def test_variable_binary_unsupported_raises(x, op):
    with pytest.raises(TypeError):
        op(x, "garbage")


def test_expression_eq_unsupported_is_false(x):
    assert (2 * x == "garbage") is False


@pytest.mark.parametrize("op", INPLACE)
def test_expression_inplace_unsupported_raises(x, op):
    expr = 2 * x
    with pytest.raises(TypeError):
        op(expr, "garbage")
    assert str(expr) == "2 x"


def test_objective_iadd_unsupported_raises(model, x):
    model.objective = 2 * x
    with pytest.raises(TypeError):
        model.objective += "garbage"
    assert str(model.objective) == "2 x"
