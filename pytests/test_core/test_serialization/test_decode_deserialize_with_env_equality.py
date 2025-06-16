import pytest

from .creators import create_serialized_objects_with_env


@pytest.mark.parametrize(
    "initial,serialized_object,class_type,env", create_serialized_objects_with_env()
)
def test_decode_and_deserialize_with_env_equality(
    initial, serialized_object, class_type, env
):
    decoded = class_type.decode(serialized_object, env)
    deserialized = class_type.deserialize(serialized_object, env)
    assert isinstance(decoded, class_type)
    assert isinstance(deserialized, class_type)
    assert isinstance(decoded, type(deserialized))
    assert decoded == deserialized
    assert decoded == initial
    assert deserialized.equal_contents(initial)
