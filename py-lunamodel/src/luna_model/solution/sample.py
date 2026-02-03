from __future__ import annotations

from typing import TYPE_CHECKING, Protocol, overload

if TYPE_CHECKING:
    from collections.abc import Sequence

    from luna_model.variable import Variable


class Sample(Protocol):
    """Sample."""

    def to_dict(self) -> dict[str, int | float]:
        """Placeholding docsting."""
        ...

    def __getitem__(self, item: int | Variable | str) -> int | float:
        """Placeholding docsting."""
        ...

    def __len__(self) -> int:
        """Placeholding docsting."""
        ...

    def __iter__(self) -> SampleIter:
        """Placeholding docsting."""
        ...

    def __str__(self, /) -> str:
        """Placeholding docsting."""
        ...


class Samples(Protocol):
    """Samples."""

    def tolist(self) -> Sequence[Sequence[int | float]]:
        """Placeholding docsting."""
        ...

    @overload
    def __getitem__(self, item: int) -> Sample: ...
    @overload
    def __getitem__(self, item: tuple[int, int]) -> float: ...
    def __getitem__(self, item: int | tuple[int, int]) -> Sample | float:
        """Placeholding docsting."""
        ...

    def __len__(self) -> int:
        """Placeholding docsting."""
        ...

    def __iter__(self) -> SamplesIter:
        """Placeholding docsting."""
        ...


class SamplesIter(Protocol):
    """Samples iterator."""

    def __iter__(self) -> SamplesIter:
        """Placeholding docsting."""
        ...

    def __next__(self) -> Sample:
        """Placeholding docsting."""
        ...


class SampleIter(Protocol):
    """Sample iterator."""

    def __iter__(self) -> SampleIter:
        """Placeholding docsting."""
        ...

    def __next__(self) -> int | float:
        """Placeholding docsting."""
        ...
