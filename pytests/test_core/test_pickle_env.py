import pytest
from .test_serialization.creators import serializable_objects
import pickle

from aqmodels import Environment


@pytest.mark.parametrize("env", serializable_objects([Environment]))
def test_pickle_env(env: Environment):
    blob = pickle.dumps(env)
    loaded = pickle.loads(blob)
    assert env.equal_contents(loaded)
