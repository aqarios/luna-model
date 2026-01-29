import pytest
from luna_model import Model, Variable, Vtype
from luna_model.translator import LpTranslator
from luna_model.errors import IllegalConstraintNameError, TranslationError

ILLEGAL_WORD_START = [
    "0",
    "0name",
    ".name",
    "1word",
    "nan",
    "inf",
    "nanometer",
    "infeasiblility",
]


@pytest.mark.parametrize("word", ILLEGAL_WORD_START)
def test_illegal_words(word: str):
    model = Model(f"test_{word}")
    with model.environment:
        x = Variable("x")
        y = Variable("y")
    model.objective = x * y
    with pytest.raises(IllegalConstraintNameError):
        model.constraints.add_constraint(x + y * 3 <= 10, word)


@pytest.mark.parametrize("word", ILLEGAL_WORD_START)
def test_illegal_vars(word: str):
    model = Model(f"test_{word}")
    with model.environment:
        x = Variable("x")
        y = Variable("y", vtype=Vtype.SPIN)
    model.objective = x * y
    with pytest.raises(TranslationError):
        _ = LpTranslator.from_lm(model)
