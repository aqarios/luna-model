from pathlib import Path
import pytest
from dimod import lp as dimod_lp
from pyscipopt import Model as ScipModel
from aqmodels import Model, Variable, LpTranslator, Vtype


@pytest.fixture
def model_lp_str() -> str:
    model = Model("test_model")
    with model.environment:
        x = Variable("x")
        y = Variable("y", vtype=Vtype.Binary)
        z = Variable("z", vtype=Vtype.Binary)
    model.objective = x + y * z
    model.constraints += x - z >= 3
    model.constraints += x + y <= 5
    return LpTranslator.from_model(model)


@pytest.mark.translator
def test_translate_to_cqm(model_lp_str: str):
    _ = dimod_lp.loads(model_lp_str)


@pytest.mark.translator
def test_translate_to_zib():
    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(Path("./model.lp"))

