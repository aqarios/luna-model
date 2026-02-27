from __future__ import annotations

from typing import TYPE_CHECKING

from luna_model._lm import PyToUnconstrainedBinaryPipeline

if TYPE_CHECKING:
    from luna_model.transformation.pipeline import PipelineProto


class ToUnconstrainedBinaryPipeline:
    """todo."""

    def __new__(cls, penalty_factor: float = 10.0) -> PipelineProto:
        """Todo."""
        return PyToUnconstrainedBinaryPipeline.create(penalty_factor)

    def __init__(self, penalty_factor: float = 10.0) -> None:
        self._pf = penalty_factor
