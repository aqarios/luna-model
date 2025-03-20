import pytest

from .creators import create_serializable_objects


@pytest.mark.parametrize("serializeable_object", create_serializable_objects())
@pytest.mark.parametrize("compressed", [True, False])
@pytest.mark.parametrize("level", [0, 1, 2, 3])
def test_encode(serializeable_object, compressed: bool, level: int):
    encoded = serializeable_object.encode(compress=compressed, level=level)
    assert isinstance(encoded, bytes)
