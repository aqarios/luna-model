from ..data import (
    serializable_objects,
    serialized_objects,
    serialized_objects_with_env,
)
from aq_models import Expression, Constraints, Model, Environment

create_serializable_objects = lambda: serializable_objects(
    [Expression, Constraints, Model, Environment]
)

create_serialized_objects = lambda: serialized_objects([Model, Environment])

create_serialized_objects_with_env = lambda: serialized_objects_with_env(
    [Expression, Constraints]
)
