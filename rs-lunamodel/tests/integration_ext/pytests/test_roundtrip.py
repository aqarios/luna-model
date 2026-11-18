from __future__ import annotations

import luna_model
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


def test_bounds_created_by_extension_transfers_through_c_handle(ext):
    value = ext.new_bounds(-2, 5)
    assert isinstance(value, Bounds)
    assert value == Bounds(-2, 5)


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


def test_model_evaluation_returns_c_backed_solution(ext):
    model = sample_model()
    value = Solution([{"x": 1, "y": 0}], counts=[1], env=model.environment)

    evaluated = ext.evaluate_model(model, value)

    assert evaluated.obj_values.tolist() == [1.0]
    assert next(iter(evaluated.results)).feasible is True


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


def test_model_specs_created_by_extension_transfers_through_c_handle(ext):
    value = ext.new_model_specs(4)
    assert isinstance(value, ModelSpecs)
    assert value.max_degree == 4


def test_variable_created_by_c_facade_transfers_through_c_handle(ext):
    model = sample_model()
    value = ext.model_variable(model, "x")
    assert isinstance(value, Variable)
    assert value == model.get_variable("x")


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


@pytest.mark.parametrize(
    ("function_name", "value", "expected"),
    [
        ("roundtrip_vtype", luna_model._lm.PyVtype.Binary, Vtype.BINARY),
        ("roundtrip_sense", luna_model._lm.PySense.Min, Sense.MIN),
        ("roundtrip_ctype", luna_model._lm.PyCtype.Equality, Ctype.EQUALITY),
        ("roundtrip_comparator", luna_model._lm.PyComparator.Eq, Comparator.EQ),
        ("roundtrip_value_source", luna_model._lm.PyValueSource.Raw, ValueSource.RAW),
        (
            "roundtrip_translation_target",
            luna_model._lm.PyTranslationTarget.Qubo,
            TranslationTarget.QUBO,
        ),
    ],
)
def test_internal_enum_values_remain_accepted(ext, function_name, value, expected):
    assert getattr(ext, function_name)(value) == expected


def test_roundtrip_unbounded(ext):
    assert ext.roundtrip_unbounded(Unbounded) is Unbounded
