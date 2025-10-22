import sys
import pytest
import dimod
import numpy as np


from luna_model import Solution, Vtype
from luna_model.translator import BqmTranslator, DwaveTranslator

from ..utils import make_seed

NOT_RUN_DWAVE = False
try:
    from dwave.samplers import TabuSampler
except ImportError as _:
    print(
        "Dwave is not installed and thus, the CPLEX tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_DWAVE = True


@pytest.mark.skipif(NOT_RUN_DWAVE, reason="Dwave is required for test")
def test_bqm_solution():
    seed = make_seed()
    cqm = dimod.generators.random_bin_packing(num_items=5, seed=seed)  # type: ignore
    bqm, _ = dimod.cqm_to_bqm(cqm)

    # luna_model flow
    aqm = BqmTranslator.to_aq(bqm)
    bqm2 = BqmTranslator.from_aq(aqm)
    assert bqm2.is_almost_equal(bqm), "the bqms are not equal"

    bqm_vars = bqm2.variables
    aqm_vars = [v for v in aqm.variables()]
    aqm_var_names = [v.name for v in aqm_vars]
    assert bqm_vars == aqm_var_names, "the variables names are not ordered equally"

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

    assert np.allclose(bqm_qubo, aqm_qubo), "the qubos are not equal"

    res = TabuSampler().sample(bqm, seed=seed)  # type: ignore[reportPossiblyUnboundVariable] # this is save. I have a SKIP-IF.

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

    quest = sol_from_dict.samples[0].to_dict()
    for k, v in sol.samples[0].to_dict().items():
        assert quest[k] == v, "incorrect assignment in solution built from dict"

    assert sol.variable_names == aqm_var_names, (
        "(sol) the variable names don't have the expected format or ordering"
    )
    assert sol.variable_names == bqm_vars, (
        "(sol) the variable names don't match the BQM variable names"
    )

    aqm_np = np.array(sol.samples.tolist()[0])
    assert np.allclose(dimod_np, aqm_np), (
        "the numpy samples representation does not match"
    )

    sol_best = sol.best()
    assert sol_best is not None, "the best energy is not None"
    assert sol_best.obj_value == bqm.energy(res), "the objective values are not correct"
    sol_dict_best = sol_from_dict.best()
    assert sol_dict_best is not None, "the best solution is falsly set"
    assert sol_dict_best.obj_value == bqm.energy(res), (
        "the objective values do not match"
    )


@pytest.mark.skipif(NOT_RUN_DWAVE, reason="Dwave is required for test")
def test_bqm_solution_with_substitution():
    cqm = dimod.generators.random_bin_packing(num_items=2, seed=102)  # type: ignore
    bqm, _ = dimod.cqm_to_bqm(cqm)

    # luna_model flow
    aqm = BqmTranslator.to_aq(bqm)
    # print(aqm)

    rep = aqm.add_variable("s", vtype=Vtype.Spin)
    target = aqm.variables()[0]
    target_name = target.name
    target_vtype = target.vtype
    target_lower = target.bounds.lower
    target_upper = target.bounds.upper

    aqm.substitute(target, rep)
    # And now back to the original one to have a valid model and solution.
    if target_vtype not in [Vtype.Binary, Vtype.Spin]:
        back_target = aqm.add_variable(
            target_name, vtype=target_vtype, lower=target_lower, upper=target_upper
        )
    else:
        back_target = aqm.add_variable(target_name, vtype=target_vtype)

    aqm.substitute(rep, back_target)

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

    res = TabuSampler().sample(bqm, seed=42)  # type: ignore[reportPossiblyUnboundVariable] # this is safe I have a SKIP-IF.

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
