import pytest

from aqmodels import Environment, Model, Vtype
from aqmodels.errors import DuplicateConstraintNameError, NoConstraintForKeyError


def model_iadd(request) -> Model:
    with Environment():
        model = Model()
        x = model.add_variable("x")
        y = model.add_variable("y")

    comp = request.param
    if comp == "le":
        model.constraints += x + y <= 1
    elif comp == "eq":
        model.constraints += x + y == 1
    elif comp == "ge":
        model.constraints += x + y >= 1

    return model


def model_iadd_named(request) -> Model:
    with Environment():
        model = Model()
        x = model.add_variable("x")
        y = model.add_variable("y")

    comp = request.param
    if comp == "le":
        model.constraints += x + y <= 1, "constraint_le"
    elif comp == "eq":
        model.constraints += x + y == 1, "constraint_eq"
    elif comp == "ge":
        model.constraints += x + y >= 1, "constraint_ge"

    return model


def model_fadd(request) -> Model:
    with Environment():
        model = Model()
        x = model.add_variable("x")
        y = model.add_variable("y")

    comp = request.param
    if comp == "le":
        model.constraints.add_constraint(x + y <= 1)
    elif comp == "eq":
        model.constraints.add_constraint(x + y == 1)
    elif comp == "ge":
        model.constraints.add_constraint(x + y >= 1)

    return model


def model_fadd_named(request) -> Model:
    with Environment():
        model = Model()
        x = model.add_variable("x")
        y = model.add_variable("y")

    comp = request.param
    if comp == "le":
        model.constraints.add_constraint(x + y <= 1, "constraint_le")
    elif comp == "eq":
        model.constraints.add_constraint(x + y == 1, "constraint_eq")
    elif comp == "ge":
        model.constraints.add_constraint(x + y >= 1, "constraint_ge")

    return model


def model_direct_add(request) -> Model:
    with Environment():
        model = Model()
        x = model.add_variable("x")
        y = model.add_variable("y")

    comp = request.param
    if comp == "le":
        model.add_constraint(x + y <= 1)
    elif comp == "eq":
        model.add_constraint(x + y == 1)
    elif comp == "ge":
        model.add_constraint(x + y >= 1)

    return model


def model_direct_add_named(request) -> Model:
    with Environment():
        model = Model()
        x = model.add_variable("x")
        y = model.add_variable("y")

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
    return (model_iadd(request), model_fadd(request), model_direct_add(request))


@pytest.fixture
def models_named(request) -> tuple[Model, Model, Model]:
    return (
        model_iadd_named(request),
        model_fadd_named(request),
        model_direct_add_named(request),
    )


def check_get_constraint_error(model, index, name, error_type):
    with pytest.raises(error_type):
        model.constraints[index]
    with pytest.raises(error_type):
        model.constraints[name]
    with pytest.raises(error_type):
        model.constraints.get(index)
    with pytest.raises(error_type):
        model.constraints.get(name)


@pytest.mark.constraint
def test_model_get_constraint_not_existing():
    check_get_constraint_error(Model(), 0, "my_constraint", NoConstraintForKeyError)
    check_get_constraint_error(Model(), 0, "my_constraint", IndexError)


@pytest.mark.constraint
def test_model_remove_constraint():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    def add_constraint():
        model.constraints += x + y <= 1, "my_constraint"

    def check(error_type):
        check_get_constraint_error(model, 0, "my_constraint", error_type)

    check(NoConstraintForKeyError)
    check(IndexError)
    add_constraint()
    model.constraints.remove(0)
    check(NoConstraintForKeyError)
    check(IndexError)
    add_constraint()
    model.constraints.remove("my_constraint")
    check(NoConstraintForKeyError)
    check(IndexError)


@pytest.mark.constraint
def test_model_add_constraint_le():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y <= 1, "my_constraint"
    assert model.num_constraints == 1
    assert model.constraints[0] == model.constraints["my_constraint"]
    assert model.constraints.get(0) == model.constraints.get("my_constraint")


@pytest.mark.constraint
def test_model_add_constraint_eq():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y == 0, "my_constraint"
    assert model.num_constraints == 1
    assert model.constraints[0] == model.constraints["my_constraint"]
    assert model.constraints.get(0) == model.constraints.get("my_constraint")


@pytest.mark.constraint
def test_model_add_constraint_ge():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y >= 1, "my_constraint"
    assert model.num_constraints == 1
    assert model.constraints[0] == model.constraints["my_constraint"]
    assert model.constraints.get(0) == model.constraints.get("my_constraint")


@pytest.mark.constraint
def test_model_add_constraint_le_named():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y <= 1, "constraint"
    assert model.num_constraints == 1
    assert model.constraints[0].name == "constraint"
    assert model.constraints[0] == model.constraints["constraint"]
    assert model.constraints.get(0) == model.constraints.get("constraint")


@pytest.mark.constraint
def test_model_add_constraint_le_named_duplicate():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y <= 1, "constraint"
    with pytest.raises(DuplicateConstraintNameError):
        model.constraints += x - y == 2, "constraint"


@pytest.mark.constraint
def test_model_add_constraint_eq_named():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y == 0, "constraint"
    assert model.num_constraints == 1
    assert model.constraints[0].name == "constraint"


@pytest.mark.constraint
def test_model_add_constraint_eq_named_duplicate():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y == 0, "constraint"
    with pytest.raises(DuplicateConstraintNameError):
        model.constraints += x - y == 2, "constraint"


@pytest.mark.constraint
def test_model_add_constraint_ge_named():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y >= 1, "constraint"
    assert model.num_constraints == 1
    assert model.constraints[0].name == "constraint"


@pytest.mark.constraint
def test_model_add_constraint_ge_named_duplicate():
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")

    model.constraints += x + y >= 0, "constraint"
    with pytest.raises(DuplicateConstraintNameError):
        model.constraints += x - y == 2, "constraint"


@pytest.mark.constraint
@pytest.mark.parametrize("models", ["le", "eq", "ge"], indirect=True)
def test_model_add_constraint_same(models: tuple[Model, Model, Model]):
    model_a, model_b, model_c = models
    assert model_a.constraints.equal_contents(model_b.constraints)
    assert model_b.constraints.equal_contents(model_c.constraints)


@pytest.mark.constraint
@pytest.mark.parametrize("models_named", ["le", "eq", "ge"], indirect=True)
def test_model_add_constraint_same_named(models_named: tuple[Model, Model, Model]):
    model_a, model_b, model_c = models_named
    assert model_a.constraints.equal_contents(model_b.constraints)
    assert model_b.constraints.equal_contents(model_c.constraints)


@pytest.mark.constraint
def test_model_constraints_len():
    m = Model()
    assert len(m.constraints) == 0
    x = m.add_variable("x")
    y = m.add_variable("y")
    z = m.add_variable("z", vtype=Vtype.Integer)
    assert len(m.constraints) == 0
    m.constraints += x + y <= 1
    assert len(m.constraints) == 1
    m.objective = x - y - 0.1 * z
    m.add_constraint(z <= 10)
    assert len(m.constraints) == 2
