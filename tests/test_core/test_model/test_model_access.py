import pytest

from aq_models import Model
from aq_models import Variable
from aq_models import Environment
from aq_models import Expression

from ..utils import (
    assert_linear,
    assert_offset,
)


@pytest.fixture
def model() -> Model:
    with Environment():
        return Model()


@pytest.mark.model
def test_access_name(model: Model):
    name = model.name
    assert type(name) == str
    assert name == "unnamed"


@pytest.mark.model
def test_access_objective(model: Model):
    objective_a = model.objective
    objective_b = model.objective
    assert type(objective_a) == Expression
    assert type(objective_b) == Expression
    assert objective_a == objective_b
    assert model == model


@pytest.mark.model
def test_use_model_environment(model: Model):
    with model.environment:
        _ = Variable("x")


@pytest.mark.model
def test_use_instanceadd_to_model(model: Model):
    with model.environment:
        x = Variable("x")

    model.objective += x
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 1)
    assert model.objective == model.objective
