import pytest

from .creators import create_serializable_objects


@pytest.mark.parametrize("serializable_object", create_serializable_objects())
def test_encode_and_serialize_equality(serializable_object):
    encoded = serializable_object.encode()
    serialized = serializable_object.serialize()
    assert encoded == serialized
