"""End-to-end test of a C++-authored plugin, driven entirely from Python.

The passes are written in C++ with the header-only ``lm::plugin`` SDK and exposed via nanobind
(``luna_cxx_plugin_ext``). The user imports the plugin, registers it, and builds a ``PassManager``
mixing C++ passes with a built-in one — never touching the C ABI or the descriptor. This mirrors the
pyo3 plugin test exactly, proving the cxx SDK is on par with the Rust one.
"""

import importlib

import pytest

import luna_model
from luna_model import Model, Sense, Solution, Vtype
from luna_model.transformation import PassManager
from luna_model.transformation.passes import ChangeSensePass, MinValueForConstraintAnalysis


@pytest.fixture(scope="session")
def plugin():
    module = importlib.import_module("luna_cxx_plugin_ext")
    # The one registration call the user makes; installs the C++ plugin's reversal handlers.
    luna_model.register(module)
    return module


def test_cxx_transform_forward_and_reverse(plugin):
    model = Model("original")

    # A C++ plugin pass and a built-in pass, mixed in one manager.
    pm = PassManager([plugin.RenamePass(), ChangeSensePass(Sense.MAX)])
    out = pm.run(model)

    # The C++ `forward` renamed the model; the built-in flipped the sense.
    assert out.model.name == "renamed:original"
    assert out.model.sense == Sense.MAX

    # Reversal dispatches back through both passes, including the C++ `backward`.
    solution = Solution([{}], counts=[1], obj_values=[0.0], env=out.model.environment)
    reversed_solution = out.record.backward(solution)
    assert reversed_solution is not None


def test_cxx_analysis_feeds_cxx_transform(plugin):
    """A C++ analysis result is consumed by a C++ transform, mediated by luna_model.

    The analysis runs first and deposits the variable count; the transform ``require_analysis``-es it
    back across the C ABI and tags the model with it.
    """
    model = Model("m")
    vs = model.add_variables("x", 3)
    model.objective = vs.sum()

    pm = PassManager([plugin.VarCountAnalysis(), plugin.TagWithCountPass()])
    out = pm.run(model)

    assert out.model.name == "m:vars=3"


def test_cxx_analysis_dependency_is_enforced(plugin):
    """Without the analysis, the transform's required analysis is missing, so the run fails."""
    model = Model("m")
    vs = model.add_variables("x", 2)
    model.objective = vs.sum()

    pm = PassManager([plugin.TagWithCountPass()])
    with pytest.raises(Exception):
        pm.run(model)


def test_cxx_transform_consumes_builtin_analysis(plugin):
    """A C++ transform consumes a BUILT-IN (Rust) analysis via the shared luna_model .proto.

    luna-model's own ``MinValueForConstraintAnalysis`` produces a ``MinConstraintValues`` protobuf
    result; the C++ pass decodes it with the class generated from the wheel's bundled
    ``luna_model/plugin.proto`` (``luna_model.capi.get_proto()``) and tags the model with the number
    of constraint entries — the real cross-language path: a Rust analysis result read by a C++ plugin
    across the C ABI through one schema.
    """
    model = Model("m")
    x = model.add_variable("x", vtype=Vtype.BINARY)
    y = model.add_variable("y", vtype=Vtype.BINARY)
    model.objective = x + y
    model.add_constraint(x + y <= 1, "c1")
    model.add_constraint(x <= 1, "c2")

    pm = PassManager([MinValueForConstraintAnalysis(), plugin.TagWithMinConstraintPass()])
    out = pm.run(model)

    # Two constraints -> two entries in the built-in analysis result, decoded in C++.
    assert out.model.name == "min-constraints:2"


def test_cxx_composite_transforms_and_provides(plugin):
    """A composite pass's transform half runs: one C++ ``forward`` renames the model.

    ``RenameAndCountPass`` is the fused kind — a single pass that both mutates the model and produces
    an analysis result. Here we exercise the transform half in isolation.
    """
    model = Model("m")
    vs = model.add_variables("x", 3)
    model.objective = vs.sum()

    pm = PassManager([plugin.RenameAndCountPass()])
    out = pm.run(model)

    # The composite's `forward` renamed the model (prefixing the original name).
    assert out.model.name == "renamed-and-counted:m"


def test_cxx_composite_result_feeds_consumer(plugin):
    """A composite's *analysis* half feeds the existing consumer, mediated by luna_model.

    ``RenameAndCountPass`` publishes its variable count under the same slot ``VarCountAnalysis`` uses,
    so ``TagWithCountPass`` reads it back across the C ABI exactly as before — proving the composite
    is a drop-in producer of the shared contract.
    """
    model = Model("m")
    vs = model.add_variables("x", 3)
    model.objective = vs.sum()

    pm = PassManager([plugin.RenameAndCountPass(), plugin.TagWithCountPass()])
    out = pm.run(model)

    # The composite renamed the model to "renamed-and-counted:m", then the consumer appended the
    # count the composite's analysis half published (3 variables).
    assert out.model.name == "renamed-and-counted:m:vars=3"


def test_cxx_meta_analysis_feeds_consumer(plugin):
    """A C++ meta-analysis introspects the following steps; its count feeds a consumer transform.

    ``CountFollowingMeta`` reads no model — it counts the pipeline steps that follow it (the view
    starts at the meta pass's OWN position, so index 0 is the meta pass itself), reusing the
    ``VariableCount`` message and publishing under the same slot ``VarCountAnalysis`` uses, so the
    existing ``TagWithCountPass`` consumer reads it back unchanged. The view here is
    ``[CountFollowingMeta, TagWithCountPass]`` — two steps, the first being the meta pass — so the
    consumer sees ``count == 2``.
    """
    model = Model("m")
    vs = model.add_variables("x", 3)
    model.objective = vs.sum()

    pm = PassManager([plugin.CountFollowingMeta(), plugin.TagWithCountPass()])
    out = pm.run(model)

    assert out.model.name == "m:vars=2"


def test_cxx_control_flow_plan_runs_chosen_steps(plugin):
    """A C++ control-flow pass builds a plan of steps at run time, which the engine then runs.

    ``PickBranchPass`` reads the model/context and returns a ``ControlFlowPlan`` holding one transform
    step — the existing rename. The engine executes that plan, so the model ends up renamed exactly
    as if ``RenamePass`` had run directly, proving the plan is built and its steps are driven.
    """
    model = Model("original")

    pm = PassManager([plugin.PickBranchPass()])
    out = pm.run(model)

    # The plan's single transform step renamed the model, just like RenamePass.forward does.
    assert out.model.name == "renamed:original"


def test_cxx_plugin_infeasible_raises_model_infeasible_error(plugin):
    """A plugin reporting the "infeasible" kind raises the same exception as a native pass."""
    from luna_model.errors import ModelInfeasibleError

    with pytest.raises(ModelInfeasibleError, match="cxxplugin::failing: no feasible point"):
        PassManager([plugin.FailingPass("infeasible", "no feasible point")]).run(Model("m"))


def test_cxx_plugin_error_kind_is_the_cause_of_transform_error(plugin):
    """Any other kind raises TransformError, with the kind's own exception as its cause."""
    from luna_model.errors import ModelVtypeError, TransformError

    with pytest.raises(TransformError, match="spins are not supported") as raised:
        PassManager([plugin.FailingPass("vtype", "spins are not supported")]).run(Model("m"))
    assert isinstance(raised.value.__cause__, ModelVtypeError)
    assert "spins are not supported" in str(raised.value.__cause__)


def test_cxx_plugin_error_with_unknown_kind_has_no_cause(plugin):
    from luna_model.errors import TransformError

    with pytest.raises(TransformError, match="custom failure") as raised:
        PassManager([plugin.FailingPass("cxxplugin::custom", "custom failure")]).run(Model("m"))
    assert raised.value.__cause__ is None
