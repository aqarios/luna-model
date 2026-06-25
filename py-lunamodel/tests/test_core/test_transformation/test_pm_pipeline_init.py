from luna_model.transformation import PassManager
from luna_model.transformation.pipeline import Pipeline
from luna_model.transformation.pipelines import ToBinaryMinimizationPipeline

def test_init_with_pipeline():
    _ = PassManager(Pipeline([ToBinaryMinimizationPipeline()], name="myname"))
    _ = PassManager(ToBinaryMinimizationPipeline())
