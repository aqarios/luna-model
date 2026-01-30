import pytest

from luna_model import Environment, Model

from .utils import serialized_objects


@pytest.mark.parametrize("initial,serialized_object,class_type", serialized_objects([Environment]))
def test_decode_and_deserialize_equality_environment(initial, serialized_object, class_type):
    decoded = class_type.decode(serialized_object)
    deserialized = class_type.deserialize(serialized_object)
    assert isinstance(decoded, class_type)
    assert isinstance(deserialized, class_type)
    assert isinstance(decoded, type(deserialized))
    assert decoded.equal_contents(deserialized)
    assert decoded.equal_contents(initial)
    assert deserialized.equal_contents(initial)


@pytest.mark.parametrize("initial,serialized_object,class_type", serialized_objects([Model]))
def test_decode_and_deserialize_equality_model(initial, serialized_object, class_type):
    decoded = class_type.decode(serialized_object)
    deserialized = class_type.deserialize(serialized_object)
    assert isinstance(decoded, class_type)
    assert isinstance(deserialized, class_type)
    assert isinstance(decoded, type(deserialized))
    assert decoded.equal_contents(deserialized)
    assert decoded.equal_contents(initial)
    assert deserialized.equal_contents(initial)
