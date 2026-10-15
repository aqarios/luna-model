import pytest

from luna_model import ConstraintCollection, Model


def names(coll):
    return [name for name, _ in coll.items()]


@pytest.fixture()
def model():
    return Model()


@pytest.fixture()
def xy(model):
    return model.add_variable("x"), model.add_variable("y")


def test_iadd_collection_with_prefix(model, xy):
    x, y = xy
    coll = ConstraintCollection()
    coll += x <= 1, "a"
    coll += y <= 1, "b"
    model.constraints += (coll, "p")
    assert names(model.constraints) == ["p_a", "p_b"]


def test_iadd_collection_without_prefix(model, xy):
    x, _ = xy
    coll = ConstraintCollection()
    coll += x <= 1, "a"
    model.constraints += coll
    assert names(model.constraints) == ["a"]


def test_iadd_sequence_with_base_name(model, xy):
    x, y = xy
    model.constraints += ([x <= 1, y <= 1], "row")
    assert names(model.constraints) == ["row_0", "row_1"]


def test_iadd_tuple_of_constraints(model, xy):
    x, y = xy
    model.constraints += (x <= 1, y <= 1)
    assert len(model.constraints) == 2


def test_iadd_named_constraint(model, xy):
    x, _ = xy
    model.constraints += (x <= 1, "cap")
    assert names(model.constraints) == ["cap"]


@pytest.mark.parametrize("other", ["garbage", 42, None, [1, 2]])
def test_iadd_unsupported_raises(model, other):
    with pytest.raises(TypeError):
        model.constraints += other
    assert len(model.constraints) == 0


@pytest.mark.parametrize("other", ["garbage", 42])
def test_iadd_unsupported_raises_on_standalone_collection(other):
    coll = ConstraintCollection()
    with pytest.raises(TypeError):
        coll += other
