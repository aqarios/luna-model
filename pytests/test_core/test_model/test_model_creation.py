import pytest

from aq_models import (
    Environment,
    Model,
    MultipleActiveEnvironmentsException,
    NoActiveEnvironmentFoundException,
)


@pytest.mark.model
def test_create_model_explicit():
    env = Environment()
    model = Model(env)
    assert isinstance(model, Model)


@pytest.mark.model
def test_create_model_no_env():
    with pytest.raises(NoActiveEnvironmentFoundException):
        _ = Model()


@pytest.mark.model
def test_create_model_in_context():
    with pytest.raises(NoActiveEnvironmentFoundException):
        _ = Model()


@pytest.mark.model
def test_create_model_in_double_context():
    with pytest.raises(MultipleActiveEnvironmentsException):
        with Environment():
            with Environment():
                _ = Model()
