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
from __future__ import annotations

from typing import TYPE_CHECKING

from luna_model._lm import PyQubo, PyQuboTranslator
from luna_model.model.model import Model
from luna_model.model.sense import Sense
from luna_model.variable.vtype import Vtype

if TYPE_CHECKING:
    from numpy.typing import NDArray


class Qubo:
    """QUBO representation of an optimization problem.

    A QUBO (Quadratic Unconstrained Binary Optimization) problem is represented
    by a matrix Q where the objective is to minimize/maximize:

        x^T Q x + offset

    where x is a binary vector and offset is a constant term.

    Parameters
    ----------
    None
        Qubo objects are created through QuboTranslator.from_lm()

    Attributes
    ----------
    name : str
        The name of the QUBO problem.
    variable_names : list[str]
        Names of binary variables in order.
    matrix : NDArray
        The Q matrix defining the QUBO.
    offset : float
        Constant offset term.
    vtype : Vtype
        Variable type (BINARY or SPIN).
    sense : Sense
        Optimization sense (MIN or MAX).

    Examples
    --------
    Create QUBO from a model:

    >>> from luna_model import Model, Variable
    >>> from luna_model.translator import QuboTranslator
    >>> model = Model()
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> model.objective = x * y - 2 * x + y
    >>> qubo = QuboTranslator.from_lm(model)
    >>> print(qubo.matrix)
    >>> print(qubo.offset)
    """

    _q: PyQubo

    @classmethod
    def _from_pyq(cls, py_q: PyQubo) -> Qubo:
        q = cls.__new__(cls)
        q._q = py_q
        return q

    @property
    def name(self) -> str:
        """Get the name of the QUBO problem.

        Returns
        -------
        str
            Problem name.
        """
        return self._q.name

    @property
    def variable_names(self) -> list[str]:
        """Get the ordered list of variable names.

        Returns
        -------
        list[str]
            Variable names corresponding to matrix indices.
        """
        return self._q.variable_names

    @property
    def matrix(self) -> NDArray:
        """Get the QUBO matrix.

        Returns
        -------
        NDArray
            Q matrix where Q[i,j] is the coefficient
            for x[i]*x[j]. Diagonal entries Q[i,i] are linear coefficients.
        """
        return self._q.matrix

    @property
    def offset(self) -> float:
        """Get the constant offset term.

        Returns
        -------
        float
            Constant offset added to the objective value.
        """
        return self._q.offset

    @property
    def vtype(self) -> Vtype:
        """Get the variable type.

        Returns
        -------
        Vtype
            Variable type (BINARY or SPIN).
        """
        return Vtype._from_pyvtype(self._q.vtype)

    @property
    def sense(self) -> Sense:
        """Get the optimization sense.

        Returns
        -------
        Sense
            MIN or MAX.
        """
        return Sense._from_pysense(self._q.sense)


class QuboTranslator:
    """Translator for QUBO format.

    QuboTranslator provides static methods to convert between LunaModel's internal
    Model representation and the QUBO (Quadratic Unconstrained Binary Optimization)
    format. QUBO is a matrix-based representation widely used in quantum computing
    and combinatorial optimization.

    QUBO format requires:
    - All variables must be binary (or spin)
    - No constraints (unconstrained)
    - At most quadratic degree (no cubic or higher terms)

    If the model doesn't meet these requirements, use transformations first.

    Examples
    --------
    Convert matrix to LunaModel:

    >>> import numpy as np
    >>> from luna_model.translator import QuboTranslator
    >>> # Define QUBO matrix (upper-triangular)
    >>> Q = np.array([[-1, 2], [0, -1]])
    >>> model = QuboTranslator.to_lm(Q, variable_names=["x", "y"])

    Convert model to QUBO:

    >>> from luna_model import Model
    >>> model = Model()
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> model.objective = x * y - 2 * x + y
    >>> qubo = QuboTranslator.from_lm(model)
    """

    @staticmethod
    def to_lm(
        qubo: NDArray,
        *,
        offset: float | None = None,
        variable_names: list[str] | None = None,
        name: str | None = None,
        vtype: Vtype | None = None,
    ) -> Model:
        """Convert QUBO matrix to LunaModel.

        Creates a LunaModel Model from a QUBO matrix representation.

        Parameters
        ----------
        qubo : NDArray
            QUBO matrix where Q[i,j] represents the coefficient for x[i]*x[j].
            Diagonal elements Q[i,i] are linear coefficients. If the matrix is
            not symmetric, it will be made symmetric by summing Q[i,j] and Q[j,i].
        offset : float, optional
            Constant offset term to add to objective. Default is 0.
        variable_names : list[str], optional
            Names for variables. If None, generates names like "x0", "x1", etc.
        name : str, optional
            Name for the model.
        vtype : Vtype, optional
            Variable type (BINARY or SPIN). Default is BINARY.

        Returns
        -------
        Model
            LunaModel with objective function representing the QUBO.

        Examples
        --------
        Basic usage:

        >>> import numpy as np
        >>> Q = np.array([[-2, 1], [0, -1]])
        >>> model = QuboTranslator.to_lm(Q)

        With custom names and offset:

        >>> model = QuboTranslator.to_lm(Q, offset=5.0, variable_names=["x1", "x2"], name="MyQUBO")

        Notes
        -----
        Non-symmetric matrices are automatically symmetrized: the coefficient for
        x[i]*x[j] becomes Q[i,j] + Q[j,i].
        """
        return Model._from_pym(
            PyQuboTranslator.to_lm(
                qubo,
                offset,
                variable_names,
                name,
                vtype=vtype._val if vtype else None,
            )
        )

    @staticmethod
    def from_lm(model: Model) -> Qubo:
        """Convert LunaModel to QUBO format.

        Converts a LunaModel Model to QUBO representation.

        Parameters
        ----------
        model : Model
            The model to convert. Must be:
            - Unconstrained (no constraints)
            - Binary or spin variables only
            - At most quadratic (degree ≤ 2)

        Returns
        -------
        Qubo
            QUBO representation with matrix, offset, and metadata.

        Raises
        ------
        TranslationError
            If the model has constraints, non-binary variables, or
            higher-order terms.

        Examples
        --------
        >>> from luna_model import Model
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> model.objective = x * y - 2 * x + y
        >>> qubo = QuboTranslator.from_lm(model)
        >>> print(qubo.matrix)

        Notes
        -----
        If your model doesn't meet the QUBO requirements, use
        transformations to convert it:
        """
        return Qubo._from_pyq(PyQuboTranslator.from_lm(model._m))
