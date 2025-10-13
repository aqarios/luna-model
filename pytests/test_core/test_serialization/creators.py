from aqmodels import Constraints, Environment, Expression, Model

from ..data import serializable_objects, serialized_objects, serialized_objects_with_env


def create_pickable_objects():
    return serializable_objects([Model])


def create_serializable_objects():
    return serializable_objects([Expression, Constraints, Model, Environment])


def create_serialized_objects():
    return serialized_objects([Model, Environment])


def create_serialized_objects_with_env():
    return serialized_objects_with_env([Constraints])


def create_serialized_objects_with_env_contents():
    return serialized_objects_with_env([Expression])
