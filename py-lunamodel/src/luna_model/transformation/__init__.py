# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from __future__ import annotations

import warnings
from typing import TYPE_CHECKING, Any

from . import passes, pipelines
from .analysis import AnalysisPass
from .artifact import NothingArtifact, TransformationPassArtifact
from .composite import CompositePass
from .context import PassContext
from .control_flow import ControlFlowPass, ControlFlowPlan
from .decorators import (
    allowed_import_prefixes,
    analyse,
    analyze,
    composite,
    control_flow,
    meta_analyse,
    meta_analyze,
    register_allowed_import_prefix,
    transform,
)
from .meta_analysis import MetaAnalysisPass
from .output import AnalysisCache, TransformationOutput
from .pass_manager import PassManager
from .pipeline import Pipeline
from .record import AnalysisEntry, ControlFlowEntry, PassEntry, PipelineEntry, TransformationRecord, TransformEntry
from .transformation import TransformationPass
from .typing import Pass

if TYPE_CHECKING:
    from .legacy import IR, TransformationOutcome
    from .passes import IfElsePass
    from .typing import Pass as BasePass


_LEGACY_EXPORTS = {
    "IfElsePass": (
        "`luna_model.transformation.IfElsePass` is deprecated and will be removed in the next release. "
        "Import `IfElsePass` from `luna_model.transformation.passes` instead.",
        lambda: passes.IfElsePass,
    ),
    "IR": (
        "`luna_model.transformation.IR` is deprecated and will be removed in the next release. "
        "Use `TransformationOutput` from `luna_model.transformation.output` instead. "
        "Behavior that previously relied on the analysis cache is now represented by "
        "`TransformationRecord` plus `PassContext`.",
        lambda: TransformationOutput,
    ),
    "TransformationOutcome": (
        "`luna_model.transformation.TransformationOutcome` is deprecated and will be removed in the next release. "
        "It is kept only so this import path resolves. It does not preserve the old runtime behavior: "
        "transformation passes now return `tuple[Model, Artifact]`, not the old outcome object API.",
        lambda: (
            __import__(
                "luna_model.transformation.legacy",
                fromlist=["TransformationOutcome"],
            ).TransformationOutcome
        ),
    ),
    "BasePass": (
        "`luna_model.transformation.BasePass` is deprecated and will be removed in the next release. "
        "Use `Pass` from `luna_model.transformation.typing` instead.",
        lambda: Pass,
    ),
}

_REMOVED_EXPORTS = {
    "ActionType": (
        "`ActionType` is no longer available from `luna_model.transformation` and "
        "has no direct compatibility shim in the current transformation API. "
        "This mainly affects custom pass implementations based on the pre-`b738a4f` API."
    ),
    "LogElement": (
        "`LogElement` is no longer available from `luna_model.transformation`. "
        "Use the structured entries on `TransformationRecord` instead, such as "
        "`TransformEntry`, `AnalysisEntry`, `PipelineEntry`, and `ControlFlowEntry`."
    ),
}


def __getattr__(name: str) -> Any:  # noqa: ANN401
    legacy = _LEGACY_EXPORTS.get(name)
    if legacy is not None:
        message, resolver = legacy
        warnings.warn(
            message,
            FutureWarning,
            stacklevel=2,
        )
        return resolver()

    removed = _REMOVED_EXPORTS.get(name)
    if removed is not None:
        raise AttributeError(removed)

    msg = f"module {__name__!r} has no attribute {name!r}"
    raise AttributeError(msg)


def __dir__() -> list[str]:
    return sorted(set(globals()) | set(_LEGACY_EXPORTS) | set(_REMOVED_EXPORTS))


__all__ = [
    "IR",
    "AnalysisCache",
    "AnalysisEntry",
    "AnalysisPass",
    "BasePass",
    "CompositePass",
    "ControlFlowEntry",
    "ControlFlowPass",
    "ControlFlowPlan",
    "IfElsePass",
    "MetaAnalysisPass",
    "NothingArtifact",
    "Pass",
    "PassContext",
    "PassEntry",
    "PassManager",
    "Pipeline",
    "PipelineEntry",
    "TransformEntry",
    "TransformationOutcome",
    "TransformationOutput",
    "TransformationPass",
    "TransformationPassArtifact",
    "TransformationRecord",
    "allowed_import_prefixes",
    "analyse",
    "analyze",
    "composite",
    "control_flow",
    "meta_analyse",
    "meta_analyze",
    "passes",
    "pipelines",
    "register_allowed_import_prefix",
    "transform",
]
