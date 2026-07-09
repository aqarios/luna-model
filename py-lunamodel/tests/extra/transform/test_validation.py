import pytest

from luna_model import Model
from luna_model.transformation import analyze, PassContext, PassManager, transform, NothingArtifact
from luna_model.transformation.passes import IfElsePass
from luna_model.errors import LunaModelError

def model() -> Model:
    m = Model()
    x = m.add_variables("x", 2)
    m.objective = (2 * x).sum()
    return m

def build_pass(name: str, *, requires: list[str] = []):
    @analyze(name=name, provides=name, requires=requires)
    def mock_analysis(model: Model, ctx: PassContext) -> int:
        _ = model, ctx
        return 0

    return mock_analysis

def build_transform(name: str, *, requires: list[str] = [], invalidates: list[str] = []):
    @transform(name=name, requires=requires, invalidates=invalidates)
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
    then = [build_transform("A", invalidates=["BASE"]), build_pass("BASE")]
    PassManager([build_pass("BASE"), IfElsePass(always, then=then, otherwise=[]), build_pass("REQ_BASE", requires=["BASE"])]).run(model())

def test_multi_branch_invalidate_for_downstream_fail():
    then = [build_transform("A", invalidates=["BASE"])]
    with pytest.raises(LunaModelError):
        PassManager([build_pass("BASE"), IfElsePass(always, then=then, otherwise=[]), build_pass("REQ_BASE", requires=["BASE"])]).run(model())
