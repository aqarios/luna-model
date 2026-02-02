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
