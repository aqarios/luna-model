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

from typing import TypeAlias

from luna_model._lm import PyBounds, PyUnbounded
from luna_model.variable.vtype import Vtype

Unbounded: TypeAlias = PyUnbounded
"""Indicating the lower (-inf) or upper bound (+inf)."""


class Bounds:
    """Bounds for constraining variable values.

    Bounds specify the lower and upper limits on the values a variable can take
    during optimization. Bounds can be finite values or unbounded (infinite).

    Parameters
    ----------
    lower : float or type[Unbounded], optional
        The lower bound. Use ``Unbounded`` for negative infinity.
        Use ``None`` for the default value based on a variable's vtype.
    upper : float or type[Unbounded], optional
        The upper bound. Use ``Unbounded`` for positive infinity.
        Use ``None`` for the default value based on a variable's vtype.

    Attributes
    ----------
    lower : float or type[Unbounded] or None
        The lower bound value. If set to ``None`` the lower bound will be determined
        based on the variable's vtype this bounds object is be associated with during
        variable creation.
    upper : float or type[Unbounded] or None
        The upper bound value. If set to ``None`` the upper bound will be determined
        based on the variable's vtype this bounds object is be associated with during
        variable creation.
    """

    _b: PyBounds

    def __init__(
        self,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
    ) -> None:
        self._b = PyBounds(lower, upper)

    @classmethod
    def _from_pyb(cls, py_b: PyBounds) -> Bounds:
        b = cls.__new__(cls)
        b._b = py_b
        return b

    @property
    def lower(self) -> float | type[Unbounded] | None:
        """Get the lower bound.

        Returns
        -------
        float or type[Unbounded], optional
            The lower bound value if set (Unbounded for negative infinity).
        """
        return self._b.lower

    @property
    def upper(self) -> float | type[Unbounded] | None:
        """Get the upper bound.

        Returns
        -------
        float or type[Unbounded], optional
            The upper bound value if set (Unbounded for positive infinity).
        """
        return self._b.upper

    @classmethod
    def default(cls, vtype: Vtype) -> Bounds:
        """Get the default bounds for a variable type.

        Parameters
        ----------
        vtype : Vtype
            The variable type.

        Returns
        -------
        Bounds
            Default bounds for the specified vtype.
        """
        match vtype:
            case Vtype.BINARY | Vtype.INVERTED_BINARY:
                return cls.binary()
            case Vtype.SPIN:
                return cls.spin()
            case Vtype.INTEGER:
                return cls.integer()
            case Vtype.REAL:
                return cls.real()

    @classmethod
    def binary(cls) -> Bounds:
        """Get the bounds for binary variables.

        Returns
        -------
        Bounds
            Bounds with lower=0, upper=1.
        """
        return cls._from_pyb(PyBounds.binary())

    @classmethod
    def spin(cls) -> Bounds:
        """Get the bounds for spin variables.

        Returns
        -------
        Bounds
            Bounds with lower=-1, upper=1.
        """
        return cls._from_pyb(PyBounds.spin())

    @classmethod
    def integer(cls) -> Bounds:
        """Get the bounds for integer variables.

        Returns
        -------
        Bounds
            Bounds with typical integer range limits.
        """
        return cls._from_pyb(PyBounds.integer())

    @classmethod
    def real(cls) -> Bounds:
        """Get the bounds for real variables.

        Returns
        -------
        Bounds
            Bounds for real-valued variables.
        """
        return cls._from_pyb(PyBounds.real())

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the bounds.
        """
        return self._b.__str__()

    def __repr__(self) -> str:
        """Return detailed string representation.

        Returns
        -------
        str
            Detailed representation of the bounds.
        """
        return self._b.__repr__()

    def __eq__(self, other: Bounds) -> bool:  # type: ignore[override]
        """Check equality with another Bounds object.

        Parameters
        ----------
        other : Bounds
            The bounds to compare with.

        Returns
        -------
        bool
            True if bounds are equal, False otherwise.
        """
        return self._b.__eq__(other._b)

    def __hash__(self) -> int:
        """Compute hash for use in sets and dictionaries.

        Returns
        -------
        int
            Hash value of the bounds.
        """
        return self._b.__hash__()
