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

from luna_model._lm import PyModelSpecs
from luna_model.model.ctype import Ctype
from luna_model.model.sense import Sense
from luna_model.variable import Vtype


class ModelSpecs:
    """Specifications and requirements for models.

    Defines the characteristics and limitations of an optimization model,
    including optimization sense, allowed variable types, constraint types,
    maximum polynomial degree, and size limits.

    Parameters
    ----------
    sense : Sense | None, optional
        The optimization direction (MIN or MAX).
    vtypes : set[Vtype] | None, optional
        Set of allowed variable types (BINARY, INTEGER, REAL, SPIN).
    constraints : set[Ctype] | None, optional
        Set of allowed constraint types (UNCONSTRAINED, EQUALITY, INEQUALITY, LESS_EQUAL, GREATER_EQUAL).
    max_degree : int | None, optional
        Maximum degree of objective polynomial.
    max_constraint_degree : int | None, optional
        Maximum degree of constraint polynomials.
    max_num_variables : int | None, optional
        Maximum number of variables allowed.

    Attributes
    ----------
    sense : Sense | None
        The optimization sense.
    max_degree : int | None
        Maximum objective degree.
    max_constraint_degree : int | None
        Maximum constraint degree.
    max_num_variables : int | None
        Maximum variable count.
    vtypes : list[Vtype] | None
        Allowed variable types.
    constraints : list[Ctype] | None
        Allowed constraint types.

    Examples
    --------
    Create specifications for a QUBO model:

    >>> from luna_model.model import ModelSpecs, Sense, Ctype
    >>> from luna_model.variable import Vtype
    >>> target_specs = ModelSpecs(
    ...     sense=Sense.MIN, vtypes={Vtype.BINARY}, constraints={Ctype.UNCONSTRAINED}, max_degree=2
    ... )

    Check if a model satisfies specifications:

    >>> model_specs = model.get_specs()
    >>> if model_specs.satisfies(target_specs):
    ...     print("Model meets requirements")
    """

    _sp: PyModelSpecs

    def __init__(  # noqa: PLR0913
        self,
        sense: Sense | None = None,
        vtypes: set[Vtype] | None = None,
        constraints: set[Ctype] | None = None,
        max_degree: int | None = None,
        max_constraint_degree: int | None = None,
        max_num_variables: int | None = None,
    ) -> None:
        """Initialize model specifications with requirements."""
        self._sp = PyModelSpecs(
            sense=sense._val if sense else None,
            vtypes=[v._val for v in vtypes] if vtypes else None,
            constraints=[c._val for c in constraints] if constraints else None,
            max_degree=max_degree,
            max_constraint_degree=max_constraint_degree,
            max_num_variables=max_num_variables,
        )

    @classmethod
    def _from_pysp(cls, py_specs: PyModelSpecs) -> ModelSpecs:
        """Construct LunaModel ModelSpecs from FFI PyModelSpecs object."""
        specs = cls.__new__(cls)
        specs._sp = py_specs
        return specs

    @property
    def sense(self) -> Sense | None:
        """Get the optimization sense."""
        pys = self._sp.sense
        if pys:
            return Sense._from_pysense(pys)
        return None

    @property
    def max_degree(self) -> int | None:
        """Get the maximum objective polynomial degree."""
        return self._sp.max_degree

    @property
    def max_constraint_degree(self) -> int | None:
        """Get the maximum constraint polynomial degree."""
        return self._sp.max_constraint_degree

    @property
    def max_num_variables(self) -> int | None:
        """Get the maximum number of variables allowed."""
        return self._sp.max_num_variables

    @property
    def vtypes(self) -> list[Vtype] | None:
        """Get the list of allowed variable types."""
        if self._sp.vtypes is None:
            return None
        return [Vtype._from_pyvtype(v) for v in self._sp.vtypes]

    @property
    def constraints(self) -> list[Ctype] | None:
        """Get the list of allowed constraint types."""
        if self._sp.constraints is None:
            return None
        return [Ctype._from_pyctype(c) for c in self._sp.constraints]

    def satisfies(self, other: ModelSpecs) -> bool:
        """Check if these specifications satisfy another set of requirements.

        Parameters
        ----------
        other : ModelSpecs
            The required specifications to check against.

        Returns
        -------
        bool
            True if this model satisfies all requirements in other.
        """
        return self._sp.satisfies(other._sp)

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the model specs.
        """
        return self._sp.__str__()

    def __repr__(self) -> str:
        """Return detailed string representation.

        Returns
        -------
        str
            Detailed representation of the model specs.
        """
        return self._sp.__repr__()
