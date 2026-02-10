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

from .action_type import ActionType

if TYPE_CHECKING:
    from luna_model._lm import PyLogElement
    from luna_model.timer import Timing


class LogElement:
    """An element of the execution log of an intermediate representation (IR)."""

    _le: PyLogElement

    @classmethod
    def _from_pyle(cls, py_le: PyLogElement) -> LogElement:
        le = cls.__new__(cls)
        le._le = py_le
        return le

    @property
    def pass_name(self) -> str:
        """The name of the pass."""
        return self._le.pass_name

    @property
    def timing(self) -> Timing:
        """Timing information for this log element."""
        return self._le.timing

    @property
    def kind(self) -> ActionType | None:
        """Transformation type information for this log element, if available."""
        at = self._le.kind
        if at is None:
            return None
        return ActionType._from_pyat(at)
