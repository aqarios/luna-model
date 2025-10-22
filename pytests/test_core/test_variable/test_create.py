import pytest

from luna_model import Environment, Variable
from luna_model.errors import (
    MultipleActiveEnvironmentsError,
    NoActiveEnvironmentFoundError,
    VariableExistsError,
    VariableNamesError,
)


@pytest.mark.variable
def test_create_variable_explicit():
    env = Environment()
    _ = Variable("x", env=env)


@pytest.mark.variable
def test_create_variable_explicit_del_create():
    env = Environment()
    x = Variable("x", env=env)
    del x
    with pytest.raises(VariableExistsError):
        _ = Variable("x", env=env)


@pytest.mark.variable
def test_create_variable_in_context():
    with Environment():
        _ = Variable("x")


@pytest.mark.variable
def test_create_variable_no_context_no_env():
    with pytest.raises(NoActiveEnvironmentFoundError):
        _ = Variable("x")


@pytest.mark.variable
def test_create_variable_in_double_context():
    with pytest.raises(MultipleActiveEnvironmentsError):
        with Environment():
            with Environment():
                _ = Variable("x")


@pytest.mark.variable
def test_create_variable_with_same_name_different_environment():
    env1 = Environment()
    env2 = Environment()
    _ = Variable("x", env=env1)
    _ = Variable("x", env=env2)


@pytest.mark.variable
def test_create_variable_with_same_name_different_environment_context():
    with Environment():
        _ = Variable("x")
    with Environment():
        _ = Variable("x")


@pytest.mark.variable
def test_create_variable_with_invalid_name():
    with Environment():
        with pytest.raises(
            VariableNamesError,
            match="Variable names must start with an alphabetic character",
        ):
            _ = Variable("0")
        with pytest.raises(
            VariableNamesError,
            match="Variable names must only contain alphanumeric characters or '_' or ','",
        ):
            _ = Variable("xß")
