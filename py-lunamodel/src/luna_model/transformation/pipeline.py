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

from typing import overload

from luna_model._lm import PyPipeline

from .base import BasePass


class Pipeline(PyPipeline, BasePass):
    """Pipeline."""

    @overload
    def __init__(self, passes: list[BasePass]) -> None: ...
    @overload
    def __init__(self, passes: list[BasePass], name: str) -> None: ...
    def __init__(self, passes: list[BasePass], name: str | None = None) -> None:
        super().__init__(passes, name)

    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return super().name

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        return super().requires

    @property
    def satisfies(self) -> set[str]:
        """Get a list of required passes that need to be run before this pass."""
        return super().satisfies

    @property
    def passes(self) -> list[BasePass]:
        """Get all passes that are part of the pipeline."""
        return super().passes

    def add(self, new_pass: BasePass) -> None:
        """Add new pass to pipeline."""
        super().add(new_pass)

    def clear(self) -> None:
        """Clear pipeline."""
        super().clear()

    def __len__(self) -> int:
        """Get the length."""
        return super().__len__()

    def __str__(self) -> str:
        """Pipeline as string."""
        return super().__str__()

    def __repr__(self) -> str:
        """Pipeline as debug string."""
        return super().__repr__()
