import pytest

from aq_models import Model
from aq_models import Variable
from aq_models import Environment


def model_iadd(request) -> Model:
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    comp = request.param
    if comp == "le":
        model.constraints += x + y <= 1
    elif comp == "eq":
        model.constraints += x + y == 1
    elif comp == "ge":
        model.constraints += x + y >= 1

    return model


def model_fadd(request) -> Model:
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    comp = request.param
    if comp == "le":
        model.constraints.add_constraint(x + y <= 1)
    elif comp == "eq":
        model.constraints.add_constraint(x + y == 1)
    elif comp == "ge":
        model.constraints.add_constraint(x + y >= 1)

    return model


@pytest.fixture
def models(request) -> tuple[Model, Model]:
    return (model_iadd(request), model_fadd(request))


@pytest.mark.constraint
def test_model_add_constraint_le():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y <= 1
    assert model.num_constraints() == 1


@pytest.mark.constraint
def test_model_add_constraint_eq():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y == 0
    assert model.num_constraints() == 1


@pytest.mark.constraint
def test_model_add_constraint_ge():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y >= 1
    assert model.num_constraints() == 1


@pytest.mark.constraint
@pytest.mark.parametrize("models", ["le", "eq", "ge"], indirect=True)
def test_model_add_constraint_same(models: tuple[Model, Model]):
    model_a, model_b = models
    assert model_a.constraints == model_b.constraints
