import pytest

from aq_models import (
    Environment,
    MultipleActiveEnvironmentsException,
    NoActiveEnvironmentFoundException,
    Variable,
    VariableExistsException,
)


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
