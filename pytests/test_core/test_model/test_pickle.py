import pytest
from ..test_serialization.creators import create_pickable_objects
import pickle

from aqmodels import Model


def test_pickle_empty_model():
    m = Model()
    blob = pickle.dumps(m)
    mp = pickle.loads(blob)
    assert m.equal_contents(mp)


@pytest.mark.parametrize("model", create_pickable_objects())
def test_pickle_model(model: Model):
    blob = pickle.dumps(model)
    model_loaded = pickle.loads(blob)
    assert model.equal_contents(model_loaded)
