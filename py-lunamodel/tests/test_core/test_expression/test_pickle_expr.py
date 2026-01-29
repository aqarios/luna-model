import pickle

import pytest

from luna_model import Expression

from ..test_serialization.creators import serializable_objects


@pytest.mark.parametrize("expr", serializable_objects([Expression]))
def test_pickle_expr(expr: Expression):
    blob = pickle.dumps(expr)
    loaded = pickle.loads(blob)
    assert expr.equal_contents(loaded)
