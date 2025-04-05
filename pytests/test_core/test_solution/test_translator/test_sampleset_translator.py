from random import Random

import dimod
import numpy as np
import pytest
from dimod import BinaryQuadraticModel, SampleSet, Vartype, as_samples
from dwave.samplers import SimulatedAnnealingSampler

from aqmodels import Environment, SampleSetTranslator, Timer, Variable, Vtype
from pytests.test_core.utils import make_seed, random_int


def generate_bqms(
    n_models: int, rand: Random, n_vars_max: int = 100
) -> list[BinaryQuadraticModel]:
    out = []
    for _ in range(n_models):
        n_vars = rand.randint(1, n_vars_max)
        density = rand.random() * (1 - 1 / n_vars)
        num_interactions = int(density * n_vars**2 / 2)
        vartype = Vartype.BINARY if rand.randint(0, 1) == 0 else Vartype.SPIN
        bqm = dimod.generators.gnm_random_bqm(
            [f"x{i}" for i in range(n_vars)],
            num_interactions,
            vartype,
            random_state=random_int(rand),
        )
        out.append(bqm)
    return out


def mock_env(n_variables: int, vartype: Vartype = Vartype.BINARY) -> Environment:
    env = Environment()
    vtype = Vtype.Binary if vartype == Vartype.BINARY else Vtype.Spin
    with env:
        for i in range(n_variables):
            _ = Variable(str(f"x{i}"), vtype=vtype)
    return env


@pytest.mark.solution_translation
def test_from_dimod_constructed():
    samples = [[0, 1, 1], [0, 0, 1], [0, 1, 0]]
    num_occurrences = [1, 2, 3]
    energy = [-1, 0, 1]
    sampleset = SampleSet.from_samples(
        as_samples(np.array(samples)),
        "BINARY",
        energy,
        num_occurrences=np.array(num_occurrences),
    )

    with Environment():
        for v in range(3):
            _ = Variable(str(v))
        sol = SampleSetTranslator.from_dimod_sample_set(sampleset)

    assert sol.samples.tolist() == samples
    assert sol.num_occurrences.tolist() == num_occurrences
    results = list(sol.results)
    assert len(results) == 3
    assert sol.obj_values.tolist() == [None, None, None]
    assert sol.raw_energies.tolist() == energy


def test_from_dimod_sa():
    bqm = BinaryQuadraticModel(vartype=Vartype.BINARY)
    bqm.add_variable("x1")
    bqm.add_variable("x2")
    bqm.add_variable("x3")
    bqm.add_linear("x1", 1)
    bqm.add_linear("x2", -2)
    bqm.add_linear("x3", -1)
    bqm.add_quadratic("x1", "x2", 5)
    bqm.add_quadratic("x1", "x3", -1)
    bqm.add_quadratic("x2", "x3", 2)

    timer = Timer.start()
    sampler = SimulatedAnnealingSampler()
    sampleset: SampleSet = sampler.sample(bqm, num_reads=5, seed=42)
    timing = timer.stop()
    assert timing.start.timestamp() < timing.end.timestamp()
    assert timing.qpu is None
    assert timing.total.total_seconds() > 0
    assert timing.total_seconds > 0

    env = mock_env(bqm.num_variables)
    sol = SampleSetTranslator.from_dimod_sample_set(sampleset, timing, env)
    assert sol.samples.tolist() == [[0.0, 1.0, 0.0]]
    assert sol.num_occurrences.tolist() == [5]
    assert sol.runtime.total.total_seconds() > 0
    assert sol.runtime.total_seconds > 0
    assert sol.runtime.qpu is None
    assert sol.obj_values.tolist() == [None]
    assert sol.raw_energies.tolist() == [-2.0]

    results = list(sol.results)
    assert len(results) == 1
    sample = results[0].sample
    assert list(sample) == [0.0, 1.0, 0.0]


def test_from_dimod_sa_random_models():
    rand = Random(make_seed())
    bqms = generate_bqms(20, rand)
    for bqm in bqms:
        timer = Timer.start()
        sampler = SimulatedAnnealingSampler()
        sampleset: SampleSet = sampler.sample(bqm, num_reads=128, seed=random_int(rand))
        timing = timer.stop()
        env = mock_env(bqm.num_variables, vartype=bqm.vartype)
        sol = SampleSetTranslator.from_dimod_sample_set(sampleset, timing, env)

        assert len(sol.samples) > 0
        assert len(sol.num_occurrences) == len(sol.samples)
        assert sol.runtime.total.total_seconds() > 0
        assert sol.runtime.total_seconds > 0
        assert sol.runtime.qpu is None
        assert sol.obj_values.tolist() == [None] * len(sol.samples)
        assert len(sol.raw_energies) > 0
