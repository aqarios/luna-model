import pytest

from luna_model import ConstraintCollection, Model
from luna_model.errors import DifferentEnvsError


@pytest.fixture()
def model():
    return Model()


@pytest.fixture()
def x(model):
    return model.add_variable("x")


@pytest.fixture()
def foreign():
    return Model().add_variable("a")


# Objective


def test_objective_assign_foreign_variable(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.objective = foreign


def test_objective_assign_foreign_expression(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.objective = 2 * foreign + 1


def test_objective_iadd_foreign_variable(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.objective += foreign


def test_set_objective_foreign(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.set_objective(foreign)


# add_constraint / add_constraints


def test_add_constraint_foreign_on_empty_model(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.add_constraint(foreign <= 0)


def test_add_constraint_foreign_after_own(model, x, foreign):
    model.add_constraint(x <= 1)
    with pytest.raises(DifferentEnvsError):
        model.add_constraint(foreign <= 0)


def test_add_constraints_foreign_on_empty_model(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.add_constraints([foreign <= 0])


def test_add_constraints_foreign_adds_nothing(model, x, foreign):
    with pytest.raises(DifferentEnvsError):
        model.add_constraints([x <= 1, foreign <= 0])
    assert len(model.constraints) == 0


def test_add_constraints_foreign_collection(model, foreign):
    coll = ConstraintCollection()
    coll += foreign <= 0
    with pytest.raises(DifferentEnvsError):
        model.add_constraints(coll)


# model.constraints view methods


def test_constraints_view_add_constraint_foreign(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.constraints.add_constraint(foreign <= 0)


def test_constraints_view_add_constraints_foreign(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.constraints.add_constraints([foreign <= 0])


def test_constraints_view_setitem_foreign(model, x, foreign):
    model.add_constraint(x <= 1, "c")
    with pytest.raises(DifferentEnvsError):
        model.constraints["c"] = foreign <= 0


# constraints += / constraints =


def test_constraints_iadd_foreign_on_empty_model(model, foreign):
    with pytest.raises(DifferentEnvsError):
        model.constraints += foreign <= 0


def test_constraints_iadd_foreign_after_own(model, x, foreign):
    model.add_constraint(x <= 1)
    with pytest.raises(DifferentEnvsError):
        model.constraints += foreign <= 0


def test_constraints_assign_foreign_collection(model, foreign):
    coll = ConstraintCollection()
    coll += foreign <= 0
    with pytest.raises(DifferentEnvsError):
        model.constraints = coll


# Standalone collection


def test_collection_mixing_envs(x, foreign):
    coll = ConstraintCollection()
    coll += x <= 1
    with pytest.raises(DifferentEnvsError):
        coll += foreign <= 0


def test_collection_iadd_list_mixing_envs(x, foreign):
    coll = ConstraintCollection()
    with pytest.raises(DifferentEnvsError):
        coll += [x <= 1, foreign <= 0]


def test_collection_setitem_mixing_envs(x, foreign):
    coll = ConstraintCollection()
    coll += x <= 1, "c"
    coll += x >= 0, "d"
    with pytest.raises(DifferentEnvsError):
        coll["c"] = foreign <= 0


# Same environment still works


def test_constraints_iadd_own(model, x):
    model.constraints += x <= 1
    assert len(model.constraints) == 1


def test_constraints_assign_own_collection(model, x):
    model.add_constraint(x <= 1)
    model.constraints = model.constraints
    assert len(model.constraints) == 1


# Error messages name the offending item


def test_error_names_objective_and_variable(model, foreign):
    with pytest.raises(DifferentEnvsError, match="objective uses variable 'a'"):
        model.objective = foreign


def test_error_names_constraint_and_variable(model, foreign):
    with pytest.raises(DifferentEnvsError, match="constraint 'ca' uses variable 'a'"):
        model.add_constraint(foreign <= 0, "ca")


def test_error_names_collection_constraint(model, foreign):
    coll = ConstraintCollection()
    coll += foreign <= 0, "ca"
    with pytest.raises(DifferentEnvsError, match="constraint 'ca' uses variable 'a'"):
        model.constraints = coll


def test_error_names_unnamed_constraint(model, foreign):
    with pytest.raises(DifferentEnvsError, match="an unnamed constraint uses variable 'a'"):
        model.add_constraint(foreign <= 0)


def test_error_names_objective_on_iadd(model, foreign):
    with pytest.raises(DifferentEnvsError, match="objective cannot be combined"):
        model.objective += foreign


# substitute


def test_substitute_foreign_replacement(model, x, foreign):
    model.objective = x
    with pytest.raises(DifferentEnvsError, match="substitution replacement uses variable 'a'"):
        model.substitute(x, foreign)
    assert str(model.objective) == "x"


def test_substitute_foreign_target(model, x, foreign):
    model.objective = x
    with pytest.raises(DifferentEnvsError, match="substitution target 'a'"):
        model.substitute(foreign, x)
    assert str(model.objective) == "x"
