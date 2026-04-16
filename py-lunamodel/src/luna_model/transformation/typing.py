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
