import itertools

import numpy as np
import pytest
from qiskit.quantum_info import SparseObservable

from luna_model import Model, Sense, TranslationTarget, Vtype
from luna_model.errors import (
    ModelNotUnconstrainedError,
    ModelSenseNotMinimizeError,
    ModelVtypeError,
    TranslationError,
)
from luna_model.translator import SparseObservableTranslator


@pytest.fixture()
def model() -> Model:
    model = Model("spin_model")
    s0 = model.add_variable("s0", vtype=Vtype.SPIN)
    s1 = model.add_variable("s1", vtype=Vtype.SPIN)
    s2 = model.add_variable("s2", vtype=Vtype.SPIN)
    s3 = model.add_variable("s3", vtype=Vtype.SPIN)
    model.objective = 1.5 + s0 - 2 * s1 + 0.5 * s3 + 3 * s0 * s1 - s1 * s2 + 0.25 * s0 * s2 * s3
    return model


def _diagonal(obs: SparseObservable) -> np.ndarray:
    dense = obs.to_sparse_list()
    n = obs.num_qubits
    diag = np.zeros(2**n)
    for basis in range(2**n):
        for paulis, indices, coeff in dense:
            assert set(paulis) <= {"Z"}
            sign = (-1) ** sum((basis >> i) & 1 for i in indices)
            diag[basis] += np.real(coeff) * sign
    return diag


def test_from_lm_terms(model: Model) -> None:
    obs = SparseObservableTranslator.from_lm(model)

    expected = SparseObservable.from_sparse_list(
        [
            ("", [], 1.5),
            ("Z", [0], 1.0),
            ("Z", [1], -2.0),
            ("Z", [3], 0.5),
            ("ZZ", [0, 1], 3.0),
            ("ZZ", [1, 2], -1.0),
            ("ZZZ", [0, 2, 3], 0.25),
        ],
        num_qubits=4,
    )
    assert obs.num_qubits == 4
    assert (obs - expected).simplify().num_terms == 0


def test_from_lm_matches_objective(model: Model) -> None:
    obs = SparseObservableTranslator.from_lm(model)
    diag = _diagonal(obs)
    variables = model.environment.variables()

    # Qubit i corresponds to variables[i], with |0> -> +1 and |1> -> -1.
    for basis in range(2 ** len(variables)):
        spins = [1 - 2 * ((basis >> i) & 1) for i in range(len(variables))]
        value = model.objective.get_offset()
        idx = {v.name: i for i, v in enumerate(variables)}
        value += sum(c * spins[idx[v.name]] for v, c in model.objective.linear_items())
        value += sum(c * spins[idx[u.name]] * spins[idx[v.name]] for u, v, c in model.objective.quadratic_items())
        value += sum(c * np.prod([spins[idx[v.name]] for v in vs]) for vs, c in model.objective.higher_order_items())
        assert np.isclose(diag[basis], value)


def test_from_lm_unused_variable_and_constant() -> None:
    model = Model()
    s0 = model.add_variable("s0", vtype=Vtype.SPIN)
    model.add_variable("s1", vtype=Vtype.SPIN)
    model.objective = 2 * s0 + 4.0

    obs = SparseObservableTranslator.from_lm(model)
    assert obs.num_qubits == 2
    assert sorted(obs.to_sparse_list(), key=lambda t: len(t[1])) == [("", [], 4.0), ("Z", [0], 2.0)]


def test_from_lm_empty_objective() -> None:
    model = Model()
    model.add_variable("s0", vtype=Vtype.SPIN)
    obs = SparseObservableTranslator.from_lm(model)
    assert obs.num_qubits == 1
    assert obs.to_sparse_list() == [("", [], 0.0)]


def test_from_lm_spin_powers_reduced() -> None:
    model = Model()
    a = model.add_variable("a", vtype=Vtype.SPIN)
    b = model.add_variable("b", vtype=Vtype.SPIN)
    model.objective = a * a * b + 2 * a * a + a * b * b * b

    obs = SparseObservableTranslator.from_lm(model)
    expected = SparseObservable.from_sparse_list([("", [], 2.0), ("Z", [1], 1.0), ("ZZ", [0, 1], 1.0)], num_qubits=2)
    assert (obs - expected).simplify().num_terms == 0


def test_from_lm_binary_vars(model: Model) -> None:
    x = model.add_variable("x", vtype=Vtype.BINARY)
    model.objective += x
    with pytest.raises(ModelVtypeError):
        _ = SparseObservableTranslator.from_lm(model)


def test_from_lm_constrained(model: Model) -> None:
    s0, s1 = model.variables()[:2]
    model.constraints += s0 + s1 <= 0
    with pytest.raises(ModelNotUnconstrainedError):
        _ = SparseObservableTranslator.from_lm(model)
    with pytest.raises(TranslationError):
        _ = SparseObservableTranslator.from_lm(model)


def test_from_lm_wrong_sense(model: Model) -> None:
    model.set_sense(Sense.MAX)
    with pytest.raises(ModelSenseNotMinimizeError):
        _ = SparseObservableTranslator.from_lm(model)


@pytest.mark.parametrize("n", [1, 3, 5])
def test_from_lm_dense_quadratic(n: int) -> None:
    rng = np.random.default_rng(n)
    model = Model()
    spins = [model.add_variable(f"s{i}", vtype=Vtype.SPIN) for i in range(n)]
    h = rng.normal(size=n)
    j = rng.normal(size=(n, n))
    objective = 0.0 + sum(float(h[i]) * spins[i] for i in range(n))
    for a, b in itertools.combinations(range(n), 2):
        objective += float(j[a, b]) * spins[a] * spins[b]
    model.objective = objective

    diag = _diagonal(SparseObservableTranslator.from_lm(model))
    for basis in range(2**n):
        s = np.array([1 - 2 * ((basis >> i) & 1) for i in range(n)])
        value = h @ s + sum(j[a, b] * s[a] * s[b] for a, b in itertools.combinations(range(n), 2))
        assert np.isclose(diag[basis], value)


def test_model_to_sparse_observable(model: Model) -> None:
    obs = model.to(TranslationTarget.SPARSE_OBSERVABLE)
    assert isinstance(obs, SparseObservable)
    assert (obs - SparseObservableTranslator.from_lm(model)).simplify().num_terms == 0
