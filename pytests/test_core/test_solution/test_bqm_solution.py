import dimod
import numpy as np
from aqmodels._core import Solution
from aqmodels.translator import BqmTranslator
from aqmodels.translator import DwaveTranslator
from dwave.samplers import TabuSampler


def test_bqm_solution():
    cqm = dimod.generators.random_bin_packing(num_items=5, seed=102)  # type: ignore
    bqm, _ = dimod.cqm_to_bqm(cqm)

    # aqmodels flow
    aqm = BqmTranslator.to_aq(bqm)
    bqm2 = BqmTranslator.from_aq(aqm)
    assert bqm2.is_almost_equal(bqm)

    bqm_vars = bqm2.variables
    aqm_vars = [v for v in aqm.variables()]
    aqm_var_names = [v.name for v in aqm_vars]
    assert bqm_vars == aqm_var_names

    bqm_qubo = np.zeros((len(bqm_vars), len(bqm_vars)))
    for i, u in enumerate(bqm_vars):
        bqm_qubo[i, i] = bqm2.linear[u]
        for j, v in enumerate(bqm_vars):
            if i == j:
                continue
            bqm_qubo[i, j] = bqm2.quadratic.get((u, v), 0)
            bqm_qubo[j, i] = bqm2.quadratic.get((v, u), 0)

    aqm_qubo = np.zeros((len(aqm_vars), len(aqm_vars)))
    for i, u in enumerate(aqm_vars):
        aqm_qubo[i, i] = aqm.objective.get_linear(u)
        for j, v in enumerate(aqm_vars):
            if i == j:
                continue
            aqm_qubo[i, j] = aqm.objective.get_quadratic(u, v)
            aqm_qubo[j, i] = aqm.objective.get_quadratic(v, u)

    assert np.allclose(bqm_qubo, aqm_qubo)

    res = TabuSampler().sample(bqm, seed=42)

    ordering = aqm_var_names.copy()
    dimod_positions = {v: i for i, v in enumerate(bqm_vars)}

    dimod_np = np.zeros(len(ordering))
    dimod_sample = res.samples()[0]
    for v, pos in dimod_positions.items():
        dimod_np[pos] = dimod_sample[v]  # type: ignore

    with aqm.environment:
        sol = DwaveTranslator.to_aq(res)

    dimod_sample = res.samples()[0]
    with aqm.environment:
        sol_from_dict = Solution.from_dict(
            {str(v): float(val) for v, val in dimod_sample.items()}  # type: ignore
        )

    sol = aqm.evaluate(sol)
    sol_from_dict = aqm.evaluate(sol_from_dict)

    assert sol.variable_names == aqm_var_names
    assert sol.variable_names == bqm_vars

    aqm_np = np.array(sol.samples.tolist()[0])
    assert np.allclose(dimod_np, aqm_np)

    sol_best = sol.best()
    assert sol_best is not None
    assert bqm.energy(res) == sol_best.obj_value
    sol_dict_best = sol_from_dict.best()
    assert sol_dict_best is not None
    assert bqm.energy(res) == sol_dict_best.obj_value
