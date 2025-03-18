import pytest

from .creators import create_serialized_objects_with_env


@pytest.mark.parametrize(
    "initial,serialized_object,class_type,env", create_serialized_objects_with_env()
)
def test_deserialize_with_env(initial, serialized_object, class_type, env):
    decoded = class_type.deserialize(serialized_object, env)
    assert type(decoded) == class_type
    assert decoded == initial
