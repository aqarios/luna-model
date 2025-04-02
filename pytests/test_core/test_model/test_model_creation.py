import pytest

from aqmodels import (
    Environment,
    Model,
    MultipleActiveEnvironmentsError,
)


@pytest.mark.model
def test_create_model_explicit():
    env = Environment()
    model = Model(env)
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
