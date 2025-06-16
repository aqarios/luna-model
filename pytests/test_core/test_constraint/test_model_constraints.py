import pytest

from aqmodels import Environment, Model, Variable, Vtype
from aqmodels.errors import DuplicateConstraintNameError


def model_iadd(request, env) -> Model:
    with env:
        model = Model()
        x = Variable("x")
        y = Variable("y")

    comp = request.param
    if comp == "le":
        model.constraints += x + y <= 1
    elif comp == "eq":
        model.constraints += x + y == 1
    elif comp == "ge":
        model.constraints += x + y >= 1

    return model


def model_iadd_named(request, env) -> Model:
    with env:
        model = Model()
        x = Variable("x")
        y = Variable("y")

    comp = request.param
    if comp == "le":
        model.constraints += x + y <= 1, "constraint_le"
    elif comp == "eq":
        model.constraints += x + y == 1, "constraint_eq"
    elif comp == "ge":
        model.constraints += x + y >= 1, "constraint_ge"

    return model


def model_fadd(request, env) -> Model:
    with env:
        model = Model()
        x = Variable("x")
        y = Variable("y")

    comp = request.param
    if comp == "le":
        model.constraints.add_constraint(x + y <= 1)
    elif comp == "eq":
        model.constraints.add_constraint(x + y == 1)
    elif comp == "ge":
        model.constraints.add_constraint(x + y >= 1)

    return model


def model_fadd_named(request, env) -> Model:
    with env:
        model = Model()
        x = Variable("x")
        y = Variable("y")

    comp = request.param
    if comp == "le":
        model.constraints.add_constraint(x + y <= 1, "constraint_le")
    elif comp == "eq":
        model.constraints.add_constraint(x + y == 1, "constraint_eq")
    elif comp == "ge":
        model.constraints.add_constraint(x + y >= 1, "constraint_ge")

    return model


def model_direct_add(request, env) -> Model:
    with env:
        model = Model()
        x = Variable("x")
        y = Variable("y")

    comp = request.param
    if comp == "le":
        model.add_constraint(x + y <= 1)
    elif comp == "eq":
        model.add_constraint(x + y == 1)
    elif comp == "ge":
        model.add_constraint(x + y >= 1)

    return model


def model_direct_add_named(request, env) -> Model:
    with env:
        model = Model()
        x = Variable("x")
        y = Variable("y")

    comp = request.param
    if comp == "le":
        model.add_constraint(x + y <= 1, "constraint_le")
    elif comp == "eq":
        model.add_constraint(x + y == 1, "constraint_eq")
    elif comp == "ge":
        model.add_constraint(x + y >= 1, "constraint_ge")

    return model


@pytest.fixture
def models(request) -> tuple[Model, Model, Model]:
    env = Environment()
    return (model_iadd(request, env), model_fadd(request, env), model_direct_add(request, env))


@pytest.fixture
def models_named(request) -> tuple[Model, Model, Model]:
    env = Environment()
    return (
        model_iadd_named(request, env),
        model_fadd_named(request, env),
        model_direct_add_named(request, env),
    )


@pytest.mark.constraint
def test_model_add_constraint_le():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y <= 1
    assert model.num_constraints == 1


@pytest.mark.constraint
def test_model_add_constraint_eq():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y == 0
    assert model.num_constraints == 1


@pytest.mark.constraint
def test_model_add_constraint_ge():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y >= 1
    assert model.num_constraints == 1


@pytest.mark.constraint
def test_model_add_constraint_le_named():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y <= 1, "constraint"
    assert model.num_constraints == 1
    assert model.constraints[0].name == "constraint"

@pytest.mark.constraint
def test_model_add_constraint_le_named_duplicate():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y <= 1, "constraint"
    with pytest.raises(DuplicateConstraintNameError):
        model.constraints += x - y == 2, "constraint"


@pytest.mark.constraint
def test_model_add_constraint_eq_named():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y == 0, "constraint"
    assert model.num_constraints == 1
    assert model.constraints[0].name == "constraint"

@pytest.mark.constraint
def test_model_add_constraint_eq_named_duplicate():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y == 0, "constraint"
    with pytest.raises(DuplicateConstraintNameError):
        model.constraints += x - y == 2, "constraint"


@pytest.mark.constraint
def test_model_add_constraint_ge_named():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y >= 1, "constraint"
    assert model.num_constraints == 1
    assert model.constraints[0].name == "constraint"

@pytest.mark.constraint
def test_model_add_constraint_ge_named_duplicate():
    with Environment():
        model = Model()
        x = Variable("x")
        y = Variable("y")

    model.constraints += x + y >= 0, "constraint"
    with pytest.raises(DuplicateConstraintNameError):
        model.constraints += x - y == 2, "constraint"


@pytest.mark.constraint
@pytest.mark.parametrize("models", ["le"], indirect=True)
def test_model_add_constraint_same_le(models: tuple[Model, Model, Model]):
    model_a, model_b, model_c = models
    assert model_a.constraints == model_b.constraints
    assert model_b.constraints == model_c.constraints

@pytest.mark.constraint
@pytest.mark.parametrize("models", ["eq"], indirect=True)
def test_model_add_constraint_same_eq(models: tuple[Model, Model, Model]):
    model_a, model_b, model_c = models
    assert model_a.constraints == model_b.constraints
    assert model_b.constraints == model_c.constraints

@pytest.mark.constraint
@pytest.mark.parametrize("models", ["ge"], indirect=True)
def test_model_add_constraint_same_ge(models: tuple[Model, Model, Model]):
    model_a, model_b, model_c = models
    assert model_a.constraints == model_b.constraints
    assert model_b.constraints == model_c.constraints


@pytest.mark.constraint
@pytest.mark.parametrize("models_named", ["le", "eq", "ge"], indirect=True)
def test_model_add_constraint_same_named(models_named: tuple[Model, Model, Model]):
    model_a, model_b, model_c = models_named
    assert model_a.constraints == model_b.constraints
    assert model_b.constraints == model_c.constraints


@pytest.mark.constraint
def test_model_constraints_len():
    m = Model()
    assert len(m.constraints) == 0
    with m.environment:
        x = Variable("x")
        y = Variable("y")
        z = Variable("z", vtype=Vtype.Integer)
    assert len(m.constraints) == 0
    m.constraints += x + y <= 1
    assert len(m.constraints) == 1
    m.objective = x - y - 0.1 * z
    m.add_constraint(z <= 10)
    assert len(m.constraints) == 2
