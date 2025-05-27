import pytest
from typing import Callable
from aqmodels import Environment, Model, Variable, Vtype, Bounds, Unbounded


@pytest.fixture
def model_empty_maker() -> Callable[[], Model]:
    def creator():
        return Model()
    return creator

@pytest.fixture
def model_empty_with_vars_maker() -> Callable[[], Model]:
    def creator():
        m = Model()
        with m.environment:
            _ = Variable("b", vtype=Vtype.Binary)
            _ = Variable("s", vtype=Vtype.Spin)
            _ = Variable("i", vtype=Vtype.Integer)
            _ = Variable("r", vtype=Vtype.Real)
        return m
    return creator

@pytest.fixture
def model_objective_maker() -> Callable[[], Model]:
    def creator():
        m = Model()
        with m.environment:
            b = Variable("b", vtype=Vtype.Binary)
            s = Variable("s", vtype=Vtype.Spin)
            i = Variable("i", vtype=Vtype.Integer)
            r = Variable("r", vtype=Vtype.Real)
        m.objective += b * s + i * r
        return m
    return creator

@pytest.fixture
def model_objective_and_constraints_maker() -> Callable[[], Model]:
    def creator():
        m = Model()
        with m.environment:
            b = Variable("b", vtype=Vtype.Binary)
            s = Variable("s", vtype=Vtype.Spin)
            i = Variable("i", vtype=Vtype.Integer)
            r = Variable("r", vtype=Vtype.Real)
        m.objective += b * s + i * r
        m.add_constraint(b + s >= 2, "constraint")
        return m
    return creator

@pytest.fixture
def model_objective_and_constraints_and_bounds_maker() -> Callable[[], Model]:
    def creator():
        m = Model()
        with m.environment:
            b = Variable("b", vtype=Vtype.Binary)
            s = Variable("s", vtype=Vtype.Spin)
            i = Variable("i", vtype=Vtype.Integer, bounds=Bounds(lower=Unbounded))
            r = Variable("r", vtype=Vtype.Real)
        m.objective += b * s + i * r
        m.add_constraint(b + s >= 2, "constraint")
        return m
    return creator

@pytest.mark.model
def test_hash_model_empty(model_empty_maker):
    empty1 = model_empty_maker()
    empty2 = model_empty_maker()
    assert hash(empty1) == hash(empty2)


@pytest.mark.model
def test_hash_model_objective():
    model = Model(name="objective")
    with model.environment:
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")

    model.objective += x * y + z

def test_hash_model_objective_and_constraints():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
        model = Model(name="objective")

    model.objective += 1
    model.objective += x
    model.objective += x * y
    model.objective += x * y * z

def test_hash_model_objective_and_constraints_and_bounds():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
        model = Model(name="objective")

    model.objective += 1
    model.objective += x
    model.objective += x * y
    model.objective += x * y * z

def test_hash_model_different():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
        model = Model(name="objective")

    model.objective += 1
    model.objective += x
    model.objective += x * y
    model.objective += x * y * z

