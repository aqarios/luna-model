import pytest

from aqmodels import (
    Environment,
    Model,
)

from aqmodels.errors import MultipleActiveEnvironmentsError


@pytest.mark.model
def test_create_model_explicit():
    env = Environment()
    model = Model(env=env)
    assert isinstance(model, Model)


@pytest.mark.model
def test_create_model_no_env():
    model = Model()
    assert isinstance(model, Model)


@pytest.mark.model
def test_create_model_in_double_context():
    with pytest.raises(MultipleActiveEnvironmentsError):
        with Environment():
            with Environment():
                _ = Model()
