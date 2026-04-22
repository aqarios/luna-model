import warnings
from typing import TypeAlias, TypeVar

from luna_model.model.model import Model
from luna_model.transformation.artifact import TransformationPassArtifact
from luna_model.transformation.output import TransformationOutput
from luna_model.transformation.passes.control_flow.ifelse import IfElsePass

warnings.warn(
    "`luna_model.transformation.legacy` is deprecated and will be removed in the next release. "
    "`IfElsePass` must be imported from `luna_model.transformation.passes`. "
    "`IR` is replaced by `TransformationOutput` from `luna_model.transformation.output`. "
    "Behavior that previously relied on the analysis cache is now represented by "
    "`TransformationRecord` plus `PassContext`. "
    "`TransformationOutcome` is also deprecated and only kept so the import path resolves; "
    "it does not preserve the old runtime behavior because transformation passes now return "
    "`tuple[Model, Artifact]`.",
    FutureWarning,
    stacklevel=2,
)

Artifact = TypeVar("Artifact", bound=TransformationPassArtifact)
TransformationOutcome: TypeAlias = tuple[Model, Artifact]
IR: TypeAlias = TransformationOutput

__all__ = ["IR", "IfElsePass", "TransformationOutcome"]
