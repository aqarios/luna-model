from typing import Callable

import pytest

from aqmodels import Bounds, Model, Unbounded, Variable, Vtype


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
            r = Variable("r", vtype=Vtype.Real, bounds=Bounds(lower=10, upper=5))
        m.objective += b * s + i * r
        m.add_constraint(b + s >= 2, "constraint")
        return m

    return creator


@pytest.mark.model
def test_hash_model_empty(model_empty_maker):
    m1 = model_empty_maker()
    m2 = model_empty_maker()
    assert hash(m1) == hash(m2)


@pytest.mark.model
def test_hash_model_empty_with_vars(model_empty_with_vars_maker):
    m1 = model_empty_with_vars_maker()
    m2 = model_empty_with_vars_maker()
    assert hash(m1) == hash(m2)


@pytest.mark.model
def test_hash_model_objective(model_objective_maker):
    m1 = model_objective_maker()
    m2 = model_objective_maker()
    assert hash(m1) == hash(m2)


def test_hash_model_objective_and_constraints(model_objective_and_constraints_maker):
    m1 = model_objective_and_constraints_maker()
    m2 = model_objective_and_constraints_maker()
    assert hash(m1) == hash(m2)


def test_hash_model_objective_and_constraints_and_bounds(
    model_objective_and_constraints_and_bounds_maker,
):
    m1 = model_objective_and_constraints_and_bounds_maker()
    m2 = model_objective_and_constraints_and_bounds_maker()
    assert hash(m1) == hash(m2)


def test_hash_model_different(
    model_empty_maker,
    model_empty_with_vars_maker,
    model_objective_maker,
    model_objective_and_constraints_maker,
    model_objective_and_constraints_and_bounds_maker,
):
    m1 = model_empty_maker()
    m2 = model_empty_with_vars_maker()
    m3 = model_objective_maker()
    m4 = model_objective_and_constraints_maker()
    m5 = model_objective_and_constraints_and_bounds_maker()

    assert hash(m1) != hash(m2)
    assert hash(m1) != hash(m3)
    assert hash(m1) != hash(m4)
    assert hash(m1) != hash(m5)

    assert hash(m2) != hash(m3)
    assert hash(m2) != hash(m4)
    assert hash(m2) != hash(m5)

    assert hash(m3) != hash(m4)
    assert hash(m3) != hash(m5)

    assert hash(m4) != hash(m5)
