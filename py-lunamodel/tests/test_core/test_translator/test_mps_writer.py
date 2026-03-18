"""Tests that Luna's MPS writer produces files equivalent to the original model."""

from pathlib import Path

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
def test_roundtrip_gurobi(mps_file: str, tmp_path: Path):
    """Read MPS with Luna, write back to MPS, read with Gurobi, compare."""
    mps_path = MPS_DIR / mps_file
    lm_model = MpsTranslator.to_lm(mps_path)
    roundtrip_path = tmp_path / mps_file
    MpsTranslator.from_lm(lm_model, roundtrip_path)
    Path("./debug.mps").write_text(roundtrip_path.read_text())
    gp_norm = extract_gurobi(gp.read(str(roundtrip_path)))
    lm_norm = extract_luna(lm_model)
    ok, kind, msg = compare_models(gp_norm, lm_norm)
    assert ok, (
        f"Roundtrip Gurobi mismatch for {mps_file} [{kind.value}]: {msg}\n"
        f"{model_details(kind, lm_norm)}"
    )


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
@pytest.mark.parametrize("mps_file", MPS_FILES)
def test_roundtrip_scip(mps_file: str, tmp_path: Path):
    """Read MPS with Luna, write back to MPS, read with SCIP, compare."""
    mps_path = MPS_DIR / mps_file
    lm_model = MpsTranslator.to_lm(mps_path)
    roundtrip_path = tmp_path / mps_file
    MpsTranslator.from_lm(lm_model, roundtrip_path)
    scip_model = ScipModel()
    scip_model.readProblem(str(roundtrip_path))
    lm_norm = extract_luna(lm_model)
    lm_var_names = {v.name for v in lm_norm.variables}
    scip_norm = extract_scip(scip_model, var_names=lm_var_names)
    ok, kind, msg = compare_models(scip_norm, lm_norm)
    assert ok, (
        f"Roundtrip SCIP mismatch for {mps_file} [{kind.value}]: {msg}\n"
        f"{model_details(kind, lm_norm)}"
    )
