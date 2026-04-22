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

import sys

if sys.version_info < (3, 13):
    from typing_extensions import deprecated
else:
    from warnings import deprecated

from typing import TYPE_CHECKING, TypeVar

from luna_model.errors import LunaModelError
from luna_model.model.model import Model
from luna_model.transformation.context import PassContext
from luna_model.transformation.key import AnalysisKey
from luna_model.transformation.record import TransformationRecord, TransformEntry

if TYPE_CHECKING:
    from luna_model._lm import PyTransformationOutput


class TransformationOutput:
    """The result of the PassManager's run method.

    It contains the transformed model, the TransformationRecord and the final PassContext.
    """

    _to: PyTransformationOutput

    @classmethod
    def _from_pyto(cls, pyto: PyTransformationOutput) -> TransformationOutput:
        to = cls.__new__(cls)
        to._to = pyto
        return to

    @property
    def model(self) -> Model:
        """
        Get the transformed model.

        Returns
        -------
        Model
            The model after execution of the PassManager.
        """
        return Model._from_pym(self._to.model)

    @property
    def record(self) -> TransformationRecord:
        """
        Get the transformation record produced during the PassManager execution.

        Returns
        -------
        TransformationRecord
            The transformation record after execution of the PassManager.
        """
        return TransformationRecord._from_pytr(self._to.record)

    @property
    def context(self) -> PassContext:
        """
        Get the final context produced during the PassManager execution.

        Returns
        -------
        PassContext
            The final context after execution of the PassManager.
        """
        return PassContext._from_pyctx(self._to.context)

    @property
    @deprecated(
        "The 'cache' property is deprecated and will be removed in the next release. "
        "Use the 'context' property instead."
    )
    def cache(self) -> AnalysisCache:
        """
        Deprecated cache access now replaced by the ``record`` and the ``context`` properties.

        Returns
        -------
        AnalysisCache
            Deprecated AnalysisCache for temporary backwards compatibility.
        """
        return AnalysisCache(self.context, self.record)


T = TypeVar("T")


@deprecated("The 'AnalysisCache' class is deprecated and will be removed in the next release.")
class AnalysisCache:
    """Deprecated analysis cache replaced by PassContext and TransformationRecord."""

    _ctx: PassContext
    _record: TransformationRecord

    def __init__(self, ctx: PassContext, record: TransformationRecord) -> None:
        self._ctx = ctx
        self._record = record

    @deprecated(
        "The '__getitem__' method is unstable and thus deprecated. "
        "It will be removed in the next release. "
        "Use the 'require_analysis' method of the `PassContext` instead. "
    )
    def __getitem__(self, key: str) -> T:  # type: ignore[reportInvalidTypeVarUse]
        """Get the analysis entry for the specified key.

        Parameters
        ----------
        key : str
            The cache key to retrieve as a str.

        Returns
        -------
        T
            The cached value associated with the key.
        """
        try:
            return self._ctx.require_analysis(AnalysisKey(key))
        except LunaModelError as _:
            pass

        try:
            entry = self._record.find(key, exact=False)
            if isinstance(entry, TransformEntry):
                return entry.artifact.restore()
        except LunaModelError as _:
            pass

        msg = f"could not find cache entry for '{key}'"
        raise RuntimeError(msg)
