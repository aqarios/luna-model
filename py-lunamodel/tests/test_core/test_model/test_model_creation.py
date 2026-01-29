import pytest

from luna_model import Environment, Model
from luna_model.errors import MultipleActiveEnvironmentsError


def test_create_model_explicit():
    env = Environment()
    model = Model(env=env)
    assert isinstance(model, Model)


def test_create_model_no_env():
    model = Model()
    assert isinstance(model, Model)


def test_create_model_in_double_context():
    with pytest.raises(MultipleActiveEnvironmentsError), Environment(), Environment():
        _ = Model()
