import pytest

from aq_models import Variable
from aq_models import Environment
from aq_models import Expression
from aq_models import VariableExistsException
from aq_models import NoActiveEnvironmentFoundException
from aq_models import MultipleActiveEnvironmentsException


@pytest.mark.variable
def test_create_variable_explicit():
    env = Environment()
    _ = Variable("x", env)


@pytest.mark.variable
def test_create_variable_explicit_del_create():
    env = Environment()
    x = Variable("x", env)
    del x
    with pytest.raises(VariableExistsException):
        _ = Variable("x", env)


@pytest.mark.variable
def test_create_variable_in_context():
    with Environment():
        _ = Variable("x")


@pytest.mark.variable
def test_create_variable_no_context_no_env():
    with pytest.raises(NoActiveEnvironmentFoundException):
        _ = Variable("x")


@pytest.mark.variable
def test_create_variable_in_double_context():
    with pytest.raises(MultipleActiveEnvironmentsException):
        with Environment():
            with Environment():
                _ = Variable("x")


@pytest.mark.variable
def test_create_variable_with_same_name_different_evironment():
    env1 = Environment()
    env2 = Environment()
    _ = Variable("x", env1)
    _ = Variable("x", env2)


@pytest.mark.variable
def test_create_variable_with_same_name_different_evironment_context():
    with Environment():
        _ = Variable("x")
    with Environment():
        _ = Variable("x")


@pytest.mark.variable
def test_add_variable_and_float():
    with Environment():
        x = Variable("x")

    result = x + 1
    assert type(result) == Expression
    assert result.num_variables() == 1
    assert result.get_linear(x) == 1


@pytest.mark.variable
def test_add_two_variables():
    with Environment():
        x = Variable("x")
        y = Variable("y")

    result = x + y
    assert type(result) == Expression
    assert result.num_variables() == 2
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1


@pytest.mark.variable
def test_variable_add_expression():
    with Environment():
        x, y, z = Variable("x"), Variable("y"), Variable("z")
    expr = x + y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    result = z + expr
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1
    assert result.get_linear(z) == 1


@pytest.mark.variable
def test_expression_add_variable():
    with Environment():
        x, y, z = Variable("x"), Variable("y"), Variable("z")
    expr = x + y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    result = expr + z
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1
    assert result.get_linear(z) == 1


@pytest.mark.variable
def test_expression_add_variable_twice():
    with Environment():
        x, y, z = Variable("x"), Variable("y"), Variable("z")
    expr = x + y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    result = expr + z
    result = result + z
    assert type(result) == Expression
    assert result.num_variables() == 3
    assert result.get_linear(x) == 1
    assert result.get_linear(y) == 1
    assert result.get_linear(z) == 2


@pytest.mark.variable
def test_expression_instanceadd_variable():
    with Environment():
        x, y, z = Variable("x"), Variable("y"), Variable("z")
    expr = x + y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr += z
    assert type(expr) == Expression
    assert expr.num_variables() == 3
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1
    assert expr.get_linear(z) == 1


@pytest.mark.variable
def test_expression_instanceadd_variable_twice():
    with Environment():
        x, y, z = Variable("x"), Variable("y"), Variable("z")
    expr = x + y
    assert type(expr) == Expression
    assert expr.num_variables() == 2
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1

    expr += z
    expr += z
    assert type(expr) == Expression
    assert expr.num_variables() == 3
    assert expr.get_linear(x) == 1
    assert expr.get_linear(y) == 1
    assert expr.get_linear(z) == 2
