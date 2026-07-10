from luna_model.transformation import PassManager
from luna_model.transformation.pipeline import Pipeline
from luna_model.transformation.pipelines import ToBinaryMinimizationPipeline


def test_init_with_pipeline():
    _ = PassManager(Pipeline([ToBinaryMinimizationPipeline()], name="myname"))
    _ = PassManager(ToBinaryMinimizationPipeline())


def test_pipeline_display_does_not_duplicate_last_step():
    pm = PassManager(ToBinaryMinimizationPipeline())

    assert str(pm) == (
        "PassManager\n"
        "🛢️ to-binary-minimization  \n"
        "  🔎 check-specs\n"
        "  ⚙️ change-sense\n"
        "  ⚙️ binary-spin\n"
        "  ⚙️ integer-to-binary"
    )
