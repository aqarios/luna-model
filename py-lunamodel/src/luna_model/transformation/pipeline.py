from typing import overload
from luna_model._lm import PyPipeline

from luna_model.transformation.base import BasePass


class Pipeline(BasePass):
    _p: PyPipeline

    @overload
    def __init__(self, passes: list[BasePass]) -> None: ...
    @overload
    def __init__(self, passes: list[BasePass], name: str) -> None: ...
    def __init__(self, passes: list[BasePass], name: str | None = None) -> None:
        self._p = PyPipeline(passes=passes, name=name)

    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return self._p.name

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._p.requires

    @property
    def satisfies(self) -> set[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._p.satisfies

    def add(self, new_pass: BasePass) -> None:
        """Add new pass to pipeline."""
        return self._p.add(new_pass)

    def clear(self) -> None:
        """Clear pipeline."""
        return self._p.clear()

    @property
    def passes(self) -> list[BasePass]:
        """Get all passes that are part of the pipeline."""
        return self._p.passes

    def __len__(self) -> int:
        return self._p.__len__()
