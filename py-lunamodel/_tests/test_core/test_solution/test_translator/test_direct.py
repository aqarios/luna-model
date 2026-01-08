from pathlib import Path
import numpy as np
from luna_model import Solution, Timer, TranslationTarget

NOT_TEST_SCIP = True
try:
    from pyscipopt import Model as ScipModel
    NOT_TEST_IBM = False
except ImportError:
    NOT_TEST_SCIP = True

NOT_TEST_IBM = True
try:
    from _tests.test_core.test_solution.test_translator.test_ibm_translator import (
        compute_result,
        controlled_lm,
        controlled_qp,
        extract,
    )
    from qiskit_optimization.translators import from_docplex_mp
    NOT_TEST_IBM = False
except ImportError:
    NOT_TEST_IBM = True

from _tests.test_core.utils import make_seed, random, random_int
from .fixtures import *  # noqa: F403
from ..test_from_dict import vars


def make_scip_model(zib_model):
    lp_str = zib_model.to(TranslationTarget.Lp)
    lp_filepath = Path(__file__).parent / "model.lp"
    with open(lp_filepath, "w") as f:
        f.write(lp_str)

    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(lp_filepath)
    scip_model.optimize()
    return scip_model


@pytest.mark.skipif(NOT_TEST_SCIP, reason="pyscipopt is required for test")
def test_sol_direct_from_scip(zib_model):
    timer = Timer.start()
    scip_model = make_scip_model(zib_model)
    timing = timer.stop()
    truth_sample = {x.name: scip_model.getVal(x) for x in scip_model.getVars()}

    with zib_model.environment:
        sol = Solution.from_(scip_model, timing=timing)

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


def test_sol_direct_from_qctrl():
    rand = Random(make_seed())
    sample_len = rand.randint(2, 5)
    num_samples = rand.randint(1, max(sample_len // 2, 1))
    fake_result, _ = fake_qctrl_result(
        rand, sample_len, random(random_int(rand)), num_samples
    )
    with Environment():
        _ = [Variable(f"x{i}") for i in range(sample_len)]
        sol = Solution.from_(fake_result)

    samples = sol.samples.tolist()
    assert len(samples) == num_samples, "number of samples does not match"
    assert len(samples[0]) == sample_len, "sample len (num variables) does not match"
    assert sol.raw_energies == None
    assert len(sol.counts.tolist()) == num_samples
    assert sol.runtime is None

    for result in sol.results:
        assert result.constraints is None
        assert result.feasible is None

    results = list(sol.results)
    assert len(results) == num_samples

def test_sol_direct_from_aws_with_warning(aws_model, aws_result):
    import warnings
    warnings.filterwarnings("error") # so we can catch it...
    with pytest.raises(UserWarning):
        with aws_model.environment:
            _ = Solution.from_(aws_result, counts=[1, 2, 3, 4])

def test_sol_direct_from_aws(aws_model, aws_result):
    with aws_model.environment:
        sol = Solution.from_(aws_result)

    (sol_agg, indices, _) = np.unique(
        aws_result["samples"], return_index=True, return_counts=True, axis=0
    )
    assert sol.samples.tolist() == sol_agg.tolist()
    for i, result in enumerate(sol.results):
        assert result.raw_energy == aws_result["energies"][indices[i]]
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None


def test_sol_direct_from_np(np_model, np_result):
    res, energies = np_result
    with np_model.environment:
        sol = Solution.from_(res, energies=energies)

    (sol_agg, indices, _) = np.unique(
        res, return_index=True, return_counts=True, axis=0
    )

    assert sol.samples.tolist() == sol_agg.tolist()
    for i, result in enumerate(sol.results):
        assert result.raw_energy == energies[indices[i]]
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None


@pytest.mark.skipif(NOT_TEST_IBM, reason="qiskit is required for test")
def test_sol_direct_from_ibm():
    lm = controlled_lm()
    timer = Timer.start()
    qp = controlled_qp()
    res = compute_result(qp)
    timing = timer.stop()
    with lm.environment:
        sol = Solution.from_(res, quadratic_program=qp, timing=timing)

    truth_samples, truth_energies, truth_counts = extract(res, qp)
    assert len(sol.samples) == len(truth_samples), "sample lengths not matching"
    assert sol.samples.tolist() == truth_samples
    assert sol.raw_energies is not None
    assert len(sol.raw_energies) == len(truth_energies)
    assert sol.raw_energies.tolist() == truth_energies
    assert len(sol.counts) == len(truth_counts)
    assert sol.counts.tolist() == truth_counts
    assert len(sol.counts) == len(sol.samples)
    assert sol.runtime is not None
    assert np.isclose(sol.runtime.total.total_seconds(), timing.total_seconds)
    assert np.isclose(sol.runtime.total_seconds, timing.total.total_seconds())
    assert sol.runtime.qpu is None
    assert sol.obj_values is None

    results = list(sol.results)
    assert len(results) == len(sol.samples)
    for i, result in enumerate(results):
        assert result.counts == sol.counts.tolist()[i]  # type: ignore
        assert list(result.sample) == list(sol.samples[i])
        assert result.obj_value is None
        assert result.raw_energy == sol.raw_energies.tolist()[i]  # type: ignore
        assert result.constraints is None
        assert result.feasible is None


def test_sol_direct_from_dwave(dwave_result):
    with mock_env(3):
        sol = Solution.from_(dwave_result.sampleset)

    assert sol.samples.tolist() == dwave_result.samples
    assert sol.counts.tolist() == dwave_result.counts
    assert sol.raw_energies is not None
    assert sol.raw_energies.tolist() == dwave_result.energy
    assert sol.obj_values is None
    assert sol.runtime is None

    for result in sol.results:
        assert result.constraints is None
        assert result.feasible is None

    results = list(sol.results)
    assert 3 == len(results)


def test_sol_direct_from_dict():
    (x, y, z), env = vars(3, Vtype.Binary)
    sample = {x: 0, y: 0, z: 1}
    with env:
        sol = Solution.from_(sample)
    assert sol.samples.tolist() == [[0, 0, 1]]


def test_sol_direct_from_dicts():
    (x, y, z), env = vars(3, Vtype.Binary)
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        {x: 0, y: 1, z: 1},  # -2
    ]
    with env:
        sol = Solution.from_(samples)
    assert sol.samples.tolist() == [
        [0, 0, 1],
        [1, 1, 1],
        [1, 0, 0],
        [0, 1, 1],
    ]
