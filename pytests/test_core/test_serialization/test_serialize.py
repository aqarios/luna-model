import pytest

from .creators import create_serializable_objects


@pytest.mark.parametrize("serializable_object", create_serializable_objects())
@pytest.mark.parametrize("compressed", [True, False])
@pytest.mark.parametrize("level", [0, 1, 2, 3])
def test_serialize(serializable_object, compressed: bool, level: int):
    serialized = serializable_object.serialize(compress=compressed, level=level)
    assert isinstance(serialized, bytes)
