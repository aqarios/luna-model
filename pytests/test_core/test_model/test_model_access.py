import pytest

from aqmodels import Environment, Expression, Model, Sense, Unbounded, Variable, Vtype
from aqmodels.errors import VariableCreationError, VariableExistsError

from ..utils import assert_linear, assert_offset, assert_quadratic


def make_model() -> Model:
    with Environment():
        return Model()


def var_names(variables: list[Variable]) -> list[str]:
    return [v.name for v in variables]


@pytest.fixture
def model() -> Model:
    return make_model()


@pytest.mark.model
def test_access_name(model: Model):
    name = model.name
    assert isinstance(name, str)
    assert name == "unnamed"


@pytest.mark.model
def test_access_objective(model: Model):
    objective_a = model.objective
    objective_b = model.objective
    assert isinstance(objective_a, Expression)
    assert isinstance(objective_b, Expression)
    assert objective_a.is_equal(objective_b)
    assert model == model


@pytest.mark.model
def test_use_model_environment():
    model = make_model()
    with model.environment:
        _ = Variable("x")
        _ = Model()


@pytest.mark.model
def test_use_instanceadd_bias_to_aq():
    model = make_model()
    with model.environment:
        _ = Variable("x")

    model.objective += 1
    assert_offset(model.objective, 1)


@pytest.mark.model
def test_use_instanceadd_variable_to_aq():
    model = make_model()
    with model.environment:
        x = Variable("x")

    model.objective += x
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 1)


@pytest.mark.model
def test_use_instanceadd_expression_to_aq():
    model = make_model()
    with model.environment:
        x = Variable("x")
        y = Variable("y")

    model.objective += x * y
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 0)
    assert_quadratic(model.objective, (x, y), 1)


@pytest.mark.model
def test_use_set_expression():
    model = make_model()
    with model.environment:
        x = Variable("x")
        y = Variable("y")

    model.set_objective(x * y)
    assert model.sense == Sense.Min
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 0)
    assert_quadratic(model.objective, (x, y), 1)


@pytest.mark.model
def test_use_set_expression_with_sense_min():
    model = make_model()
    with model.environment:
        x = Variable("x")
        y = Variable("y")

    model.set_objective(x * y, sense=Sense.Min)
    assert model.sense == Sense.Min
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 0)
    assert_quadratic(model.objective, (x, y), 1)


@pytest.mark.model
def test_use_set_expression_with_sense_max():
    model = make_model()
    with model.environment:
        x = Variable("x")
        y = Variable("y")

    model.set_objective(x * y, sense=Sense.Max)
    assert model.sense == Sense.Max
    assert_offset(model.objective, 0)
    assert_linear(model.objective, (x,), 0)
    assert_quadratic(model.objective, (x, y), 1)


@pytest.mark.model
def test_access_variables():
    with Environment() as env:
        x = Variable("x")
        y = Variable("y")

        m = Model(env=env)
        assert var_names(m.variables()) == ["x", "y"]
        assert var_names(m.variables(active=False)) == ["x", "y"]
        assert var_names(m.variables(active=True)) == []

        m.objective = 1 * x
        assert var_names(m.variables()) == ["x", "y"]
        assert var_names(m.variables(active=False)) == ["x", "y"]
        assert var_names(m.variables(active=True)) == ["x"]

        m.objective = 1 * y
        assert var_names(m.variables()) == ["x", "y"]
        assert var_names(m.variables(active=False)) == ["x", "y"]
        assert var_names(m.variables(active=True)) == ["y"]

        m.objective = y + x
        assert var_names(m.variables()) == ["x", "y"]
        assert var_names(m.variables(active=False)) == ["x", "y"]
        assert var_names(m.variables(active=True)) == ["x", "y"]

        m2 = Model(env=env)
        m2.objective = 1 * y
        assert var_names(m2.variables()) == ["x", "y"]
        assert var_names(m2.variables(active=False)) == ["x", "y"]
        assert var_names(m2.variables(active=True)) == ["y"]


@pytest.mark.model
def test_add_variables_direct():
    m = Model("test")
    a = m.add_variable("a")
    assert a == m.get_variable("a")

    b = m.add_variable("b", vtype=Vtype.Binary)
    assert b == m.get_variable("b")
    c = m.add_variable("c", vtype=Vtype.Spin)
    assert c == m.get_variable("c")
    d = m.add_variable("d", vtype=Vtype.Integer)
    assert d == m.get_variable("d")
    e = m.add_variable("e", vtype=Vtype.Real)
    assert e == m.get_variable("e")

    with pytest.raises(VariableExistsError):
        _ = m.add_variable("a")

    with pytest.raises(VariableExistsError):
        _ = m.add_variable("b", vtype=Vtype.Binary)

    with pytest.raises(VariableExistsError):
        _ = m.add_variable("c", vtype=Vtype.Spin)

    with pytest.raises(VariableExistsError):
        _ = m.add_variable("d", vtype=Vtype.Integer)

    with pytest.raises(VariableExistsError):
        _ = m.add_variable("e", vtype=Vtype.Real)

    with pytest.raises(VariableCreationError):
        _ = m.add_variable("bf1", vtype=Vtype.Binary, lower=0)
    with pytest.raises(VariableCreationError):
        _ = m.add_variable("bf2", vtype=Vtype.Binary, upper=1)
    with pytest.raises(VariableCreationError):
        _ = m.add_variable("bf3", vtype=Vtype.Binary, lower=0, upper=1)

    with pytest.raises(VariableCreationError):
        _ = m.add_variable("sf1", vtype=Vtype.Spin, lower=0)
    with pytest.raises(VariableCreationError):
        _ = m.add_variable("sf2", vtype=Vtype.Spin, upper=1)
    with pytest.raises(VariableCreationError):
        _ = m.add_variable("sf3", vtype=Vtype.Spin, lower=0, upper=1)

    if1 = m.add_variable("if1", vtype=Vtype.Integer, lower=0)
    assert if1 == m.get_variable("if1")
    if2 = m.add_variable("if2", vtype=Vtype.Integer, upper=1)
    assert if2 == m.get_variable("if2")
    if3 = m.add_variable("if3", vtype=Vtype.Integer, lower=0, upper=1)
    assert if3 == m.get_variable("if3")
    if4 = m.add_variable("if4", vtype=Vtype.Integer, lower=Unbounded)
    assert if4 == m.get_variable("if4")
    if5 = m.add_variable("if5", vtype=Vtype.Integer, upper=Unbounded)
    assert if5 == m.get_variable("if5")
    if6 = m.add_variable("if6", vtype=Vtype.Integer, lower=Unbounded, upper=Unbounded)
    assert if6 == m.get_variable("if6")

    rf1 = m.add_variable("rf1", vtype=Vtype.Real, lower=0)
    assert rf1 == m.get_variable("rf1")
    rf2 = m.add_variable("rf2", vtype=Vtype.Real, upper=1)
    assert rf2 == m.get_variable("rf2")
    rf3 = m.add_variable("rf3", vtype=Vtype.Real, lower=0, upper=1)
    assert rf3 == m.get_variable("rf3")
    rf4 = m.add_variable("rf4", vtype=Vtype.Real, lower=Unbounded)
    assert rf4 == m.get_variable("rf4")
    rf5 = m.add_variable("rf5", vtype=Vtype.Real, upper=Unbounded)
    assert rf5 == m.get_variable("rf5")
    rf6 = m.add_variable("rf6", vtype=Vtype.Real, lower=Unbounded, upper=Unbounded)
    assert rf6 == m.get_variable("rf6")
