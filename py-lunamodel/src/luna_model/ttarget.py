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

from luna_model._lm import PyTranslationTarget


class TranslationTarget(Enum):
    """Target format for model translation.

    Specifies the format to translate a model into.

    Attributes
    ----------
    QUBO : str
        Quadratic Unconstrained Binary Optimization format.
        Binary variables only, no constraints.
    LP : str
        Linear Programming format.
        Linear or quadratic objective and linear or quadratic constraints.
    MPS : str
        Mathematical Programming System format.
        Linear or quadratic objective and linear or quadratic constraints.
    BQM : str
        Binary Quadratic Model format (D-Wave).
        Binary/spin variables with quadratic terms.
    CQM : str
        Constrained Quadratic Model format (D-Wave).
        Quadratic model with constraints.
    """

    QUBO = "Qubo"
    LP = "Lp"
    MPS = "Mps"
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
            case TranslationTarget.MPS:
                return PyTranslationTarget.MPS
            case TranslationTarget.BQM:
                return PyTranslationTarget.Bqm
            case TranslationTarget.CQM:
                return PyTranslationTarget.Cqm

    @classmethod
    def _from_pyttarget(cls, py_ttarget: PyTranslationTarget) -> TranslationTarget:
        match py_ttarget:
            case PyTranslationTarget.QUBO:
                return TranslationTarget.QUBO
            case PyTranslationTarget.LP:
                return TranslationTarget.LP
            case PyTranslationTarget.MPS:
                return TranslationTarget.MPS
            case PyTranslationTarget.BQM:
                return TranslationTarget.BQM
            case PyTranslationTarget.CQM:
                return TranslationTarget.CQM
        msg = f"unknown sense: {py_ttarget}"
        raise RuntimeError(msg)
