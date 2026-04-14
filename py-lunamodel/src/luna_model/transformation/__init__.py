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

from . import passes
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
from .pass_manager import PassManager
from .pipeline import Pipeline
from .record import AnalysisEntry, ControlFlowEntry, PassEntry, PipelineEntry, TransformationRecord, TransformEntry
from .transformation import TransformationPass
from .typing import Pass

__all__ = [
    "AnalysisEntry",
    "AnalysisPass",
    "CompositePass",
    "ControlFlowEntry",
    "ControlFlowPass",
    "ControlFlowPlan",
    "MetaAnalysisPass",
    "NothingArtifact",
    "Pass",
    "PassContext",
    "PassEntry",
    "PassManager",
    "Pipeline",
    "PipelineEntry",
    "TransformEntry",
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
    "register_allowed_import_prefix",
    "transform",
]
