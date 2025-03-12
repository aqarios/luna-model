import pytest

from aq_models import Constraints
from aq_models import Constraints
from aq_models import Environment
from aq_models import Variable, Vtype


@pytest.fixture
def constraints() -> Constraints:
    with Environment():
        w = Variable("w", vtype=Vtype.Binary)
        x = Variable("x", vtype=Vtype.Spin)
        y = Variable("y", vtype=Vtype.Integer)
        z = Variable("z", vtype=Vtype.Real)

    offset_expr = x * x * 3
    linear_expr = w + x + y + z
    quadratic_expr = w * x + y * z
    ho_expr = w * x * y + w * x * y * z

    constrs = Constraints()
    constrs.add_constraint(offset_expr <= 1)
    constrs.add_constraint(offset_expr == 2)
    constrs.add_constraint(offset_expr >= 3)

    constrs.add_constraint(linear_expr <= 1)
    constrs.add_constraint(linear_expr == 2)
    constrs.add_constraint(linear_expr >= 3)

    constrs.add_constraint(quadratic_expr <= 1)
    constrs.add_constraint(quadratic_expr == 2)
    constrs.add_constraint(quadratic_expr >= 3)

    constrs.add_constraint(ho_expr <= 1)
    constrs.add_constraint(ho_expr == 2)
    constrs.add_constraint(ho_expr >= 3)

    return constrs


def test_expression_serialize(constraints: Constraints):
    data_encoded = constraints.encode()
    data_serialized = constraints.serialize()

    assert (
        data_encoded == data_serialized
    ), ".encode and .serialize do not produce equal results"


def test_expression_serialize_compress(constraints: Constraints):
    data_enc_compressed = constraints.encode(compress=True)
    data_ser_compressed = constraints.serialize(compress=True)

    assert (
        data_enc_compressed == data_ser_compressed
    ), ".encode and .serialize do not produce equal results when using compression"


def test_expression_deserialize_from_encode(constraints: Constraints):
    data_encoded = constraints.encode()
    expr_decoded_a = Constraints.decode(data_encoded)
    expr_deserialized_a = Constraints.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce equal results for the same input data"


def test_expression_deserialize_from_encode_compressed(constraints: Constraints):
    data_encoded = constraints.encode(compress=True)
    expr_decoded_a = Constraints.decode(data_encoded)
    expr_deserialized_a = Constraints.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce equal results for the same input data when compression was used"


def test_expression_deserialized_object_equal(constraints: Constraints):
    data_encoded = constraints.encode()
    expr_decoded_a = Constraints.decode(data_encoded)
    expr_deserialized_a = Constraints.deserialize(data_encoded)

    assert (
        expr_decoded_a == expr_deserialized_a
    ), ".decode and .deserialize do not produce the same object"


def test_expression_deserialized_object_equal_to_initial(constraints: Constraints):
    decoded = Constraints.decode(constraints.encode())
    assert constraints == decoded, "decoded/deserialized object not equal to input"
