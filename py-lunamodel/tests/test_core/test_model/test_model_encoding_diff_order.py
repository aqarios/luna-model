import pytest

from luna_model import Bounds, Model, Unbounded, Variable, Vtype


@pytest.fixture()
def model_a():
    model = Model("model_a")
    with model.environment:
        x = Variable("x", vtype=Vtype.SPIN)
        y = Variable("y", vtype=Vtype.INTEGER, bounds=Bounds(lower=Unbounded))
        z = Variable("z", vtype=Vtype.BINARY)
        a = Variable("a", vtype=Vtype.REAL, bounds=Bounds(10, 20))

    model.objective += x + y + z + x * y + x * z + a
    model.constraints += x + y <= 3, "c1"
    model.constraints += z + x >= 5, "c2"
    return model


@pytest.fixture()
def model_b():
    model = Model("model_b")
    with model.environment:
        z = Variable("z", vtype=Vtype.BINARY)
        x = Variable("x", vtype=Vtype.SPIN)
        a = Variable("a", vtype=Vtype.REAL, bounds=Bounds(10, 20))
        y = Variable("y", vtype=Vtype.INTEGER, bounds=Bounds(lower=Unbounded))

    model.objective = y + x + y * x + z * x + z + a
    model.constraints += y + x <= 3, "c1"
    model.constraints += x + z >= 5, "c2"

    return model


def test_same_model_var_order(model_a: Model):
    ser_model_a = model_a.encode()
    ser_model_a2 = model_a.encode()

    assert model_a == model_a
    assert ser_model_a == ser_model_a2


def test_same_model_diff_var_order(model_a: Model, model_b: Model):
    ser_model_a = model_a.encode()
    ser_model_b = model_b.encode()

    assert (model_a == model_a and ser_model_a == ser_model_b) or (model_a != model_b and ser_model_a != ser_model_b)
