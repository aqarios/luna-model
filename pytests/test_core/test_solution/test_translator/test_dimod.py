import numpy as np
from dimod import SampleSet, BinaryQuadraticModel, Vartype
from dwave.samplers import SimulatedAnnealingSampler

from aq_models import SampleSetTranslator


# @pytest.mark.solution_translation
# def test_from_dimod():
#     # sampleset = SampleSet.from_samples(
#     #     dimod.as_samples(
#     #         np.array([[0, 1, 1], [0, 0, 1], [0, 1, 0]], dtype=np.float64)
#     #     ),
#     #     "BINARY",
#     #     0,
#     #     num_occurrences=np.array([1, 2, 3], dtype=np.uint64),
#     # )
#     sampleset = SampleSet.from_samples(
#         dimod.as_samples(
#             np.array([[0, 1, 1], [0, 0, 1], [0, 1, 0]])
#         ),
#         "BINARY",
#         0,
#         num_occurrences=np.array([1, 2, 3]),
#     )
#     print(f"{sampleset.record.sample.dtype = }")
#     print(f"{sampleset.record.num_occurrences.dtype = }")
#     sol = Solution.from_dimod_sample_set(sampleset)
#     print(sol)
#     raise Exception


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

    sampler = SimulatedAnnealingSampler()
    sampleset: SampleSet = sampler.sample(bqm, num_reads=5, seed=42)

    sol = SampleSetTranslator.from_dimod_sample_set(sampleset)
    np.allclose(sol.samples, np.array([[0.0, 1.0, 0.0]]))
    assert np.allclose(sol.num_occurrences, np.array([5]))

    # print(sol.results[0])
    print(sol)
    print(f"{len(sol.results) = }")
    print(sol.results[0].sample)
    print(sol.results[0].num_occurrences)
    print(sol.results[0].obj_value)
    print(sol.results[0].feasible)
    print(sol.results[0].constraint_satisfaction)

    # print(sampleset.info)
    raise Exception
