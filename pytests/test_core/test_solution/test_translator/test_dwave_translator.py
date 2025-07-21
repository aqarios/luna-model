from contextlib import nullcontext as does_not_raise
from random import Random

import numpy as np
import pytest
from dimod import SampleSet, Vartype, as_samples
from dwave.samplers import SimulatedAnnealingSampler

from aqmodels import Environment, Timer, Variable, Vtype
from aqmodels.errors import SampleIncompatibleVtypeError, SampleUnexpectedVariableError
from aqmodels.translator import DwaveTranslator
from pytests.test_core.utils import generate_bqms, make_seed, random_int


def mock_env(n_variables: int, vtype: Vtype = Vtype.Binary) -> Environment:
    env = Environment()
    with env:
        for i in range(n_variables):
            _ = Variable(str(f"x{i}"), vtype=vtype)
    return env


@pytest.mark.solution_translation
def test_sampleset_translator_constructed():
    samples_raw = [
        {"x0": 0, "x1": 1, "x2": 1},
        {"x0": 0, "x1": 0, "x2": 1},
        {"x0": 0, "x1": 1, "x2": 0},
    ]
    samples = [[v for v in sample.values()] for sample in samples_raw]
    counts = [1, 2, 3]
    energy = [-1, 0, 1]
    sampleset = SampleSet.from_samples(
        as_samples(samples_raw),
        "BINARY",
        energy,
        num_occurrences=np.array(counts),
    )

    with mock_env(3):
        sol = DwaveTranslator.to_aq(sampleset)

    assert sol.samples.tolist() == samples
    assert sol.counts.tolist() == counts
    assert sol.raw_energies.tolist() == energy
    assert sol.obj_values is None
    assert sol.runtime is None

    for result in sol.results:
        assert result.constraints is None
        assert result.feasible is None

    results = list(sol.results)
    assert len(results) == 3


@pytest.mark.solution_translation
def test_sampleset_translator_sa_random_models():
    rand = Random(make_seed())
    bqms = generate_bqms(20, rand)
    for bqm in bqms:
        timer = Timer.start()
        sampler = SimulatedAnnealingSampler()
        sampleset: SampleSet = sampler.sample(bqm, num_reads=128, seed=random_int(rand))
        timing = timer.stop()
        vtype = Vtype.Binary if bqm.vartype == Vartype.BINARY else Vtype.Spin
        env = mock_env(bqm.num_variables, vtype=vtype)
        sol = DwaveTranslator.to_aq(sampleset, timing, env=env)

        sampleset_agg = sampleset.aggregate()

        dimod_positions = {v: i for i, v in enumerate(bqm.variables)}
        samples_ordered = []
        for sample in sampleset_agg.samples():
            dimod_np = np.zeros(len(bqm.variables), dtype=int)
            for v, pos in dimod_positions.items():
                dimod_np[pos] = sample[v]  # type: ignore
            samples_ordered.append(dimod_np.tolist())

        assert len(sol.samples) == len(sampleset_agg.record.sample)
        for sample in sol.samples.tolist():
            assert sample in samples_ordered
        # assert sol.samples.tolist() == samples_ordered
        assert sol.counts.tolist() == sampleset_agg.record.num_occurrences.tolist()
        assert len(sol.counts) == len(sol.samples)
        assert sol.runtime is not None
        assert sol.runtime.total.total_seconds() > 0
        assert sol.runtime.total_seconds > 0
        assert sol.runtime.qpu is None
        assert sol.obj_values is None
        assert sol.raw_energies.tolist() == sampleset_agg.record.energy.tolist()

        results = list(sol.results)
        assert len(results) == len(sol.samples)
        for i, result in enumerate(results):
            assert result.counts == sol.counts.tolist()[i]
            assert list(result.sample) == list(sol.samples[i])
            assert result.obj_value is None
            assert result.raw_energy == sol.raw_energies.tolist()[i]
            assert result.constraints is None
            assert result.feasible is None


@pytest.mark.solution_translation
def test_sampleset_translator_error_handling():
    samples_raw = [{"x0": -1, "x1": 1, "x2": 1}]
    samples = [[v for v in sample.values()] for sample in samples_raw]
    energy = [-5]
    sampleset = SampleSet.from_samples(as_samples(samples_raw), "SPIN", energy)

    env = mock_env(3)
    with pytest.raises(
        SampleIncompatibleVtypeError,
        match="sample contains variable assignments incompatible",
    ):
        DwaveTranslator.to_aq(sampleset, env=env)

    env = mock_env(3, vtype=Vtype.Spin)
    sol = DwaveTranslator.to_aq(sampleset, env=env)
    with pytest.raises(IndexError):
        _ = sol.samples[1]

    samples_raw = [{"x0": 0, "x1": 1, "x2": 1}]
    samples = [[v for v in sample.values()] for sample in samples_raw]
    sampleset = SampleSet.from_samples(as_samples(samples_raw), "BINARY", energy)
    with does_not_raise():
        DwaveTranslator.to_aq(sampleset, env=env)

    samples_raw = [{"x0": -10, "x1": 10, "x2": 6.43}]
    samples = [[v for v in sample.values()] for sample in samples_raw]
    env = mock_env(3, vtype=Vtype.Integer)
    sampleset = SampleSet.from_samples(as_samples(samples_raw), "INTEGER", energy)
    with does_not_raise():
        DwaveTranslator.to_aq(sampleset, env=env)


@pytest.mark.solution_translation
def test_dwave_translator_incorrect_sample_length():
    env = Environment()
    with env:
        x = Variable("x")
        y = Variable("y")

    sampleset = SampleSet.from_samples({"a": 1, "b": 0}, "BINARY", 0)
    with pytest.raises(SampleUnexpectedVariableError):
        _ = DwaveTranslator.to_aq(sampleset, env=env)


