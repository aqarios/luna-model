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

from typing import TYPE_CHECKING, Protocol

if TYPE_CHECKING:
    from luna_model.solution.sample import Sample


class Result(Protocol):
    """Protocol for solution results.

    Provides access to solution information including the sample,
    objective value, feasibility, and constraint satisfaction.

    Attributes
    ----------
    sample : Sample
        The variable assignments in this result.
    obj_value : float | None
        The objective function value, if available.
    constraints : dict[str, bool] | None
        Constraint satisfaction status by constraint name.
    variable_bounds : dict[str, bool] | None
        Variable bound satisfaction status by variable name.
    feasible : bool | None
        Whether the solution is feasible, if known.
    """

    @property
    def sample(self) -> Sample:
        """Get the variable assignments as a Sample."""
        ...

    @property
    def obj_value(self) -> float | None:
        """Get the objective function value."""
        ...

    @property
    def constraints(self, /) -> dict[str, bool] | None:
        """Get constraint satisfaction status."""
        ...

    @property
    def variable_bounds(self, /) -> dict[str, bool] | None:
        """Get variable bound satisfaction status."""
        ...

    @property
    def feasible(self) -> bool | None:
        """Get feasibility status."""
        ...


class ResultView(Result, Protocol):
    """Extended result view with additional metadata.

    Extends Result with counts, raw energy, and comparison capabilities.

    Attributes
    ----------
    counts : int
        Number of times this result was observed.
    raw_energy : float | None
        Raw energy value from the solver, if available.
    """

    @property
    def counts(self, /) -> int:
        """Get the number of times this result was observed."""
        ...

    @property
    def raw_energy(self, /) -> float | None:
        """Get the raw energy from the solver."""
        ...

    def __str__(self, /) -> str:
        """Get str."""
        ...

    def __repr__(self, /) -> str:
        """Get debug str."""
        ...

    def __eq__(self, other: ResultView, /) -> bool:  # type: ignore[reportIncompatibleMethodOverride]
        """Check equality."""
        ...


class ResultIter(Protocol):
    """Iterator over result views.

    Iterates over ResultView objects in a solution.
    """

    def __iter__(self) -> ResultIter:
        """Return the iterator object itself."""
        ...

    def __next__(self) -> ResultView:
        """Get the next result view."""
        ...
