import pytest

from .creators import create_serialized_objects


@pytest.mark.parametrize(
    "initial,serialized_object,class_type", create_serialized_objects()
)
def test_decode_and_deserialize_equality(initial, serialized_object, class_type):
    decoded = class_type.decode(serialized_object)
    deserialized = class_type.deserialize(serialized_object)
    assert isinstance(decoded, class_type)
    assert isinstance(deserialized, class_type)
    assert isinstance(decoded, type(deserialized))
    assert decoded == deserialized
    assert decoded == initial
    assert deserialized == initial
