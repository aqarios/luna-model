import pytest

from luna_model import Model, Sense
from luna_model.errors import ModelSenseNotMinimizeError
from luna_model.translator import QuboTranslator


def test_qubo_sense_is_min():
    m = Model()
    x = m.add_variable("x")
    m.objective = -x
    assert QuboTranslator.from_lm(m).sense == Sense.MIN


def test_qubo_from_max_model_raises():
    m = Model(sense=Sense.MAX)
    x = m.add_variable("x")
    m.objective = x
    with pytest.raises(ModelSenseNotMinimizeError):
        QuboTranslator.from_lm(m)
