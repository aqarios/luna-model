import sys
from contextlib import nullcontext as does_not_raise
from random import Random

import numpy as np
import pytest
from dimod import SampleSet, Vartype, as_samples
from .fixtures import DwaveResult, mock_env, dwave_result

NOT_RUN_DWAVE = False
try:
    from dwave.samplers import SimulatedAnnealingSampler
except ImportError as _:
    print(
        "Dwave is not installed and thus, the CPLEX tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_DWAVE = True

from luna_model import Environment, Timer, Variable, Vtype
from luna_model.errors import (
    SampleIncompatibleVtypeError,
    SampleUnexpectedVariableError,
)
from luna_model.translator import DwaveTranslator

from _tests.test_core.utils import generate_bqms, make_seed, random_int


def test_sampleset_translator_constructed(dwave_result: DwaveResult):
    with mock_env(3):
        sol = DwaveTranslator.to_aq(dwave_result.sampleset)

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


@pytest.mark.skipif(NOT_RUN_DWAVE, reason="Dwave is required for test")
def test_sampleset_translator_sa_random_models():
    rand = Random(make_seed())
    bqms = generate_bqms(20, rand)
    for bqm in bqms:
        timer = Timer.start()
        sampler = SimulatedAnnealingSampler()
        sampleset: SampleSet = sampler.sample(bqm, num_reads=128, seed=random_int(rand))
        timing = timer.stop()
        vtype = Vtype.BINARY if bqm.vartype == Vartype.BINARY else Vtype.SPIN
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
        assert sol.counts.tolist() == sampleset_agg.record.num_occurrences.tolist()
        assert len(sol.counts) == len(sol.samples)
        assert sol.runtime is not None
        assert sol.runtime.total.total_seconds() > 0
        assert sol.runtime.total_seconds > 0
        assert sol.runtime.qpu is None
        assert sol.obj_values is None
        assert sol.raw_energies is not None
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


def test_sampleset_translator_error_handling():
    samples_raw = [{"x0": -1, "x1": 1, "x2": 1}]
    energy = [-5]
    sampleset = SampleSet.from_samples(as_samples(samples_raw), "SPIN", energy)

    env = mock_env(3)
    with pytest.raises(
        SampleIncompatibleVtypeError,
        match="sample contains variable assignments incompatible",
    ):
        DwaveTranslator.to_aq(sampleset, env=env)

    env = mock_env(3, vtype=Vtype.SPIN)
    sol = DwaveTranslator.to_aq(sampleset, env=env)
    with pytest.raises(IndexError):
        _ = sol.samples[1]

    samples_raw = [{"x0": 0, "x1": 1, "x2": 1}]
    sampleset = SampleSet.from_samples(as_samples(samples_raw), "BINARY", energy)
    with does_not_raise():
        DwaveTranslator.to_aq(sampleset, env=env)

    samples_raw = [{"x0": -10, "x1": 10, "x2": 6.43}]
    env = mock_env(3, vtype=Vtype.INTEGER)
    sampleset = SampleSet.from_samples(as_samples(samples_raw), "INTEGER", energy)
    with does_not_raise():
        DwaveTranslator.to_aq(sampleset, env=env)


def test_dwave_translator_incorrect_sample_length():
    env = Environment()
    with env:
        _ = Variable("x")
        _ = Variable("y")

    sampleset = SampleSet.from_samples({"a": 1, "b": 0}, "BINARY", 0)
    with pytest.raises(SampleUnexpectedVariableError):
        _ = DwaveTranslator.to_aq(sampleset, env=env)
