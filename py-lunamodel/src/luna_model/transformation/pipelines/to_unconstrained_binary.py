from __future__ import annotations

from typing import TYPE_CHECKING

from luna_model._lm import PyToUnconstrainedBinaryPipeline

if TYPE_CHECKING:
    from luna_model.transformation.pipeline import PipelineProto


class ToUnconstrainedBinaryPipeline:
    """Convert a model to an unconstrained binary model.

    This pipeline transforms any model with constraints to an unconstrained binary model.
    It allows the input model to contain binary, spin or integer variables. Spin and integer
    variables are automatically converted to a binary representation. If the input model has
    linear constraints they are added to the model's objective as quadratic penalties scaled by
    the the maximum bias of the input model's objective times the `penalty_scaling` paramter.

    Notes
    -----
    If the model's constraints are not linear, an error is raised.

    Paramters
    ---------
    penalty_scaling : float
        The factor used to scale the quadratic penalties with.


    Raises
    ------
    AnalysisPassError
        If the model's constraints are not all linear.

    """

    def __new__(cls, penalty_scaling: float = 10.0) -> PipelineProto:
        """Todo."""
        return PyToUnconstrainedBinaryPipeline.create(penalty_scaling)

    def __init__(self, penalty_scaling: float = 10.0) -> None:
        self._ps = penalty_scaling
