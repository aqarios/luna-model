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

from enum import Enum

from luna_model._lm import PyActionType


class ActionType(Enum):
    """Enumeration of action types that describe what a pass performed.

    This enum classifies the types of operations performed by transformation/analysis
    passes on the model, such as analysis, transformation, or composite operations.
    """

    DID_TRANSFORM = "DidTransform"
    """Indicate that the pass did transform the model."""
    DID_ANALYSIS = "DidAnalysis"
    """Indicate that the pass did analyse the model."""
    DID_ANALYSIS_TRANSFORM = "DidAnalysisTransform"
    """Indicate that the pass did analyse and transfrom the model."""
    DID_IF_ELSE = "DidIfElse"
    """Indicate that the pass did an ifelse pass."""
    DID_PIPELINE = "DidPipeline"
    """Indicate that the pass did a pipeline pass."""
    DID_NOTHING = "DidNothing"
    """Indicate that the pass did NOT do anything."""

    @property
    def _val(self) -> PyActionType:
        match self:
            case ActionType.DID_TRANSFORM:
                return PyActionType.DidTransform
            case ActionType.DID_ANALYSIS:
                return PyActionType.DidAnalysis
            case ActionType.DID_ANALYSIS_TRANSFORM:
                return PyActionType.DidAnalysisTransform
            case ActionType.DID_NOTHING:
                return PyActionType.DidNothing
            case ActionType.DID_IF_ELSE:
                return PyActionType.DidIfElse
            case ActionType.DID_PIPELINE:
                return PyActionType.DidPipeline

    @classmethod
    def _from_pyat(cls, py_action_type: PyActionType) -> ActionType:
        match py_action_type:
            case PyActionType.DidTransform:
                return ActionType.DID_TRANSFORM
            case PyActionType.DidAnalysis:
                return ActionType.DID_ANALYSIS
            case PyActionType.DidAnalysisTransform:
                return ActionType.DID_ANALYSIS_TRANSFORM
            case PyActionType.DidIfElse:
                return ActionType.DID_IF_ELSE
            case PyActionType.DidPipeline:
                return ActionType.DID_PIPELINE
            case PyActionType.DidNothing:
                return ActionType.DID_NOTHING
        msg = f"unknown action type: {py_action_type}"
        raise RuntimeError(msg)
