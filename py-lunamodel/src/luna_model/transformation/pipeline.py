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
from typing import overload

if sys.version_info < (3, 12):
    from typing_extensions import override
else:
    from typing import override

from luna_model._lm import PyPipeline

from .base import BasePass


class Pipeline(PyPipeline, BasePass):
    """
    A pipeline for executing multiple transformation passes in sequence.

    Pipelines organize and execute multiple passes, managing dependencies
    and ensuring they run in the correct order.

    Parameters
    ----------
    passes : list[BasePass]
        The transformation passes to include in the pipeline.
    name : str, optional
        A custom name for the pipeline. If not provided, a default name
        will be generated.
    """

    @overload
    def __init__(self, passes: list[BasePass]) -> None: ...
    @overload
    def __init__(self, passes: list[BasePass], name: str) -> None: ...
    def __init__(self, passes: list[BasePass], name: str | None = None) -> None:
        super().__init__(passes, name)

    @property
    @override
    def name(self) -> str:
        return super().name

    @property
    @override
    def requires(self) -> list[str]:
        return super().requires

    @property
    def satisfies(self) -> set[str]:
        """
        Get the set of pass requirements that this pipeline satisfies.

        Returns
        -------
        set of str
            Names of pass requirements satisfied by executing this pipeline.
        """
        return super().satisfies

    @property
    def passes(self) -> list[BasePass]:
        """
        Get all passes that are part of the pipeline.

        Returns
        -------
        list of BasePass
            The transformation passes in this pipeline.
        """
        return super().passes

    def add(self, new_pass: BasePass) -> None:
        """
        Add a new pass to the pipeline.

        Parameters
        ----------
        new_pass : BasePass
            The transformation pass to add to the pipeline.
        """
        super().add(new_pass)

    def clear(self) -> None:
        """
        Clear all passes from the pipeline.

        Removes all transformation passes, leaving an empty pipeline.
        """
        super().clear()

    def __len__(self) -> int:
        """
        Get the number of passes in the pipeline.

        Returns
        -------
        int
            The number of transformation passes in this pipeline.
        """
        return super().__len__()

    def __str__(self) -> str:
        """
        Get a string representation of the pipeline.

        Returns
        -------
        str
            A human-readable string describing the pipeline.
        """
        return super().__str__()

    def __repr__(self) -> str:
        """
        Get a detailed string representation of the pipeline for debugging.

        Returns
        -------
        str
            A detailed string representation suitable for debugging.
        """
        return super().__repr__()
