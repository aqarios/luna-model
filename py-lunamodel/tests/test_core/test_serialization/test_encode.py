import pytest

from .creators import create_serializable_objects


@pytest.mark.parametrize("serializeable_object", create_serializable_objects())
def test_encode(serializeable_object):
    encoded = serializeable_object.encode()
    assert isinstance(encoded, bytes)
