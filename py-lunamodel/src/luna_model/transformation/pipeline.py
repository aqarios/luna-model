from __future__ import annotations
from typing import overload

from luna_model._lm import PyPipeline

from .base import BasePass


class Pipeline(PyPipeline, BasePass):
    _pipeline: PyPipeline

    @overload
    def __init__(self, passes: list[BasePass]) -> None: ...
    @overload
    def __init__(self, passes: list[BasePass], name: str) -> None: ...
    def __init__(self, passes: list[BasePass], name: str | None = None) -> None:
        self._pipeline = PyPipeline(passes, name)

    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return self._pipeline.name

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._pipeline.requires

    @property
    def satisfies(self) -> set[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._pipeline.satisfies

    @property
    def passes(self) -> list[BasePass]:
        """Get all passes that are part of the pipeline."""
        return self._pipeline.passes

    def add(self, new_pass: BasePass) -> None:
        """Add new pass to pipeline."""
        self._pipeline.add(new_pass)

    def clear(self) -> None:
        """Clear pipeline."""
        self._pipeline.clear()

    def __len__(self) -> int:
        return self._pipeline.__len__()
