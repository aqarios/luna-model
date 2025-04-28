from pathlib import Path
import pytest
from dimod import lp as dimod_lp
from pyscipopt import Model as ScipModel
from aqmodels import Model, Variable, LpTranslator, Vtype, Bounds

@pytest.fixture
def model_lp_str_bin() -> str:
    m = Model(name="TestModel")
    with m.environment:
        x0 = Variable("x0")
        m.objective = x0 * 1
        x1 = Variable("x1", vtype=Vtype.Binary)
        m.objective += x0 * x1 * -1
        x2 = Variable("x2")
        x3 = Variable("x3", vtype=Vtype.Binary, bounds=Bounds(0, 30))
        x4 = Variable("x4")
        m.objective += (
            x0 * x1 * 12.213
            + x1 * x2 * 0.5
            + x0 * x2 * -3
            + 1
            + x0 * x3 * 1848482
            + x1 * x4
        )
        m.constraints.add_constraint(x0 + x2 <= 1)
        m.constraints.add_constraint(x0 + x2 <= 1, "my_constraint")
    return LpTranslator.from_model(m)

@pytest.fixture
def model_lp_str_fancy() -> str:
    m = Model(name="TestModel")
    with m.environment:
        x0 = Variable("x0")
        m.objective = x0 * 1
        x1 = Variable("x1", vtype=Vtype.Real)
        m.objective += x0 * x1 * -1
        x2 = Variable("x2")
        x3 = Variable("x3", vtype=Vtype.Integer, bounds=Bounds(0, 30))
        x4 = Variable("x4")
        m.objective += (
            x0 * x1 * 12.213
            + x1 * x2 * 0.5
            + x0 * x2 * -3
            + 1
            + x0 * x3 * 1848482
            + x1 * x4
        )
        m.constraints.add_constraint(x0 + x2 <= 1)
        m.constraints.add_constraint(x0 + x2 <= 1, "my_constraint")
    return LpTranslator.from_model(m)


@pytest.mark.translator
def test_translate_to_cqm(model_lp_str_bin: str):
    _ = dimod_lp.loads(model_lp_str_bin)


@pytest.mark.translator
def test_translate_to_zib():
    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(str(Path(__file__).parent / "model.lp"))
