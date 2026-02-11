import pytest

from .creators import create_serializable_objects


@pytest.mark.parametrize("serializable_object", create_serializable_objects())
def test_serialize(serializable_object):
    serialized = serializable_object.serialize()
    assert isinstance(serialized, bytes)
