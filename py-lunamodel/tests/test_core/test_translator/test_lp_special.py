import os
from pathlib import Path

import pytest

from luna_model.translator import LpTranslator
from tests.test_core.test_translator.utils_model_comparison import NOT_RUN_SCIP, ScipModel


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
def test_zero_variables():
    base_file = Path(__file__).parent / "lp_string.lp"
    model = LpTranslator.to_lm(base_file)

    scip = ScipModel()
    scip.readProblem(base_file)

    scip_file_out = Path(__file__).parent / "lp_scip.lp"
    scip_file_out.touch()
    scip.writeProblem(scip_file_out)

    model_from_scip = LpTranslator.to_lm(scip_file_out)
    model_from_scip.name = model.name

    os.remove(scip_file_out)
    assert model.equal_contents(model_from_scip)
    # OLD
    # model = LpTranslator.to_lm(Path(__file__).parent / "lp_string.lp")
    # model_str = LpTranslator.from_lm(model)

    # out_file = Path(__file__).parent / "lp_out.lp"
    # out_file.touch()
    # out_file.write_text(model_str)
    # scip = ScipModel()
    # scip.readProblem(out_file)

    # scip_file = Path(__file__).parent / "lp_scip.lp"
    # scip_file.touch()
    # scip.writeProblem(scip_file)

    # model_from_scip = LpTranslator.to_lm(scip_file)
    # model_from_scip.name = model.name
    # os.remove(out_file)
    # os.remove(scip_file)
    # print(model.equal_contents(model_from_scip))
    # assert False
