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


# @pytest.mark.variable
# def test_create_variable_with_same_name_different_evironment():
#     _ = Variable("x")
#     _ = Variable("x", env=Environment())
#
#
# @pytest.mark.variable
# def test_add_variable_and_float():
#     x = Variable("x")
#     result = x + 1
#     assert type(result) == Expression
#     assert result.num_variables() == 1
#     assert result.get_linear(x) == 1
#
#
# @pytest.mark.variable
# def test_add_two_variables():
#     x = Variable("x")
#     y = Variable("y")
#     result = x + y
#     assert type(result) == Expression
#     assert result.num_variables() == 2
#     assert result.get_linear(x) == 1
#     assert result.get_linear(y) == 1
#
#
# @pytest.mark.variable
# def test_expression_add_variable():
#     x, y, z = Variable("x"), Variable("y"), Variable("z")
#     expr = x + y
#     expr2 = expr + z
#
#
# @pytest.mark.variable
# def test_variable_add_expression():
#     x, y, z = Variable("x"), Variable("y"), Variable("z")
#     expr = x + y
#     expr2 = z + expr
