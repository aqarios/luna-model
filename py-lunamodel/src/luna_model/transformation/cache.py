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

from typing import TYPE_CHECKING, Any, Literal, Protocol, overload

from luna_model._lm import PyAnalysisCache

if TYPE_CHECKING:
    from luna_model.variable.vtype import Vtype


class MaxBias(Protocol):
    """MaxBias."""

    @property
    def val(self) -> float:
        """Get the value."""
        ...


class BinarySpinInfo(Protocol):
    """BinarySpinInfo."""

    @property
    def old_vtype(self) -> Vtype:
        """Get the source vtype."""
        ...

    @property
    def new_vtype(self) -> Vtype:
        """Get the target vtype."""
        ...

    @property
    def map(self) -> dict[str, str]:
        """Get the variable name mapping."""
        ...


class IfElseInfo(Protocol):
    """IfElseInfo."""

    @property
    def fulfilled_condition(self) -> bool:
        """If the if-else condition is fulfilled."""
        ...


class AnalysisCache:
    """AnalysisCache."""

    _ac: PyAnalysisCache

    def __init__(self) -> None:
        self._ac = PyAnalysisCache()

    @classmethod
    def _from_pyac(cls, py_ac: PyAnalysisCache) -> AnalysisCache:
        ac = cls.__new__(cls)
        ac._ac = py_ac
        return ac

    @overload
    def __getitem__(self, key: Literal["max-bias"]) -> MaxBias: ...
    @overload
    def __getitem__(self, key: Literal["binary-spin"]) -> BinarySpinInfo: ...
    @overload
    def __getitem__(self, key: str) -> Any: ...  # noqa: ANN401
    def __getitem__(self, key: str) -> Any:
        """Get the cache item for the key."""
        return self._ac[key]
