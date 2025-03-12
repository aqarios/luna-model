import pytest
from ..data import serializeable_objects, serialized_objects
from aq_models import Expression, Constraints, Model, Environment


create_serializeable_objects = lambda: serializeable_objects(
    [Expression, Constraints, Model, Environment]
)

create_serialized_objects = lambda: serialized_objects(
    [Expression, Constraints, Model, Environment]
)


@pytest.mark.parametrize("serializeable_object", create_serializeable_objects())
@pytest.mark.parametrize("compressed", [True, False])
@pytest.mark.parametrize("level", [0, 1, 2, 3])
def test_encode(serializeable_object, compressed: bool, level: int):
    encoded = serializeable_object.encode(compress=compressed, level=level)
    assert type(encoded) == bytes


@pytest.mark.parametrize("serializeable_object", create_serializeable_objects())
@pytest.mark.parametrize("compressed", [True, False])
@pytest.mark.parametrize("level", [0, 1, 2, 3])
def test_serialize(serializeable_object, compressed: bool, level: int):
    serialized = serializeable_object.serialize(compress=compressed, level=level)
    assert type(serialized) == bytes


@pytest.mark.parametrize("serializeable_object", create_serializeable_objects())
@pytest.mark.parametrize("compressed", [True, False])
@pytest.mark.parametrize("level", [0, 1, 2, 3])
def test_encode_and_serialize_equality(
    serializeable_object, compressed: bool, level: int
):
    encoded = serializeable_object.encode(compress=compressed, level=level)
    serialized = serializeable_object.serialize(compress=compressed, level=level)
    assert encoded == serialized


@pytest.mark.parametrize(
    "initial,serialized_object,class_type", create_serialized_objects()
)
def test_decode(initial, serialized_object, class_type):
    decoded = class_type.decode(serialized_object)
    assert type(decoded) == class_type
    assert decoded == initial


@pytest.mark.parametrize(
    "initial,serialized_object,class_type", create_serialized_objects()
)
def test_deserialize(initial, serialized_object, class_type):
    deserialized = class_type.deserialize(serialized_object)
    assert type(deserialized) == class_type
    assert deserialized == initial


@pytest.mark.parametrize(
    "initial,serialized_object,class_type", create_serialized_objects()
)
def test_decode_and_deserialize_equality(initial, serialized_object, class_type):
    decoded = class_type.decode(serialized_object)
    deserialized = class_type.deserialize(serialized_object)
    assert type(decoded) == class_type
    assert type(deserialized) == class_type
    assert type(decoded) == type(deserialized)
    assert decoded == deserialized
    assert decoded == initial
    assert deserialized == initial
