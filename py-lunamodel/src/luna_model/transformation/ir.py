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

from luna_model.model.model import Model

from .cache import AnalysisCache
from .log import LogElement

if TYPE_CHECKING:
    from luna_model._lm import PyIR


class IR:
    """Intermediate representation of a transformed model.

    The IR contains the resulting model after transformation along with the
    analysis cache and execution log generated during execution of the PassManager.

    Notes
    -----
    IR objects are typically created as the result of running the PassManager
    on a model. Use the `model`, `cache`, and `execution_log` properties to
    access the transformation results.
    """

    _ir: PyIR

    @classmethod
    def _from_pyir(cls, pyir: PyIR) -> IR:
        ir = cls.__new__(cls)
        ir._ir = pyir
        return ir

    @property
    def model(self) -> Model:
        """
        Get the transformed model.

        Returns
        -------
        Model
            The model after execution of the PassManager.
        """
        return Model._from_pym(self._ir.model)

    @property
    def cache(self) -> AnalysisCache:
        """
        Get the analysis cache.

        Returns
        -------
        AnalysisCache
            The analysis cache containing analysis results from
            running the PassManager.
        """
        return AnalysisCache._from_pyac(self._ir.cache)

    @property
    def execution_log(self) -> list[LogElement]:
        """
        Get the execution log.

        Returns
        -------
        list[LogElement]
            A list of log elements documenting the pass manager process.
        """
        return [LogElement._from_pyle(e) for e in self._ir.execution_log]
