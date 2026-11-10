import pytest

from collections.abc import Iterable

from luna_model import Model, ModelPart, Sense
from luna_model.transformation import analyze, PassContext, PassManager, Pipeline, transform, NothingArtifact
from luna_model.transformation.passes import ChangeSensePass, IfElsePass
from luna_model.errors import LunaModelError

def model() -> Model:
    m = Model()
    x = m.add_variables("x", 2)
    m.objective = (2 * x).sum()
    return m

def build_pass(name: str, *, requires: list[str] = [], depends_on: Iterable[ModelPart] | None = None):
    @analyze(name=name, provides=name, requires=requires, depends_on=depends_on)
    def mock_analysis(model: Model, ctx: PassContext) -> int:
        _ = model, ctx
        return 0

    return mock_analysis

def build_transform(name: str, *, requires: list[str] = [], modifies: Iterable[ModelPart] | None = None):
    @transform(name=name, requires=requires, modifies=modifies)
    def mock_analysis(model: Model, ctx: PassContext) -> tuple[Model, NothingArtifact]:
        _ = ctx
        return model, NothingArtifact()

    return mock_analysis

def always(m: Model, ctx: PassContext) -> bool:
    _ = m, ctx
    return True


def test_passes_in_order_ok():
    A = build_pass("A")
    B = build_pass("B", requires=["A"])
    C = build_pass("C", requires=["B"])
    PassManager([A, B, C]).run(model())

def test_passes_out_of_order_fail():
    A = build_pass("A")
    B = build_pass("B", requires=["A"])
    C = build_pass("C", requires=["B"])
    with pytest.raises(LunaModelError):
        PassManager([B, A, C]).run(model())

def test_single_branch_inner_ok():
    then = [build_pass("A"), build_pass("B", requires=["A"])]
    PassManager([IfElsePass(always, then=then, otherwise=[])]).run(model())

def test_single_branch_inner_fail():
    then = [build_pass("B", requires=["A"]), build_pass("A")]
    with pytest.raises(LunaModelError):
        PassManager([IfElsePass(always, then=then, otherwise=[])]).run(model())

def test_single_branch_outer_ok():
    then = [build_pass("B", requires=["A"])]
    PassManager([build_pass("A"), IfElsePass(always, then=then, otherwise=[])]).run(model())

def test_single_branch_outer_fail():
    then = [build_pass("B", requires=["A"])]
    with pytest.raises(LunaModelError):
        PassManager([IfElsePass(always, then=then, otherwise=[])]).run(model())

def test_multi_branch_inner_ok():
    then = [build_pass("A"), build_pass("B", requires=["A"])]
    otherwise = [build_pass("Aa"), build_pass("Bb", requires=["Aa"])]
    PassManager([IfElsePass(always, then=then, otherwise=otherwise)]).run(model())

def test_multi_branch_inner_fail():
    then = [build_pass("A"), build_pass("B", requires=["A"])]
    otherwise = [build_pass("Bb", requires=["Aa"])]
    with pytest.raises(LunaModelError):
        PassManager([IfElsePass(always, then=then, otherwise=otherwise)]).run(model())

def test_multi_branch_outer_ok():
    then = [build_pass("B", requires=["A"])]
    otherwise = [build_pass("Bb", requires=["A"])]
    PassManager([build_pass("A"), IfElsePass(always, then=then, otherwise=otherwise)]).run(model())

def test_multi_branch_downstream_ok():
    then = [build_pass("A"), build_pass("C")]
    otherwise = [build_pass("A")]
    PassManager([IfElsePass(always, then=then, otherwise=otherwise), build_pass("D", requires=["A"])]).run(model())

def test_multi_branch_downstream_fail():
    then = [build_pass("A_a"), build_pass("C")]
    otherwise = [build_pass("A_b")]
    with pytest.raises(LunaModelError):
        PassManager([IfElsePass(always, then=then, otherwise=otherwise), build_pass("D", requires=["A"])]).run(model())

def test_multi_branch_invalidate_and_provides_for_downstream_ok():
    then = [build_transform("A"), build_pass("BASE")]
    PassManager([build_pass("BASE"), IfElsePass(always, then=then, otherwise=[]), build_pass("REQ_BASE", requires=["BASE"])]).run(model())

def test_multi_branch_invalidate_for_downstream_fail():
    then = [build_transform("A")]
    with pytest.raises(LunaModelError):
        PassManager([build_pass("BASE"), IfElsePass(always, then=then, otherwise=[]), build_pass("REQ_BASE", requires=["BASE"])]).run(model())

def test_transform_keeps_analysis_on_other_parts():
    base = build_pass("BASE", depends_on={ModelPart.CONSTRAINTS})
    rewrite = build_transform("rewrite", modifies={ModelPart.OBJECTIVE})
    PassManager([base, rewrite, build_pass("REQ_BASE", requires=["BASE"])]).run(model())

def test_transform_invalidates_overlapping_analysis():
    base = build_pass("BASE", depends_on={ModelPart.CONSTRAINTS, ModelPart.VARIABLES})
    rewrite = build_transform("rewrite", modifies={ModelPart.CONSTRAINTS})
    with pytest.raises(LunaModelError, match="computed by 'BASE' but invalidated by 'rewrite' which modifies \\{Constraints\\}"):
        PassManager([base, rewrite, build_pass("REQ_BASE", requires=["BASE"])]).run(model())

def test_invalidates_is_deprecated_and_ignored():
    with pytest.warns(DeprecationWarning, match="invalidates"):
        rewrite = transform(name="rewrite", invalidates=["BASE"])(lambda m, ctx: (m, NothingArtifact()))
    assert rewrite.modifies() == set(ModelPart)

def test_declared_parts_round_trip():
    assert build_transform("t", modifies={ModelPart.SENSE}).modifies() == {ModelPart.SENSE}
    assert build_pass("p").depends_on() == set(ModelPart)
    assert ChangeSensePass(Sense.MIN).modifies() == {ModelPart.OBJECTIVE, ModelPart.SENSE}
    assert Pipeline([ChangeSensePass(Sense.MIN), build_pass("p")], name="pipe").modifies() == {
        ModelPart.OBJECTIVE,
        ModelPart.SENSE,
    }

def test_verify_modifies_catches_undeclared_change():
    @transform(name="liar", modifies={ModelPart.SENSE})
    def liar(model: Model, ctx: PassContext) -> tuple[Model, NothingArtifact]:
        _ = ctx
        model.objective = model.objective * 2
        return model, NothingArtifact()

    with pytest.raises(LunaModelError, match="pass 'liar' modified \\{Objective\\} without declaring it"):
        PassManager([liar], verify_modifies=True).run(model())
    PassManager([liar], verify_modifies=False).run(model())
