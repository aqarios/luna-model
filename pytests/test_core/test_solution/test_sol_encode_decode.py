from random import Random

import numpy as np
import pytest
from dimod import SampleSet, Vartype, as_samples
from dwave.samplers import SimulatedAnnealingSampler

from aqmodels import (
    Environment,
    SampleSetTranslator,
    Solution,
    Timer,
    Variable,
    Vtype,
    SolutionCreationError,
)
from pytests.test_core.utils import make_seed, random_int, generate_bqms


# def mock_env(n_variables: int, vtype: Vtype = Vtype.Binary) -> Environment:
#     env = Environment()
#     with env:
#         for i in range(n_variables):
#             _ = Variable(str(f"x{i}"), vtype=vtype)
#     return env


@pytest.mark.solution_translation
def test_sampleset_translator_constructed():
    sol = Solution(
        [1, 2, 3],
        [Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real, Vtype.Binary],
        [[1, 0, 1], [0, 0, 0]],
        [[+1, -1, +1]],
        [[2, 3, -4]],
        [[2.2, 3.3, -4.4]],
    )
    print(sol)
    blob = sol.encode()
    print(blob)
    print(Solution.decode(blob))
    # samples = [[0, 1, 1], [0, 0, 1], [0, 1, 0]]
    # num_occurrences = [1, 2, 3]
    # energy = [-1, 0, 1]
    # sampleset = SampleSet.from_samples(
    #     as_samples(np.array(samples)),
    #     "BINARY",
    #     energy,
    #     num_occurrences=np.array(num_occurrences),
    # )

    # with mock_env(3):
    #     sol = SampleSetTranslator.from_dimod_sample_set(sampleset)

    # print(sol)
    # blob = sol.encode()
    # print(Solution.decode(blob))

    # assert sol.samples.tolist() == samples
    # assert sol.num_occurrences.tolist() == num_occurrences
    # assert sol.obj_values.tolist() == [None, None, None]
    # assert sol.raw_energies.tolist() == energy
    # assert sol.runtime is None

    # for result in sol.results:
    #     assert result.constraints is None
    #     assert result.feasible is None

    # results = list(sol.results)
    # assert len(results) == 3


# @pytest.mark.solution_translation
# def test_sampleset_translator_sa_random_models():
#     rand = Random(make_seed())
#     bqms = generate_bqms(20, rand)
#     for bqm in bqms:
#         timer = Timer.start()
#         sampler = SimulatedAnnealingSampler()
#         sampleset: SampleSet = sampler.sample(bqm, num_reads=128, seed=random_int(rand))
#         timing = timer.stop()
#         vtype = Vtype.Binary if bqm.vartype == Vartype.BINARY else Vtype.Spin
#         env = mock_env(bqm.num_variables, vtype=vtype)
#         sol = SampleSetTranslator.from_dimod_sample_set(sampleset, timing, env)
# 
#         sampleset_agg = sampleset.aggregate()
#         assert len(sol.samples) == len(sampleset_agg.record.sample)
#         assert sol.samples.tolist() == sampleset_agg.record.sample.tolist()
#         assert (
#                 sol.num_occurrences.tolist()
#                 == sampleset_agg.record.num_occurrences.tolist()
#         )
#         assert len(sol.num_occurrences) == len(sol.samples)
#         assert sol.runtime.total.total_seconds() > 0
#         assert sol.runtime.total_seconds > 0
#         assert sol.runtime.qpu is None
#         assert sol.obj_values.tolist() == [None] * len(sol.samples)
#         assert sol.raw_energies.tolist() == sampleset_agg.record.energy.tolist()
# 
#         results = list(sol.results)
#         assert len(results) == len(sol.samples)
#         for i, result in enumerate(results):
#             assert result.num_occurrences == sol.num_occurrences.tolist()[i]
#             assert list(result.sample) == list(sol.samples[i])
#             assert result.obj_value is None
#             assert result.raw_energy == sol.raw_energies.tolist()[i]
#             assert result.constraints is None
#             assert result.feasible is None
# 
# 
# @pytest.mark.solution_translation
# def test_sampleset_translator_error_handling():
#     samples = [[-1, 1, 1]]
#     energy = [-5]
#     sampleset = SampleSet.from_samples(as_samples(np.array(samples)), "SPIN", energy)
# 
#     env = mock_env(3)
#     with pytest.raises(SolutionCreationError):
#         _ = SampleSetTranslator.from_dimod_sample_set(sampleset, env=env)
# 
#     env = mock_env(3, vtype=Vtype.Spin)
#     sol = SampleSetTranslator.from_dimod_sample_set(sampleset, env=env)
#     with pytest.raises(IndexError):
#         _ = sol.samples[1]
