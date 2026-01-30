from __future__ import annotations

from typing import TYPE_CHECKING, Protocol

if TYPE_CHECKING:
    from luna_model.solution.sample import Sample


class Result(Protocol):
    """Result."""

    @property
    def sample(self) -> Sample:
        """Get sample."""
        ...

    @property
    def obj_value(self) -> float | None:
        """Get obj_value."""
        ...

    @property
    def constraints(self, /) -> dict[str, bool] | None:
        """Get constraints."""
        ...

    @property
    def variable_bounds(self, /) -> dict[str, bool] | None:
        """Get variable bounds."""
        ...

    @property
    def feasible(self) -> bool | None:
        """Get feasible."""
        ...


class ResultView(Result, Protocol):
    """Result view."""

    @property
    def counts(self, /) -> int:
        """Get counts."""
        ...

    @property
    def raw_energy(self, /) -> float | None:
        """Get raw energy."""
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
    """Result iterator."""

    def __iter__(self) -> ResultIter:
        """Iterate."""
        ...

    def __next__(self) -> ResultView:
        """Get next item."""
        ...
