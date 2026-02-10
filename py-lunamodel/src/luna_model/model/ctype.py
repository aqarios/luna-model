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

from luna_model._lm import PyCtype


class Ctype(Enum):
    """Types of constraints in optimization models.

    Categorizes constraints by their comparison operators and structure.

    Attributes
    ----------
    UNCONSTRAINED : str
        No constraints (unconstrained optimization).
    EQUALITY : str
        Equality constraints (``==``).
    INEQUALITY : str
        Inequality constraints (``<=`` or ``>=``).
    LESS_EQUAL : str
        Less-than-or-equal constraints (``<=``).
    GREATER_EQUAL : str
        Greater-than-or-equal constraints (``>=``).

    Examples
    --------
    >>> from luna_model import Ctype, ModelSpecs
    >>> specs = ModelSpecs(constraints={Ctype.LESS_EQUAL})
    """

    UNCONSTRAINED = "Unconstrained"
    EQUALITY = "Equality"
    INEQUALITY = "Inequality"
    LESS_EQUAL = "LessEqual"
    GREATER_EQUAL = "GreaterEqual"

    @property
    def _val(self) -> PyCtype:
        match self:
            case Ctype.UNCONSTRAINED:
                return PyCtype.Unconstrained
            case Ctype.EQUALITY:
                return PyCtype.Equality
            case Ctype.INEQUALITY:
                return PyCtype.Inequality
            case Ctype.LESS_EQUAL:
                return PyCtype.LessEqual
            case Ctype.GREATER_EQUAL:
                return PyCtype.GreaterEqual

    @classmethod
    def _from_pyctype(cls, py_ctype: PyCtype) -> Ctype:
        match py_ctype:
            case PyCtype.Unconstrained:
                return Ctype.UNCONSTRAINED
            case PyCtype.Equality:
                return Ctype.EQUALITY
            case PyCtype.Inequality:
                return Ctype.INEQUALITY
            case PyCtype.LessEqual:
                return Ctype.LESS_EQUAL
            case PyCtype.GreaterEqual:
                return Ctype.GREATER_EQUAL
        msg = f"unknown ctype: {py_ctype}"
        raise RuntimeError(msg)
