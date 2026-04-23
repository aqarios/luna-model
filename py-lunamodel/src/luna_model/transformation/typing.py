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

from typing import TypeAlias

from luna_model.transformation.analysis import AnalysisPass
from luna_model.transformation.composite import CompositePass
from luna_model.transformation.control_flow import ControlFlowPass
from luna_model.transformation.meta_analysis import MetaAnalysisPass
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis
from luna_model.transformation.passes.composite.builtin import BuiltinComposite
from luna_model.transformation.passes.control_flow.builtin import BuiltinControlFlow
from luna_model.transformation.passes.meta_analysis.builtin import BuiltinMetaAnalysis
from luna_model.transformation.passes.transformation.builtin import BuiltinTransformation
from luna_model.transformation.pipeline import Pipeline
from luna_model.transformation.transformation import TransformationPass

Pass: TypeAlias = (
    AnalysisPass
    | CompositePass
    | ControlFlowPass
    | MetaAnalysisPass
    | TransformationPass
    | BuiltinAnalysis
    | BuiltinComposite
    | BuiltinControlFlow
    | BuiltinMetaAnalysis
    | BuiltinTransformation
    | Pipeline
)
