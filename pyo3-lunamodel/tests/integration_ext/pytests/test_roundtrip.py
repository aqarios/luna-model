from __future__ import annotations

import pytest

from luna_model import (
    Bounds,
    Comparator,
    Constraint,
    ConstraintCollection,
    Ctype,
    Environment,
    Model,
    ModelSpecs,
    Sense,
    Solution,
    TranslationTarget,
    Unbounded,
    ValueSource,
    Variable,
    Vtype,
)


def sample_model() -> Model:
    model = Model("roundtrip", sense=Sense.MIN)
    x = model.add_variable("x", vtype=Vtype.BINARY)
    y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=3)
    model.objective = x + 2 * y
    model.constraints += x + y <= 2
    return model


def test_roundtrip_model(ext):
    value = sample_model()
    assert ext.roundtrip_model(value).equal_contents(value)


def test_roundtrip_environment(ext):
    value = sample_model().environment
    assert ext.roundtrip_environment(value).equal_contents(value)


def test_roundtrip_variable(ext):
    env = Environment()
    value = Variable("standalone", vtype=Vtype.BINARY, env=env)
    assert ext.roundtrip_variable(value) == value


def test_roundtrip_bounds(ext):
    value = Bounds(-1, 4)
    assert ext.roundtrip_bounds(value) == value


def test_roundtrip_expression(ext):
    model = sample_model()
    value = model.objective
    assert ext.roundtrip_expression(value).equal_contents(value)


def test_roundtrip_constraint(ext):
    value: Constraint = sample_model().constraints["c0"]
    assert ext.roundtrip_constraint(value).equal_contents(value)


def test_roundtrip_constraint_collection(ext):
    value: ConstraintCollection = sample_model().constraints
    assert ext.roundtrip_constraint_collection(value).equal_contents(value)


def test_roundtrip_solution(ext):
    env = Environment()
    Variable("x", env=env)
    value = Solution([{"x": 1}], counts=[1], obj_values=[0.0], feasible=[True], env=env)
    assert ext.roundtrip_solution(value) == value


def test_roundtrip_model_specs(ext):
    value = ModelSpecs(
        sense=Sense.MIN,
        vtypes={Vtype.BINARY},
        constraints={Ctype.LESS_EQUAL},
        max_degree=2,
        max_constraint_degree=1,
        max_num_variables=8,
    )
    result = ext.roundtrip_model_specs(value)
    assert result.sense == value.sense
    assert result.vtypes == value.vtypes
    assert result.constraints == value.constraints
    assert result.max_degree == value.max_degree
    assert result.max_constraint_degree == value.max_constraint_degree
    assert result.max_num_variables == value.max_num_variables


@pytest.mark.parametrize(
    ("function_name", "value"),
    [
        *[("roundtrip_vtype", v) for v in Vtype],
        *[("roundtrip_sense", v) for v in Sense],
        *[("roundtrip_ctype", v) for v in Ctype],
        *[("roundtrip_comparator", v) for v in Comparator],
        *[("roundtrip_value_source", v) for v in ValueSource],
        *[("roundtrip_translation_target", v) for v in TranslationTarget],
    ],
)
def test_roundtrip_enums(ext, function_name: str, value):
    assert getattr(ext, function_name)(value) == value


def test_roundtrip_unbounded(ext):
    assert ext.roundtrip_unbounded(Unbounded) is Unbounded
