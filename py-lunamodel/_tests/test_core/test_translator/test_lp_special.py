import os
import sys
from pathlib import Path

import pytest
from luna_model.translator import LpTranslator

NOT_RUN_SCIP = False
try:
    from pyscipopt import Model as ScipModel
except ImportError as _:
    print(
        "SCIP is not installed and thus, the Gurobi tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_SCIP = True


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
@pytest.mark.translator()
def test_zero_variables():
    model = LpTranslator.to_aq(Path(__file__).parent / "lp_string.lp")
    model_str = LpTranslator.from_aq(model)

    out_file = Path(__file__).parent / "lp_out.lp"
    out_file.touch()
    out_file.write_text(model_str)
    scip = ScipModel()
    scip.readProblem(out_file)

    scip_file = Path(__file__).parent / "lp_scip.lp"
    scip_file.touch()
    scip.writeProblem(scip_file)

    model_from_scip = LpTranslator.to_aq(scip_file)
    model_from_scip.name = model.name
    os.remove(out_file)
    os.remove(scip_file)
    assert model.equal_contents(model_from_scip)
