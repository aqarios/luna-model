import pytest

from .creators import create_serialized_objects


@pytest.mark.parametrize(
    "initial,serialized_object,class_type", create_serialized_objects()
)
def test_decode(initial, serialized_object, class_type):
    decoded = class_type.decode(serialized_object)
    assert isinstance(decoded, class_type)
    assert decoded == initial
