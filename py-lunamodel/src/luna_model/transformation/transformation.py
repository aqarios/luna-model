from __future__ import annotations
from typing import Any, Protocol
from abc import abstractmethod

from luna_model.model.model import Model
from luna_model.solution.sol import Solution

from luna_model.transformation.base import BasePass, ActionType
from luna_model.transformation.analysis import AnalysisCache


class TransformationOutcome:
    """Output object for transformation pass."""

    model: Model
    action: ActionType
    analysis: ...

    def __init__(
        self, model: Model, action: ActionType, analysis: object | None = None
    ) -> None: ...

    @staticmethod
    def nothing(model: Model) -> TransformationOutcome:
        """Easy nothing action return."""


class TransformationPass(BasePass, Protocol):
    @property
    @abstractmethod
    def name(self) -> str:
        """Get the name of this pass."""
        ...

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...

    @property
    def invalidates(self) -> list[str]:
        """Get a list of passes that are invalidated by this pass."""
        ...

    @abstractmethod
    def run(
        self, model: Model, cache: AnalysisCache
    ) -> (
        TransformationOutcome | tuple[Model, ActionType] | tuple[Model, ActionType, Any]
    ):
        """Run/Execute this transformation pass."""
        ...

    @abstractmethod
    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
        """Convert a solution back to fit this pass' input.

        Convert a solution from a representation fitting this pass' output to
        a solution representation fitting this pass' input.
        """
        ...
