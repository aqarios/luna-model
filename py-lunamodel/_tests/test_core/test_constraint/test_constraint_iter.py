import pytest
from luna_model import ConstraintCollection, Environment, Variable


@pytest.fixture()
def constraint_collection() -> ConstraintCollection:
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")

    collection = ConstraintCollection()
    collection += x + y <= 2, "named"
    collection += y * z + x >= 10  # unnamed => c1
    collection += y >= 1  # unnamed 2 => c2
    collection += y * z * x == 10, "named 2"
    return collection


def test_collection_iter(constraint_collection: ConstraintCollection):
    for name, constraint in constraint_collection.items():
        assert name == constraint.name
        assert constraint == constraint
