import sys
import warnings
from contextlib import nullcontext
from random import Random

import numpy as np
import pytest
from docplex.mp.model import Model as CPXModel
from scipy.optimize import minimize

from luna_model import Model, Sense, Solution, Timer, Variable, Vtype
from luna_model.translator import IbmTranslator
from pytests.test_core.utils import make_seed, random_bool

NOT_RUN_QAER = False
try:
    from qiskit_aer import AerSimulator
    from qiskit_ibm_runtime import EstimatorV2, SamplerV2, Session
    from qiskit_optimization import QuadraticProgram
    from qiskit_optimization.translators import from_docplex_mp
    from qiskit.providers import BackendV2
    from qiskit.quantum_info import SparsePauliOp
    from qiskit import QuantumCircuit, generate_preset_pass_manager
    from qiskit.circuit.library import QAOAAnsatz
    from qiskit.primitives import (
        BitArray,
        PrimitiveResult,
        PubResult,
        StatevectorEstimator,
        StatevectorSampler,
    )
except ImportError as _:
    print(
        "qiskit_aer is not installed and thus, the Gurobi tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_QAER = True

if NOT_RUN_QAER:
    ...
else:
    Backend = BackendV2 | AerSimulator | None  # type: ignore
    Sampler = SamplerV2 | StatevectorSampler  # type: ignore
    Estimator = EstimatorV2 | StatevectorEstimator  # type: ignore


def rand_float_pos_or_neg(rand: Random) -> float:
    val = rand.random()
    return val if random_bool(rand) else -val


def controlled_qp() -> QuadraticProgram:
    qp = CPXModel("a_qp")
    x = qp.binary_var(name="x")
    y = qp.binary_var(name="y")
    qp.minimize(1 * x + 2 * y + x * y - 3)
    qp = from_docplex_mp(qp)
    return qp


def controlled_aqm() -> Model:
    model = Model("a_aqm")
    with model.environment:
        x = Variable("x", vtype=Vtype.Binary)
        y = Variable("y", vtype=Vtype.Binary)
    model.set_sense(Sense.Min)
    model.objective = 1 * x + 2 * y + x * y - 3
    return model


def get_backend() -> tuple[Backend, Sampler, Estimator, Session | nullcontext]:
    backend = AerSimulator()
    sampler = SamplerV2(backend)
    estimator = EstimatorV2(backend)
    session = nullcontext()
    return backend, sampler, estimator, session


def cost_function(
    params: np.ndarray,
    ansatz: QuantumCircuit,
    hamiltonian: SparsePauliOp,
    estimator: Estimator,
):
    pub = ansatz, [hamiltonian], [params]
    result = estimator.run(pubs=[pub]).result()  # type: ignore
    cost = result[0].data.evs[0]  # type: ignore
    return cost


def solve_ansatz(
    ansatz: QuantumCircuit, op: SparsePauliOp
) -> PrimitiveResult[PubResult]:
    backend, sampler, estimator, session = get_backend()
    pass_manager = generate_preset_pass_manager(backend=backend)
    try:
        isa_ansatz = pass_manager.run(ansatz)
    except KeyError:
        msg = (
            "The input optimization problem is too large to be solved with this "
            "backend.",
        )
        raise RuntimeError(msg)
    x0 = 2 * np.pi * np.random.rand(isa_ansatz.num_parameters)
    hamiltonian = op.apply_layout(isa_ansatz.layout)

    with session:
        res = minimize(
            cost_function,
            x0,
            args=(isa_ansatz, hamiltonian, estimator),
            method="BFGS",
            options={"maxiter": 10},
        )
        qc = ansatz.assign_parameters(res.x)
        qc.measure_all()
        qc_isa = pass_manager.run(qc)
        result = sampler.run([qc_isa]).result()

    assert result is not None
    return result  # type: ignore


def compute_result(qp: QuadraticProgram) -> PrimitiveResult[PubResult]:
    op, _ = qp.to_ising()
    ansatz = QAOAAnsatz(cost_operator=op, flatten=True)
    res = solve_ansatz(ansatz, op)
    return res


def extract(result, qp):
    meas: BitArray = result[0].data.meas
    counts: dict[str, int] = meas.get_counts()

    samples = []
    energies = []
    out_counts = []

    for bitstring, count in counts.items():
        sample = [int(b) for b in bitstring]
        sample = sample[::-1]
        samples.append(sample)
        energies.append(float(qp.objective.evaluate(sample)))
        out_counts.append(count)

    return samples, energies, out_counts


@pytest.mark.skipif(NOT_RUN_QAER, reason="Qiskit Aer is required for test")
@pytest.mark.solution_translation
def test_ibm_solution_translator():
    warnings.filterwarnings("ignore")
    seed = make_seed()
    np.random.seed(seed)
    _ = Random(seed)

    aqm = controlled_aqm()

    timer = Timer.start()
    qp = controlled_qp()
    res = compute_result(qp)
    timing = timer.stop()
    sol: Solution = IbmTranslator.to_aq(res, qp, timing, env=aqm.environment)

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
