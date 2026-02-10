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

from luna_model._lm import PySense


class Sense(Enum):
    """Optimization direction for the objective function.

    Specifies whether the goal is to minimize or maximize the objective.

    Attributes
    ----------
    MIN : str
        Minimize the objective function.
    MAX : str
        Maximize the objective function.

    Examples
    --------
    >>> from luna_model import Model, Sense
    >>> model = Model(sense=Sense.MIN)  # Minimization problem
    """

    MIN = "Minimize"
    MAX = "Maximize"

    @property
    def _val(self) -> PySense:
        match self:
            case Sense.MIN:
                return PySense.Min
            case Sense.MAX:
                return PySense.Max

    @classmethod
    def _from_pysense(cls, py_sense: PySense) -> Sense:
        match py_sense:
            case PySense.Min:
                return Sense.MIN
            case PySense.Max:
                return Sense.MAX
        msg = f"unknown sense: {py_sense}"
        raise RuntimeError(msg)
