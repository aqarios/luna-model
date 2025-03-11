import pytest

from aq_models import Model
from aq_models import Environment
from aq_models import Variable, Vtype


@pytest.fixture
def model() -> Model:
    with Environment():
        model = Model()
        w = Variable("w", vtype=Vtype.Binary)
        x = Variable("x", vtype=Vtype.Spin)
        y = Variable("y", vtype=Vtype.Integer)
        z = Variable("z", vtype=Vtype.Real)

    offset_expr = x * x * 3
    linear_expr = w + x + y + z
    quadratic_expr = w * x + y * z
    ho_expr = w * x * y + w * x * y * z

    model.objective = offset_expr + linear_expr + quadratic_expr + ho_expr

    model.constraints.add_constraint(offset_expr <= 1)
    model.constraints.add_constraint(offset_expr == 2)
    model.constraints.add_constraint(offset_expr >= 3)
    model.constraints.add_constraint(linear_expr <= 1)
    model.constraints.add_constraint(linear_expr == 2)
    model.constraints.add_constraint(linear_expr >= 3)
    model.constraints.add_constraint(quadratic_expr <= 1)
    model.constraints.add_constraint(quadratic_expr == 2)
    model.constraints.add_constraint(quadratic_expr >= 3)
    model.constraints.add_constraint(ho_expr <= 1)
    model.constraints.add_constraint(ho_expr == 2)
    model.constraints.add_constraint(ho_expr >= 3)

    return model


def test_expression_serialize(model: Model):
    data_encoded = model.encode()
    data_serialized = model.serialize()

    assert (
        data_encoded == data_serialized
    ), ".encode and .serialize do not produce equal results"


def test_expression_serialize_compress(model: Model):
    data_enc_compressed = model.encode(compress=True)
    data_ser_compressed = model.serialize(compress=True)

    assert (
        data_enc_compressed == data_ser_compressed
    ), ".encode and .serialize do not produce equal results when using compression"


def test_expression_deserialize_from_encode(model: Model):
    data_encoded = model.encode()
    expr_decoded_a = Model.decode(data_encoded)
    expr_deserialized_a = Model.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce equal results for the same input data"


def test_expression_deserialize_from_encode_compressed(model: Model):
    data_encoded = model.encode(compress=True)
    expr_decoded_a = Model.decode(data_encoded)
    expr_deserialized_a = Model.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce equal results for the same input data when compression was used"


def test_expression_deserialized_object_equal(model: Model):
    data_encoded = model.encode()
    expr_decoded_a = Model.decode(data_encoded)
    expr_deserialized_a = Model.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce the same object"


def test_expression_deserialized_object_equal_to_initial(model: Model):
    decoded = Model.decode(model.encode())
    assert model == decoded, "decoded/deserialized object not equal to input"
