from pathlib import Path
import sys

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

from luna_model import Bounds, Model, Timer, Variable, Vtype
from luna_model.translator import LpTranslator, ZibTranslator


@pytest.fixture
def model() -> Model:
    m = Model(name="TestModel")
    with m.environment:
        pennies = Variable("Pennies", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        nickels = Variable("Nickels", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        dimes = Variable("Dimes", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        quarters = Variable("Quarters", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        dollars = Variable("Dollars", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        cu = Variable("Cu", vtype=Vtype.Real, bounds=Bounds(upper=1000))
        ni = Variable("Ni", vtype=Vtype.Real, bounds=Bounds(upper=50))
        zi = Variable("Zi", vtype=Vtype.Real, bounds=Bounds(upper=50))
        mn = Variable("Mn", vtype=Vtype.Real, bounds=Bounds(upper=50))
    m.objective = (
        0.01 * pennies + 0.05 * nickels + 0.1 * dimes + 0.25 * quarters + 1 * dollars
    )
    m.constraints += (
        0.06 * pennies
        + 3.8 * nickels
        + 2.1 * dimes
        + 5.2 * quarters
        + 7.2 * dollars
        - cu
        == 0,
        "Copper",
    )
    m.constraints += (
        1.2 * nickels + 0.2 * dimes + 0.5 * quarters + 0.2 * dollars - ni == 0,
        "Nickel",
    )
    m.constraints += 2.4 * pennies + 0.5 * dollars - zi == 0, "Zinc"
    m.constraints += 0.3 * dollars - mn == 0, "Manganese"
    return m


@pytest.fixture
def model_quadratic() -> Model:
    m = Model(name="TestModel")
    with m.environment:
        pennies = Variable("Pennies", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        nickels = Variable("Nickels", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        dimes = Variable("Dimes", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        quarters = Variable("Quarters", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        dollars = Variable("Dollars", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        cu = Variable("Cu", vtype=Vtype.Real, bounds=Bounds(upper=1000))
        ni = Variable("Ni", vtype=Vtype.Real, bounds=Bounds(upper=50))
        zi = Variable("Zi", vtype=Vtype.Real, bounds=Bounds(upper=50))
        mn = Variable("Mn", vtype=Vtype.Real, bounds=Bounds(upper=50))
    m.objective = (
        0.01 * pennies * nickels
        + 0.05 * nickels * dollars
        + 0.1 * dimes
        + 0.25 * quarters
        + 1 * dollars
        + 5 * nickels * dollars
    )
    m.constraints += (
        0.06 * pennies
        + 3.8 * nickels
        + 2.1 * dimes
        + 5.2 * quarters
        + 7.2 * dollars
        - cu
        == 0,
        "Copper",
    )
    m.constraints += (
        1.2 * nickels + 0.2 * dimes + 0.5 * quarters + 0.2 * dollars - ni == 0,
        "Nickel",
    )
    m.constraints += 2.4 * pennies + 0.5 * dollars - zi == 0, "Zinc"
    m.constraints += 0.3 * dollars - mn == 0, "Manganese"
    return m


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
@pytest.mark.solution_translation
def test_zib_translator(model: Model):
    lp_str = LpTranslator.from_aq(model)
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

    sol = ZibTranslator.to_aq(scip_model, timing=timing, env=model.environment)
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
        v = model.environment.get_variable(key)
        assert np.isclose(sample[v], value, atol=1e-5)


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
@pytest.mark.solution_translation
def test_zib_translator_quadratic(model_quadratic: Model):
    lp_str = LpTranslator.from_aq(model_quadratic)
    lp_filepath = Path(__file__).parent / "model.lp"
    with open(lp_filepath, "w") as f:
        f.write(lp_str)

    timer = Timer.start()
    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(lp_filepath)
    scip_model.optimize()
    timing = timer.stop()
    _ = ZibTranslator.to_aq(scip_model, timing=timing, env=model_quadratic.environment)
    truth_sample = {
        x.name: scip_model.getVal(x)
        for x in scip_model.getVars()
        if x.name in model_quadratic.environment
    }

    sol = ZibTranslator.to_aq(
        scip_model, timing=timing, env=model_quadratic.environment
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
        v = model_quadratic.environment.get_variable(key)
        assert np.isclose(sample[v], value, atol=1e-5)


@pytest.mark.solution_translation
def test_read_coins():
    lp_filepath = Path(__file__).parent / "coins.lp"
    _ = LpTranslator.to_aq(lp_filepath)
