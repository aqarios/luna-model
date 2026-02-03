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
