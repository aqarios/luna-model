import pytest

from aq_models import Model
from aq_models import Environment
from aq_models import Expression


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
