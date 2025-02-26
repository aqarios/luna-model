import pytest

from aq_models import Model
from aq_models import Environment
from aq_models import NoActiveEnvironmentFoundException
from aq_models import MultipleActiveEnvironmentsException


@pytest.mark.model
def test_create_model_explicit():
    env = Environment()
    model = Model(env)
    assert type(model) == Model


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
