from typing import TypeAlias

from luna_model.transformation.analysis import AnalysisPass
from luna_model.transformation.control_flow import ControlFlowPass
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis
from luna_model.transformation.passes.control_flow.builtin import BuiltinControlFlow
from luna_model.transformation.passes.transformation.builtin import BuiltinTransformation
from luna_model.transformation.transformation import TransformationPass

Pass: TypeAlias = (
    TransformationPass | AnalysisPass | ControlFlowPass | BuiltinTransformation | BuiltinAnalysis | BuiltinControlFlow
)
