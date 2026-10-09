# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# type: ignore[reportPossiblyUnboundVariable]
from luna_model.errors import ModelNotUnconstrainedError, ModelSenseNotMinimizeError, ModelVtypeError
from luna_model.model.model import Model
from luna_model.model.sense import Sense
from luna_model.variable.vtype import Vtype

_QISKIT_AVAILABLE: bool = False
try:
    from qiskit.quantum_info import SparseObservable

    _QISKIT_AVAILABLE = True
except ImportError:
    _QISKIT_AVAILABLE = False


class SparseObservableTranslator:
    r"""Translator for Qiskit SparseObservable format.

    Converts an unconstrained spin model into a Qiskit ``SparseObservable``.
    Each spin variable :math:`s_i \in \{-1, +1\}` is mapped to the Pauli
    operator :math:`Z_i` acting on qubit :math:`i`, where :math:`i` is the
    position of the variable in ``model.environment.variables()``. This matches
    the qubit ordering expected by ``Solution.from_counts``. Products of spins are
    mapped to tensor products of :math:`Z` operators and the constant offset
    to the identity.

    Requires the ``qiskit`` extra.

    Examples
    --------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.translator import SparseObservableTranslator
    >>> model = Model()
    >>> s0 = model.add_variable("s0", vtype=Vtype.SPIN)
    >>> s1 = model.add_variable("s1", vtype=Vtype.SPIN)
    >>> model.objective = s0 * s1 - 2 * s0 + 0.5
    >>> obs = SparseObservableTranslator.from_lm(model)

    Notes
    -----
    The model must be unconstrained, minimizing, and contain only spin
    variables. Higher-order terms are supported.
    """

    @staticmethod
    def from_lm(model: Model) -> "SparseObservable":
        """Convert LunaModel to Qiskit SparseObservable.

        Parameters
        ----------
        model : Model
            The model to convert. Must be:
            - Unconstrained (no constraints)
            - Minimizing
            - Spin variables only

        Returns
        -------
        SparseObservable
            Observable with one qubit per environment variable whose expectation
            value for a computational basis state equals the objective value of
            the corresponding spin assignment (``|0>`` ↦ ``+1``, ``|1>`` ↦ ``-1``).
            Qubit ``i`` corresponds to ``model.environment.variables()[i]``.

        Raises
        ------
        RuntimeError
            If ``qiskit`` package is not installed.
        ModelNotUnconstrainedError
            If the model has constraints.
        ModelSenseNotMinimizeError
            If the model's sense is not minimize.
        ModelVtypeError
            If the model's environment contains non-spin variables.

        Examples
        --------
        >>> from luna_model import Model, Vtype
        >>> from luna_model.translator import SparseObservableTranslator
        >>> model = Model()
        >>> s0 = model.add_variable("s0", vtype=Vtype.SPIN)
        >>> s1 = model.add_variable("s1", vtype=Vtype.SPIN)
        >>> model.objective = s0 * s1 - 2 * s0 + 0.5
        >>> obs = SparseObservableTranslator.from_lm(model)
        """
        if not _QISKIT_AVAILABLE:
            msg = "qiskit is required for the SparseObservableTranslator. You can install it using the 'qiskit' extra."
            raise RuntimeError(msg)
        if model.num_constraints > 0:
            msg = "the model is not unconstrained"
            raise ModelNotUnconstrainedError(msg)
        if model.sense != Sense.MIN:
            msg = "the model's sense is not Minimize"
            raise ModelSenseNotMinimizeError(msg)

        variables = model.environment.variables()
        for var in variables:
            if var.vtype != Vtype.SPIN:
                msg = f"variable '{var.name}' has vtype {var.vtype}, but only spin variables are supported"
                raise ModelVtypeError(msg)
        qubits = {var.name: i for i, var in enumerate(variables)}

        objective = model.objective
        terms: list[tuple[str, list[int], float]] = [("", [], objective.get_offset())]
        terms.extend(("Z", [qubits[v.name]], coeff) for v, coeff in objective.linear_items())
        terms.extend(("ZZ", [qubits[u.name], qubits[v.name]], coeff) for u, v, coeff in objective.quadratic_items())
        terms.extend(
            ("Z" * len(vs), [qubits[v.name] for v in vs], coeff) for vs, coeff in objective.higher_order_items()
        )

        return SparseObservable.from_sparse_list(terms, num_qubits=len(variables))
