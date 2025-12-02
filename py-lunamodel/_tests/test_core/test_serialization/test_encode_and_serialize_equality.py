import pytest

from .creators import create_serializable_objects


@pytest.mark.parametrize("serializable_object", create_serializable_objects())
@pytest.mark.parametrize("compressed", [True, False])
@pytest.mark.parametrize("level", [0, 1, 2, 3])
def test_encode_and_serialize_equality(
    serializable_object, compressed: bool, level: int
):
    encoded = serializable_object.encode(compress=compressed, level=level)
    serialized = serializable_object.serialize(compress=compressed, level=level)
    assert encoded == serialized
