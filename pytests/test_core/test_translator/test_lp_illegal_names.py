import pytest

from aqmodels import Model, Variable, LpTranslator
from aqmodels.errors import IllegalConstraintNameError

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


@pytest.mark.translator
@pytest.mark.parametrize("word", ILLEGAL_WORD_START)
def test_illegal_words(word: str):
    model = Model(f"test_{word}")
    with model.environment:
        x = Variable("x")
        y = Variable("y")
    model.objective = x * y
    with pytest.raises(IllegalConstraintNameError):
        model.constraints.add_constraint(x + y * 3 <= 10, word)
        _ = LpTranslator.from_aq(model)
