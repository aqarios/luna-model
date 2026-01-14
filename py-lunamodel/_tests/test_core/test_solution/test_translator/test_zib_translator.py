import sys
from pathlib import Path

import numpy as np
import pytest

NOT_RUN_SCIP = False
try:
    from pyscipopt import Model as ScipModel
except ImportError as _:
    print(
        "SCOP is not installed and thus, the Gurobi tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_SCIP = True

from luna_model import Model, Timer
from luna_model.translator import LpTranslator, ZibTranslator

from .fixtures import zib_model, zib_model_quadratic


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
def test_zib_translator(zib_model: Model):
    lp_str = LpTranslator.from_lm(zib_model)
    lp_filepath = Path(__file__).parent / "model.lp"
    with open(lp_filepath, "w") as f:
        f.write(lp_str)

    timer = Timer.start()
    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(lp_filepath)
    scip_model.optimize()
    timing = timer.stop()

    truth_sample = {x.name: scip_model.getVal(x) for x in scip_model.getVars()}

    sol = ZibTranslator.to_lm(scip_model, timing=timing, env=zib_model.environment)
    assert len(sol.samples) == 1
    assert sol.raw_energies == None
    assert len(sol.counts) == 1
    assert len(sol.counts) == len(sol.samples)
    assert sol.runtime is not None
    assert np.isclose(
        sol.runtime.total.total_seconds(), timing.total_seconds, atol=1e-5
    )
    assert np.isclose(
        sol.runtime.total_seconds, timing.total.total_seconds(), atol=1e-5
    )
    assert sol.runtime.qpu is None
    assert sol.obj_values is None
    assert sol.raw_energies is None

    results = list(sol.results)
    assert len(results) == len(sol.samples)
    for i, result in enumerate(results):
        assert result.counts == sol.counts.tolist()[i]  # type: ignore
        assert list(result.sample) == list(sol.samples[i])
        assert result.obj_value is None
        # assert result.raw_energy == sol.raw_energies.tolist()[i]  # type: ignore
        assert result.constraints is None
        assert result.feasible is None

    assert len(sol.samples) == 1
    sample = sol.samples[0]
    for key, value in truth_sample.items():
        v = zib_model.environment.get_variable(key)
        assert np.isclose(sample[v], value, atol=1e-5)


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
def test_zib_translator_quadratic(zib_model_quadratic: Model):
    lp_str = LpTranslator.from_lm(zib_model_quadratic)
    lp_filepath = Path(__file__).parent / "model.lp"
    with open(lp_filepath, "w") as f:
        f.write(lp_str)

    timer = Timer.start()
    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(lp_filepath)
    scip_model.optimize()
    timing = timer.stop()
    _ = ZibTranslator.to_lm(scip_model, timing=timing, env=zib_model_quadratic.environment)
    truth_sample = {
        x.name: scip_model.getVal(x)
        for x in scip_model.getVars()
        if x.name in zib_model_quadratic.environment
    }

    sol = ZibTranslator.to_lm(
        scip_model, timing=timing, env=zib_model_quadratic.environment
    )
    assert len(sol.samples) == 1
    assert sol.raw_energies == None
    assert len(sol.counts) == 1
    assert len(sol.counts) == len(sol.samples)
    assert sol.runtime is not None
    assert np.isclose(
        sol.runtime.total.total_seconds(), timing.total_seconds, atol=1e-5
    )
    assert np.isclose(
        sol.runtime.total_seconds, timing.total.total_seconds(), atol=1e-5
    )
    assert sol.runtime.qpu is None
    assert sol.obj_values is None
    assert sol.raw_energies is None

    results = list(sol.results)
    assert len(results) == len(sol.samples)
    for i, result in enumerate(results):
        assert result.counts == sol.counts.tolist()[i]  # type: ignore
        assert list(result.sample) == list(sol.samples[i])
        assert result.obj_value is None
        # assert result.raw_energy == sol.raw_energies.tolist()[i]  # type: ignore
        assert result.constraints is None
        assert result.feasible is None

    assert len(sol.samples) == 1
    sample = sol.samples[0]
    for key, value in truth_sample.items():
        v = zib_model_quadratic.environment.get_variable(key)
        assert np.isclose(sample[v], value, atol=1e-5)


def test_read_coins():
    lp_filepath = Path(__file__).parent / "coins.lp"
    _ = LpTranslator.to_lm(lp_filepath)
