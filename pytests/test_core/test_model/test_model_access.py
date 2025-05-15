import pytest

from aqmodels import Environment, Expression, Model, Sense, Variable
from ..utils import (
    assert_linear,
    assert_offset,
    assert_quadratic,
)


def make_model() -> Model:
    with Environment():
        return Model()


@pytest.fixture
def model() -> Model:
    return make_model()


@pytest.mark.model
def test_access_name(model: Model):
    name = model.name
    assert isinstance(name, str)
    assert name == "unnamed"


@pytest.mark.model
def test_access_objective(model: Model):
    objective_a = model.objective
    objective_b = model.objective
    assert isinstance(objective_a, Expression)
    assert isinstance(objective_b, Expression)
    assert objective_a.is_equal(objective_b)
    assert model == model


@pytest.mark.model
def test_use_model_environment():
    model = make_model()
    with model.environment:
        _ = Variable("x")


@pytest.mark.model
def test_use_instanceadd_bias_to_aq():
    model = make_model()
    with model.environment:
        _ = Variable("x")

    model.objective += 1
    assert_offset(model.objective, 1)


@pytest.mark.model
def test_use_instanceadd_variable_to_aq():
    model = make_model()
    with model.environment:
        x = Variable("x")

    model.objective += x
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 1)


@pytest.mark.model
def test_use_instanceadd_expression_to_aq():
    model = make_model()
    with model.environment:
        x = Variable("x")
        y = Variable("y")

    model.objective += x * y
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 0)
    assert_quadratic(model.objective, (x, y), 1)


@pytest.mark.model
def test_use_set_expression():
    model = make_model()
    with model.environment:
        x = Variable("x")
        y = Variable("y")

    model.set_objective(x * y)
    assert model.sense == Sense.Min
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 0)
    assert_quadratic(model.objective, (x, y), 1)


@pytest.mark.model
def test_use_set_expression_with_sense_min():
    model = make_model()
    with model.environment:
        x = Variable("x")
        y = Variable("y")

    model.set_objective(x * y, Sense.Min)
    assert model.sense == Sense.Min
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 0)
    assert_quadratic(model.objective, (x, y), 1)


@pytest.mark.model
def test_use_set_expression_with_sense_max():
    model = make_model()
    with model.environment:
        x = Variable("x")
        y = Variable("y")

    model.set_objective(x * y, Sense.Max)
    assert model.sense == Sense.Max
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 0)
    assert_quadratic(model.objective, (x, y), 1)
