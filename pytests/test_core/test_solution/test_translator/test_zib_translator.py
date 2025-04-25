import pytest
from pyscipopt import Model as ScipModel

from aqmodels import Model, LpTranslator, Variable, Bounds, Vtype
from aqmodels.translator import ZibTranslator


@pytest.fixture
def model() -> Model:
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
    return m


@pytest.mark.solution_translation
def test_zib_translator(model: Model):
    lp_str = LpTranslator.from_model(model)
    lp_filepath = "./pytests/test_core/test_solution/test_translator/model.lp"
    with open(lp_filepath, "w") as f:
        f.write(lp_str)

    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(lp_filepath)
    scip_model.optimize()

    _ = ZibTranslator.from_zib(scip_model, timing=None, env=model.environment)


@pytest.mark.solution_translation
def test_read_coins():
    lp_filepath = "./pytests/test_core/test_solution/test_translator/coins.lp"
    _ = LpTranslator.to_model(lp_filepath)
