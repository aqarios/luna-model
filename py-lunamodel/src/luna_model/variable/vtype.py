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

from luna_model._lm import PyVtype


class Vtype(Enum):
    """Enumeration of variable types for optimization models.

    The variable type determines the domain of values a variable can take
    during optimization. Different problem types and solvers support different
    variable types.

    Attributes
    ----------
    BINARY : str
        Binary variable that can be 0 or 1. Used for yes/no decisions.
    INVERTED_BINARY : str
        Inverted binary variable. Internal representation that maps 0→1 and 1→0.
        Not typically used directly by users.
    SPIN : str
        Spin variable that can be -1 or +1. Common in quantum computing formulations.
    INTEGER : str
        Integer variable that can be any integer value within bounds.
    REAL : str
        Real-valued (continuous) variable that can be any floating-point value within bounds.

    Examples
    --------
    Create different types of variables:

    >>> from luna_model import Variable, Vtype
    >>> x = Variable("x", vtype=Vtype.BINARY)
    >>> y = Variable("y", vtype=Vtype.INTEGER, bounds=(0, 10))
    >>> z = Variable("z", vtype=Vtype.REAL, bounds=(0.0, 1.0))

    Notes
    -----
    The default variable type is ``BINARY`` when not specified.

    See Also
    --------
    Variable : Decision variable class that uses this type enumeration.
    Bounds : Bounds class for constraining variable values.
    """

    BINARY = "Binary"
    INVERTED_BINARY = "InvertedBinary"
    SPIN = "Spin"
    INTEGER = "Integer"
    REAL = "Real"

    @property
    def _val(self) -> PyVtype:
        """Convert Python Vtype to internal PyVtype representation."""
        match self:
            case Vtype.BINARY:
                return PyVtype.Binary
            case Vtype.INVERTED_BINARY:
                return PyVtype.InvertedBinary
            case Vtype.SPIN:
                return PyVtype.Spin
            case Vtype.INTEGER:
                return PyVtype.Integer
            case Vtype.REAL:
                return PyVtype.Real

    @classmethod
    def _from_pyvtype(cls, py_vtype: PyVtype) -> Vtype:
        """Convert internal PyVtype representation to Python Vtype."""
        match py_vtype:
            case PyVtype.Binary:
                return Vtype.BINARY
            case PyVtype.InvertedBinary:
                return Vtype.INVERTED_BINARY
            case PyVtype.Spin:
                return Vtype.SPIN
            case PyVtype.Integer:
                return Vtype.INTEGER
            case PyVtype.Real:
                return Vtype.REAL
        msg = f"unknown vtype '{py_vtype}'"
        raise RuntimeError(msg)
