from aq_models import Constraints, Environment, Expression, Model

from ..data import (
    serializable_objects,
    serialized_objects,
    serialized_objects_with_env,
)


def create_serializable_objects():
    return serializable_objects([Expression, Constraints, Model, Environment])


def create_serialized_objects():
    return serialized_objects([Model, Environment])


def create_serialized_objects_with_env():
    return serialized_objects_with_env([Expression, Constraints])
