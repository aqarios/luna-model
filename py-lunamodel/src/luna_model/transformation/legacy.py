import warnings
from typing import TypeAlias, TypeVar

from luna_model.model.model import Model
from luna_model.transformation.artifact import TransformationPassArtifact
from luna_model.transformation.output import TransformationOutput
from luna_model.transformation.passes.control_flow.ifelse import IfElsePass

warnings.warn(
    "`luna_model.transformation.legacy` is deprecated. "
    "Import `IfElsePass` from `luna_model.transformation.passes`, "
    "`IR` from `luna_model.transformation.output`, and "
    "`TransformationOutcome` from `luna_model.transformation.legacy` only as a temporary compatibility shim.",
    FutureWarning,
    stacklevel=2,
)

Artifact = TypeVar("Artifact", bound=TransformationPassArtifact)
TransformationOutcome: TypeAlias = tuple[Model, Artifact]
IR: TypeAlias = TransformationOutput

__all__ = ["IR", "IfElsePass", "TransformationOutcome"]
