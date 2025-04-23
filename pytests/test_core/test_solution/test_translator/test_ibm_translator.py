import enum
import pytest
import numpy as np

from docplex.mp.model import Model as CPXModel
from contextlib import nullcontext
from qiskit import QuantumCircuit, generate_preset_pass_manager
from qiskit.circuit.library import QAOAAnsatz
from qiskit.primitives import (
    BitArray,
    PrimitiveResult,
    PubResult,
    StatevectorEstimator,
    StatevectorSampler,
)
from qiskit.quantum_info import SparsePauliOp
from qiskit.providers import BackendV2
from qiskit_aer import AerSimulator
from qiskit_optimization import QiskitOptimizationError, QuadraticProgram
from qiskit_optimization.converters import QuadraticProgramToQubo
from qiskit_ibm_runtime import EstimatorV2, SamplerV2, Session
from qiskit_optimization.translators import from_docplex_mp
from scipy.optimize import minimize

from random import Random
from aqmodels import (
    IbmTranslator,
    Timer,
    Variable,
    Vtype,
    Bounds,
    Sense,
    Model,
)
from pytests.test_core.utils import make_seed, random, random_bool, random_int, todo

Backend = BackendV2 | AerSimulator | None
Sampler = SamplerV2 | StatevectorSampler
Estimator = EstimatorV2 | StatevectorEstimator


def rand_float_pos_or_neg(rand: Random) -> float:
    val = rand.random()
    return val if random_bool(rand) else -val


def controlled_qp() -> QuadraticProgram:
    qp = CPXModel("a_qp")
    x = qp.binary_var(name="x")
    y = qp.binary_var(name="y")

    qp.minimize(1 * x + 2 * y + x * y - 3)
    # qp.add_constraint(v + 2 * w + t + u <= 3, "cons1")
    # qp.add_constraint(v + w + t >= 1, "cons2")
    # qp.add_constraint(v + w == 1, "cons3")
    qp = from_docplex_mp(qp)
    return qp


def controlled_aqm() -> Model:
    model = Model("a_aqm")
    with model.environment:
        x = Variable("x", vtype=Vtype.Binary)
        y = Variable("y", vtype=Vtype.Binary)
        # v = Variable("v", vtype=Vtype.Binary)
        # w = Variable("w", vtype=Vtype.Binary)
        # t = Variable("t", vtype=Vtype.Binary)
        # u = Variable("u", vtype=Vtype.Binary)
        # t = Variable("t", vtype=Vtype.Integer, bounds=Bounds(0, 3))
        # u = Variable("u", vtype=Vtype.Integer, bounds=Bounds(0, 3))
    model.set_sense(Sense.Min)
    model.objective = 1 * x + 2 * y + x * y - 3
    # model.objective = v + w + t + 5 * (u - 2) * w
    # model.constraints += v + 2 * w + t + u <= 3, "cons1"
    # model.constraints += v + w + t >= 1, "cons2"
    # model.constraints += v + w == 1, "cons3"
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
    # try:
    #     op, _ = qp.to_ising()
    # except QiskitOptimizationError:
    #     conv = QuadraticProgramToQubo()
    #     qp_d = conv.convert(qp)
    #     op, _ = qp_d.to_ising()
    op, _ = qp.to_ising()
    ansatz = QAOAAnsatz(cost_operator=op, flatten=True)
    res = solve_ansatz(ansatz, op)
    return res


@pytest.mark.solution_translation
def test_ibm_solution_translator():
    seed = make_seed()
    np.random.seed(seed)
    _ = Random(seed)

    aqm = controlled_aqm()

    timer = Timer.start()
    qp = controlled_qp()
    res = compute_result(qp)
    timing = timer.stop()

    print(qp)
    print(res)

    # extract(res, qp, timing, aqm.environment)

    sol = IbmTranslator.from_ibm(res, qp, timing, aqm.environment)
    print(sol)


def extract(result, qp, timing, env):
    meas: BitArray = result[0].data.meas
    counts: dict[str, int] = meas.get_counts()

    samples = []
    orderings = []
    energies = []
    num_occurences = []

    for bitstring, count in counts.items():
        sample = []
        order = []
        for i, b in enumerate(bitstring):
            sample.append(int(b))
            order.append(qp.variables[i].name)

        samples.append(sample)
        orderings.append(order)
        energies.append(float(qp.objective.evaluate(sample)))
        num_occurences.append(count)

        sample = {qp.variables[i].name: int(b) for i, b in enumerate(bitstring)}

    return translator.IbmTranslator.translate(
        samples, orderings, energies, num_occurences, timing, env
    )
