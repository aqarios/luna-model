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

from typing import TYPE_CHECKING, Literal, TypeVar, overload

from luna_model._lm import PyPassContext
from luna_model.errors import CompilationError

if TYPE_CHECKING:
    from luna_model.transformation.key import AnalysisKey

T = TypeVar("T")


class PassContext(PyPassContext):
    """The pass context provides access to the analysis results in Transformation and analysis passes."""

    _c: PyPassContext

    def __init__(self) -> None:
        self._c = PyPassContext()

    @classmethod
    def _from_pyctx(cls, py_ctx: PyPassContext) -> PassContext:
        ctx = cls.__new__(cls)
        ctx._c = py_ctx
        return ctx

    @overload
    def require_analysis(self, key: AnalysisKey[T]) -> T | None: ...
    @overload
    def require_analysis(self, key: AnalysisKey[T], *, allow_none: Literal[True]) -> T | None: ...
    @overload
    def require_analysis(self, key: AnalysisKey[T], *, allow_none: Literal[False]) -> T: ...
    def require_analysis(self, key: AnalysisKey[T], *, allow_none: bool = False) -> T | None:
        """Get the analysis entry for the specified key.

        Parameters
        ----------
        key : AnalysisKey[T]
            The cache key to retrieve.
        allow_none : bool
            If set to ``True`` None is returned instead of raising an error if no
            result for ``key`` can be found.


        Returns
        -------
        T
            The cached value associated with the key.
        """
        try:
            return self._c.require_analysis(key.name)
        except CompilationError as e:
            if allow_none:
                return None
            raise e from e
