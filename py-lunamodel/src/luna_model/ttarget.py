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
from enum import Enum

from luna_model._lm import PyTranslationTarget


class TranslationTarget(Enum):
    """Target format for model translation.

    Specifies the mathematical programming format to translate a model into.
    Different solvers and platforms require different formats.

    Attributes
    ----------
    QUBO : str
        Quadratic Unconstrained Binary Optimization format.
        Binary variables only, no constraints.
    LP : str
        Linear Programming format.
        Linear objective and constraints.
    BQM : str
        Binary Quadratic Model format (D-Wave).
        Binary/spin variables with quadratic terms.
    CQM : str
        Constrained Quadratic Model format (D-Wave).
        Quadratic model with constraints.

    Examples
    --------
    >>> from luna_model.ttarget import TranslationTarget
    >>> target = TranslationTarget.QUBO

    See Also
    --------
    Model : Model class that can be translated to these formats.
    """

    QUBO = "Qubo"
    LP = "Lp"
    BQM = "Bqm"
    CQM = "Cqm"

    @property
    def _val(self) -> PyTranslationTarget:
        """Convert Python TranslationTarget to internal representation."""
        match self:
            case TranslationTarget.QUBO:
                return PyTranslationTarget.Qubo
            case TranslationTarget.LP:
                return PyTranslationTarget.Lp
            case TranslationTarget.BQM:
                return PyTranslationTarget.Bqm
            case TranslationTarget.CQM:
                return PyTranslationTarget.Cqm
