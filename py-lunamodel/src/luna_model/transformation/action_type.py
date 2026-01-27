from __future__ import annotations
from enum import Enum
from luna_model._lm import PyActionType


class ActionType(Enum):
    DID_TRANSFORM = ...
    """Indicate that the pass did transform the model."""
    DID_ANALYSIS = ...
    """Indicate that the pass did analyse the model."""
    DID_ANALYSIS_TRANSFORM = ...
    """Indicate that the pass did analyse and transfrom the model."""
    DID_NOTHING = ...
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

    @classmethod
    def _from_pyat(cls, py_action_type: PyActionType) -> ActionType:
        match py_action_type:
            case PyActionType.did_transform:
                return ActionType.DID_TRANSFORM
            case PyActionType.did_analysis:
                return ActionType.DID_ANALYSIS
            case PyActionType.did_analysis_transform:
                return ActionType.DID_ANALYSIS_TRANSFORM
            case PyActionType.did_nothing:
                return ActionType.DID_NOTHING
        raise RuntimeError("unknown action type")
