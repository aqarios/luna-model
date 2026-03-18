"""Tests that Luna's MPS reader produces models equivalent to Gurobi and SCIP."""

import pytest

from luna_model.translator import MpsTranslator

from .utils_model_comparison import (
    MPS_DIR,
    MPS_FILES,
    NOT_RUN_GUROBI,
    NOT_RUN_SCIP,
    ScipModel,
    compare_models,
    extract_gurobi,
    extract_luna,
    extract_scip,
    gp,
    model_details,
)


@pytest.mark.skipif(NOT_RUN_GUROBI, reason="Gurobi is required for test")
@pytest.mark.parametrize("mps_file", MPS_FILES)
def test_gurobi_and_luna_model_equal(mps_file: str):
    mps_path = MPS_DIR / mps_file
    gp_norm = extract_gurobi(gp.read(str(mps_path)))
    lm_norm = extract_luna(MpsTranslator.to_lm(mps_path))
    ok, kind, msg = compare_models(gp_norm, lm_norm)
    assert ok, (
        f"Gurobi/Luna mismatch for {mps_file} [{kind.value}]: {msg}\n"
        f"{model_details(kind, lm_norm)}"
    )


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
@pytest.mark.parametrize("mps_file", MPS_FILES)
def test_scip_and_luna_model_equal(mps_file: str):
    mps_path = MPS_DIR / mps_file
    scip_model = ScipModel()
    scip_model.readProblem(str(mps_path))
    lm_model = MpsTranslator.to_lm(mps_path)
    lm_var_names = {v.name for v in lm_model.environment.variables()}
    scip_norm = extract_scip(scip_model, var_names=lm_var_names)
    lm_norm = extract_luna(lm_model)
    ok, kind, msg = compare_models(scip_norm, lm_norm)
    assert ok, (
        f"SCIP/Luna mismatch for {mps_file} [{kind.value}]: {msg}\n"
        f"{model_details(kind, lm_norm)}"
    )


@pytest.mark.skipif(NOT_RUN_GUROBI or NOT_RUN_SCIP, reason="Both Gurobi and SCIP are required")
@pytest.mark.parametrize("mps_file", MPS_FILES)
def test_gurobi_and_scip_model_equal(mps_file: str):
    mps_path = MPS_DIR / mps_file
    gp_norm = extract_gurobi(gp.read(str(mps_path)))
    scip_model = ScipModel()
    scip_model.readProblem(str(mps_path))
    gp_var_names = {v.name for v in gp_norm.variables}
    scip_norm = extract_scip(scip_model, var_names=gp_var_names)
    ok, kind, msg = compare_models(gp_norm, scip_norm)
    assert ok, f"Gurobi/SCIP mismatch for {mps_file} [{kind.value}]: {msg}"
