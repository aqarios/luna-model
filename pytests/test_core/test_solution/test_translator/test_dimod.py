import dimod
import numpy as np
import pytest
from dimod import SampleSet, BinaryQuadraticModel, Vartype, as_samples
from dwave.samplers import SimulatedAnnealingSampler

from aq_models import SampleSetTranslator, Timer


def generate_bqm():
    return dimod.generators.gnm_random_bqm([f"x{i}" for i in range(30)], 100, "BINARY")


@pytest.mark.solution_translation
def test_from_dimod():
    samples = [[0, 1, 1], [0, 0, 1], [0, 1, 0]]
    num_occurrences = [1, 2, 3]
    energy = [-1, 0, 1]
    sampleset = SampleSet.from_samples(
        as_samples(np.array(samples)),
        "BINARY",
        energy,
        num_occurrences=np.array(num_occurrences),
    )
    sol = SampleSetTranslator.from_dimod_sample_set(sampleset)

    assert sol.samples.tolist() == samples
    assert sol.num_occurrences.tolist() == num_occurrences
    results = list(sol.results)
    assert len(results) == 3
    # assert sol.obj_values.tolist() == energy


def test_from_dimod_2():
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
    print(timing.start)
    print(timing.end)
    print(timing.qpu)
    print(timing.total)
    print(timing.total_seconds)
    print(timing.total.total_seconds())

    sol = SampleSetTranslator.from_dimod_sample_set(sampleset, timing)
    assert sol.samples.tolist() == [[0.0, 1.0, 0.0]]
    assert sol.num_occurrences.tolist() == [5]
    assert sol.runtime.total.total_seconds() > 0
    assert sol.runtime.total_seconds > 0
    assert sol.runtime.qpu is None

    results = list(sol.results)
    assert len(results) == 1
    assert results[0].sample.tolist() == [0.0, 1.0, 0.0]

    # bqm2 = generate_bqm()
    # print(bqm2.to_qubo())
    raise Exception

# def test_iterableeeeess():
#     bqm = BinaryQuadraticModel(vartype=Vartype.BINARY)
#     bqm.add_variable("x1")
#     bqm.add_variable("x2")
#     bqm.add_variable("x3")
#     bqm.add_linear("x1", 1)
#     bqm.add_linear("x2", -2)
#     bqm.add_linear("x3", -1)
#     bqm.add_quadratic("x1", "x2", 5)
#     bqm.add_quadratic("x1", "x3", -1)
#     bqm.add_quadratic("x2", "x3", 2)
#
#     sampler = SimulatedAnnealingSampler()
#     sampleset: SampleSet = sampler.sample(bqm, num_reads=5, seed=42)
#     runtime = Runtime(total=10.2)
#
#     sol = SampleSetTranslator.from_dimod_sample_set(sampleset, runtime)
#     results = sol.results
#
#     # result0 = next(results)
#     # result1 = next(results)
#     #
#     # model.evaluate(sol)
#     #
#     # result2 = next(results)
#
#     results_list = list(sol.results)
#     print(results_list[0])
#
#     for res in results_list:
#         model.evaluate(res)
#
#     print(results_list[0])
