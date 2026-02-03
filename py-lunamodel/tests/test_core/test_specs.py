from enum import Enum
from itertools import combinations
from random import Random
from typing import TypeVar

import pytest
from luna_model import (
    Comparator,
    Constraint,
    Ctype,
    Expression,
    Model,
    ModelSpecs,
    Sense,
    Vtype,
)

from .utils import make_seed

T = TypeVar("T", bound=Enum)


def gen_test_combinations(
    values: list[T],
) -> tuple[list[None | list[T]], list[str]]:
    test_cases: list[None | list[T]] = [None]
    test_ids: list[str] = ["none"]
    for i in range(1, len(values) + 1):
        for comb in combinations(values, i):
            test_cases.append(list(comb))
            test_ids.append("-".join(v.name for v in comb))
    return test_cases, test_ids


sense, sense_ids = [None, Sense.MIN, Sense.MAX], ["None", "Sense.MIN", "Sense.MAX"]
vtypes, vtypes_ids = gen_test_combinations(
    [Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL]
)
constraints, constraints_ids = gen_test_combinations(
    [Ctype.EQUALITY, Ctype.INEQUALITY]
)
max_constraint_degree = [None, 1, 2]
max_degree = [None, 1, 2, 3]
max_num_variables = [None, 1, 2, 3, 4]


@pytest.mark.parametrize("sense", sense, ids=sense_ids)
@pytest.mark.parametrize("vtypes", vtypes, ids=vtypes_ids)
@pytest.mark.parametrize("constraints", constraints, ids=constraints_ids)
@pytest.mark.parametrize("max_degree", max_degree)
@pytest.mark.parametrize("max_constraint_degree", max_constraint_degree)
@pytest.mark.parametrize("max_num_variables", max_num_variables)
def test_modelspecs_emptymodel(
    sense: Sense | None,
    vtypes: list[Vtype] | None,
    constraints: list[Ctype] | None,
    max_degree: int | None,
    max_constraint_degree: int | None,
    max_num_variables: int | None,
):
    model = Model() if not sense else Model(sense=sense)
    modelspecs = ModelSpecs(
        sense=sense,
        vtypes=vtypes,
        constraints=constraints,
        max_degree=max_degree,
        max_constraint_degree=max_constraint_degree,
        max_num_variables=max_num_variables,
    )
    assert model.satisfies(modelspecs)


@pytest.mark.parametrize("sense", sense, ids=sense_ids)
@pytest.mark.parametrize("vtypes", vtypes, ids=vtypes_ids)
@pytest.mark.parametrize("constraints", constraints, ids=constraints_ids)
@pytest.mark.parametrize("max_degree", max_degree)
@pytest.mark.parametrize("max_constraint_degree", max_constraint_degree)
@pytest.mark.parametrize("max_num_variables", max_num_variables)
def test_modelspecs_constructed(
    sense: Sense | None,
    vtypes: list[Vtype] | None,
    constraints: list[Ctype] | None,
    max_degree: int | None,
    max_constraint_degree: int | None,
    max_num_variables: int | None,
):
    rand = Random(make_seed())
    model = Model()
    model.set_sense(sense if sense is not None else Sense.MIN)
    required_vars = max(
        len(vtypes) if vtypes is not None else 0,
        max_degree if max_degree is not None else 0,
        max_constraint_degree if max_constraint_degree is not None else 0,
    )
    num_vars = (
        min(required_vars, max_num_variables)
        if max_num_variables is not None
        else required_vars
    )
    constraint_degree = (
        min(num_vars, max_constraint_degree)
        if max_constraint_degree is not None
        else num_vars
    )
    degree = min(num_vars, max_degree) if max_degree is not None else num_vars
    if vtypes:
        for i in range(num_vars):
            model.add_variable(f"v{i}", vtype=vtypes[i % len(vtypes)])
    else:
        for i in range(num_vars):
            model.add_variable(f"v{i}")
    if max_degree:
        expr = Expression(env=model.environment)
        for i in range(degree):
            if i == 0:
                expr += model.get_variable(f"v{i}")
            else:
                expr *= model.get_variable(f"v{i}")
        model.objective += expr

    if constraints:
        for i in range(len(constraints)):
            expr: Expression = Expression(env=model.environment)
            for i in range(constraint_degree):
                expr *= model.get_variable(f"v{i}")

            model.constraints += Constraint(
                expr,
                0,
                Comparator.EQ
                if constraints[i % len(constraints)] == Ctype.EQUALITY
                else rand.choice([Comparator.LE, Comparator.GE]),
            )
    modelspecs = ModelSpecs(
        sense=sense,
        vtypes=vtypes,
        constraints=constraints,
        max_degree=max_degree,
        max_constraint_degree=max_constraint_degree,
        max_num_variables=max_num_variables,
    )
    assert model.satisfies(modelspecs)


def test_modelspecs_varsubset():
    model = Model()
    b = model.add_variable("b")
    i = model.add_variable("i", vtype=Vtype.INTEGER)
    model.objective += b + i
    modelspecs = ModelSpecs(
        vtypes=[Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL]
    )
    assert model.satisfies(modelspecs)


def test_modelspecs_not_varsubset():
    model = Model()
    b = model.add_variable("b")
    model.objective += b
    modelspecs = ModelSpecs(vtypes=[Vtype.SPIN, Vtype.INTEGER, Vtype.REAL])
    assert not model.satisfies(modelspecs)


def test_modelspecs_constr_subset_le():
    model = Model()
    b = model.add_variable("b")
    model.objective += b
    model.constraints += b <= 2
    modelspecsA = ModelSpecs(constraints=[Ctype.INEQUALITY])
    modelspecsB = ModelSpecs(constraints=[Ctype.LESS_EQUAL])
    modelspecsC = ModelSpecs(
        constraints=[Ctype.LESS_EQUAL, Ctype.GREATER_EQUAL]
    )
    modelspecsD = ModelSpecs(
        constraints=[Ctype.INEQUALITY, Ctype.GREATER_EQUAL]
    )
    assert model.satisfies(modelspecsA), "failed to satisfy modelspecsA"
    assert model.satisfies(modelspecsB), "failed to satisfy modelspecsB"
    assert model.satisfies(modelspecsC), "failed to satisfy modelspecsC"
    assert model.satisfies(modelspecsD), "failed to satisfy modelspecsD"

    modelspecs_fail = ModelSpecs(constraints=[Ctype.GREATER_EQUAL])
    assert not model.satisfies(modelspecs_fail), "failed to NOT satisfy modelspecs_fail"


def test_modelspecs_constr_subset_ge():
    model = Model()
    b = model.add_variable("b")
    model.objective += b
    model.constraints += b >= 2
    modelspecsA = ModelSpecs(constraints=[Ctype.INEQUALITY])
    modelspecsB = ModelSpecs(constraints=[Ctype.GREATER_EQUAL])
    modelspecsC = ModelSpecs(
        constraints=[Ctype.GREATER_EQUAL, Ctype.LESS_EQUAL]
    )
    modelspecsD = ModelSpecs(
        constraints=[Ctype.INEQUALITY, Ctype.LESS_EQUAL]
    )
    assert model.satisfies(modelspecsA), "failed to satisfy modelspecsA"
    assert model.satisfies(modelspecsB), "failed to satisfy modelspecsB"
    assert model.satisfies(modelspecsC), "failed to satisfy modelspecsC"
    assert model.satisfies(modelspecsD), "failed to satisfy modelspecsD"

    modelspecs_fail = ModelSpecs(constraints=[Ctype.LESS_EQUAL])
    assert not model.satisfies(modelspecs_fail), "failed to NOT satisfy modelspecs_fail"


def test_model_get_specs_basic():
    """Test that get_specs() returns correct ModelSpecs for a basic model."""
    model = Model(sense=Sense.MIN)
    x = model.add_variable("x", vtype=Vtype.BINARY)
    model.objective += x
    
    specs = model.get_specs()
    assert specs is not None
    assert specs.sense == Sense.MIN
    assert specs.vtypes is not None
    assert Vtype.BINARY in specs.vtypes
    # Unconstrained models may return None, empty list, or list with UNCONSTRAINED
    # depending on internal representation
    if specs.constraints is not None and len(specs.constraints) > 0:
        assert Ctype.UNCONSTRAINED in specs.constraints


def test_model_get_specs_with_constraints():
    """Test that get_specs() correctly reflects constraint types."""
    model = Model(sense=Sense.MAX)
    x = model.add_variable("x", vtype=Vtype.INTEGER)
    y = model.add_variable("y", vtype=Vtype.BINARY)
    model.objective += x + y
    model.constraints += x <= 5
    model.constraints += y >= 0
    
    specs = model.get_specs()
    assert specs is not None
    assert specs.sense == Sense.MAX
    assert specs.vtypes is not None
    assert Vtype.INTEGER in specs.vtypes
    assert Vtype.BINARY in specs.vtypes
    assert specs.constraints is not None
    # Check that inequality constraints are present
    assert any(c in [Ctype.INEQUALITY, Ctype.LESS_EQUAL, Ctype.GREATER_EQUAL] for c in specs.constraints)


def test_model_get_specs_objective_degree():
    """Test that get_specs() correctly reports objective degree."""
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")
    z = model.add_variable("z")
    # Create a degree-3 objective
    model.objective += x * y * z
    
    specs = model.get_specs()
    assert specs is not None
    assert specs.max_degree == 3


def test_model_get_specs_constraint_degree():
    """Test that get_specs() correctly reports constraint degree."""
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")
    # Add a quadratic constraint
    model.constraints += x * y <= 1
    
    specs = model.get_specs()
    assert specs is not None
    assert specs.max_constraint_degree is not None
    assert specs.max_constraint_degree >= 2


def test_model_satisfies_positive():
    """Test that model.satisfies() works for models that meet requirements."""
    model = Model(sense=Sense.MIN)
    x = model.add_variable("x", vtype=Vtype.BINARY)
    y = model.add_variable("y", vtype=Vtype.BINARY)
    model.objective += x + y
    
    # Should satisfy specs that allow binary variables
    specs = ModelSpecs(sense=Sense.MIN, vtypes=[Vtype.BINARY])
    assert model.satisfies(specs)
    
    # Should satisfy specs that allow binary and integer
    specs2 = ModelSpecs(vtypes=[Vtype.BINARY, Vtype.INTEGER])
    assert model.satisfies(specs2)


def test_model_satisfies_negative_vtype():
    """Test that model.satisfies() returns False when variable types don't match."""
    model = Model()
    x = model.add_variable("x", vtype=Vtype.INTEGER)
    model.objective += x
    
    # Should NOT satisfy specs requiring only binary variables
    specs = ModelSpecs(vtypes=[Vtype.BINARY])
    assert not model.satisfies(specs)
    
    # Should NOT satisfy specs requiring only spin variables
    specs2 = ModelSpecs(vtypes=[Vtype.SPIN])
    assert not model.satisfies(specs2)


def test_model_satisfies_negative_constraints():
    """Test that model.satisfies() enforces constraint type requirements."""
    model = Model()
    x = model.add_variable("x")
    model.objective += x
    model.constraints += x <= 5
    
    # Model with constraints should NOT satisfy unconstrained-only specs
    specs = ModelSpecs(constraints=[Ctype.UNCONSTRAINED])
    assert not model.satisfies(specs)
    
    # Model with <= constraint should NOT satisfy specs requiring only >=
    specs2 = ModelSpecs(constraints=[Ctype.GREATER_EQUAL])
    assert not model.satisfies(specs2)


def test_model_satisfies_degree_limits():
    """Test that model.satisfies() enforces degree limits."""
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")
    # Quadratic objective
    model.objective += x * y
    
    # Should satisfy specs allowing degree 2 or higher
    specs_ok = ModelSpecs(max_degree=2)
    assert model.satisfies(specs_ok)
    
    specs_ok2 = ModelSpecs(max_degree=3)
    assert model.satisfies(specs_ok2)
    
    # Should NOT satisfy specs requiring max degree 1
    specs_fail = ModelSpecs(max_degree=1)
    assert not model.satisfies(specs_fail)


def test_model_satisfies_sense():
    """Test that model.satisfies() enforces sense requirements."""
    model = Model(sense=Sense.MIN)
    x = model.add_variable("x")
    model.objective += x
    
    # Should satisfy specs with matching sense
    specs_min = ModelSpecs(sense=Sense.MIN)
    assert model.satisfies(specs_min)
    
    # Should NOT satisfy specs with different sense
    specs_max = ModelSpecs(sense=Sense.MAX)
    assert not model.satisfies(specs_max)
