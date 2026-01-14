import sys
from pathlib import Path

import pytest
from dimod import lp as dimod_lp

NOT_RUN_SCIP = False
try:
    from pyscipopt import Model as ScipModel
except ImportError as _:
    print(
        "SCOP is not installed and thus, the Gurobi tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_SCIP = True

from luna_model import Bounds, Model, Variable, Vtype
from luna_model.translator import LpTranslator


@pytest.fixture()
def model_lp_str_bin() -> str:
    m = Model(name="TestModel")
    with m.environment:
        x0 = Variable("x0")
        m.objective = x0 * 1
        x1 = Variable("x1", vtype=Vtype.BINARY)
        m.objective += x0 * x1 * -1
        x2 = Variable("x2")
        x3 = Variable("x3", vtype=Vtype.BINARY)
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
    return LpTranslator.from_aq(m)


@pytest.fixture()
def model_lp_str_fancy() -> str:
    m = Model(name="TestModel")
    with m.environment:
        x0 = Variable("x0")
        m.objective = x0 * 1
        x1 = Variable("x1", vtype=Vtype.REAL)
        m.objective += x0 * x1 * -1
        x2 = Variable("x2")
        x3 = Variable("x3", vtype=Vtype.INTEGER, bounds=Bounds(0, 30))
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
    return LpTranslator.from_aq(m)


def test_translate_to_cqm(model_lp_str_bin: str):
    _ = dimod_lp.loads(model_lp_str_bin)


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
def test_translate_to_zib():
    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(str(Path(__file__).parent / "model.lp"))
