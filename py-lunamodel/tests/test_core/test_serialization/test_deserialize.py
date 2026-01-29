import pytest

from luna_model import Environment, Model

from .creators import serialized_objects


@pytest.mark.parametrize("initial,serialized_object,class_type", serialized_objects([Environment]))
def test_deserialize_environment(initial, serialized_object, class_type):
    deserialized = class_type.deserialize(serialized_object)
    assert isinstance(deserialized, class_type)
    assert deserialized.equal_contents(initial)


@pytest.mark.parametrize("initial,serialized_object,class_type", serialized_objects([Model]))
def test_deserialize_model(initial, serialized_object, class_type):
    deserialized = class_type.deserialize(serialized_object)
    assert isinstance(deserialized, class_type)
    assert deserialized.equal_contents(initial)
