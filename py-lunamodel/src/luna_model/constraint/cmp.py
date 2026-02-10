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

from enum import Enum

from luna_model._lm import PyComparator


class Comparator(Enum):
    """Comparison operators for constraints.

    Defines the type of comparison used in a constraint between the left-hand
    side expression and the right-hand side value.

    Attributes
    ----------
    EQ : str
        Equality constraint (``==``). Requires lhs to equal rhs.
    LE : str
        Less-than-or-equal constraint (``<=``). Requires lhs to be at most rhs.
    GE : str
        Greater-than-or-equal constraint (``>=``). Requires lhs to be at least rhs.

    Examples
    --------
    Comparators are typically created automatically through operator overloading:

    >>> from luna_model import Variable, Environment
    >>> with Environment():
    ...     x = Variable("x")
    ...     c1 = x == 1  # Creates EQ constraint
    ...     c2 = x <= 5  # Creates LE constraint
    ...     c3 = x >= 0  # Creates GE constraint

    Notes
    -----
    Strict inequality (``<``, ``>``) is not supported in optimization constraints.
    """

    EQ = "Eq"
    LE = "Le"
    GE = "Ge"

    @property
    def _val(self) -> PyComparator:
        """Convert Python Comparator to internal PyComparator representation."""
        match self:
            case Comparator.EQ:
                return PyComparator.Eq
            case Comparator.LE:
                return PyComparator.Le
            case Comparator.GE:
                return PyComparator.Ge

    @classmethod
    def _from_pycmp(cls, py_cmp: PyComparator) -> Comparator:
        """Convert internal PyComparator representation to Python Comparator."""
        match py_cmp:
            case PyComparator.Eq:
                return Comparator.EQ
            case PyComparator.Le:
                return Comparator.LE
            case PyComparator.Ge:
                return Comparator.GE
        msg = f"unknown comparator '{py_cmp}'"
        raise RuntimeError(msg)
