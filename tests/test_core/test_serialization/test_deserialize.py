import pytest

from .creators import create_serialized_objects


@pytest.mark.parametrize(
    "initial,serialized_object,class_type", create_serialized_objects()
)
def test_deserialize(initial, serialized_object, class_type):
    deserialized = class_type.deserialize(serialized_object)
    assert type(deserialized) == class_type
    assert deserialized == initial
