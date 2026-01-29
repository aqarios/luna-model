import pickle

import pytest

from luna_model import Model

from ..test_serialization.creators import serializable_objects


def test_pickle_empty_model():
    m = Model()
    blob = pickle.dumps(m)
    mp = pickle.loads(blob)
    assert m.equal_contents(mp)


def test_pickle_empty_model_named():
    m = Model("Name")
    blob = pickle.dumps(m)
    mp = pickle.loads(blob)
    assert m.equal_contents(mp)


@pytest.mark.parametrize("model", serializable_objects([Model]))
def test_pickle_model(model: Model):
    blob = pickle.dumps(model)
    model_loaded = pickle.loads(blob)
    assert model.equal_contents(model_loaded)
