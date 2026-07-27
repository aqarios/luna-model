"""End-to-end test of a Python-wrapped plugin extension, driven entirely from Python.

The user imports the plugin, registers it, and builds a ``PassManager`` mixing the plugin pass with
a built-in pass — never touching Rust, C, or the descriptor. Reversal is exercised too.
"""

import importlib

import pytest

import luna_model
from luna_model import Model, Sense, Solution, Vtype
from luna_model.transformation import AnalysisPass, PassManager
from luna_model.transformation.passes import ChangeSensePass, MinValueForConstraintAnalysis

# Typed protobuf class generated from `protos/variable_count.proto` by conftest (via `protoc`),
# exactly as a real plugin author would generate + import theirs.
from variable_count_pb2 import VariableCount


@pytest.fixture(scope="session")
def plugin():
    module = importlib.import_module("luna_plugin_ext")
    # The one registration call the user makes; installs the plugin's reversal handlers.
    luna_model.register(module)
    return module


class PyVariableCount(AnalysisPass[VariableCount]):
    """A pure-Python analysis whose result is a shareable protobuf payload, so a plugin can consume
    it. Provides the same name/type-tag the Rust plugin transform decodes into."""

    PROVIDES = "luna_plugin_ext::variable-count"

    def name(self) -> str:
        return "py-variable-count"

    @classmethod
    def result_type_tag(cls) -> str:
        return "luna_plugin_ext::variable-count"

    def run(self, model, ctx) -> VariableCount:
        return VariableCount(count=model.num_variables)


def test_plugin_forward_and_reverse_from_python(plugin):
    model = Model("original")

    # A C-boundary plugin pass and a built-in pass, mixed in one manager.
    pm = PassManager([plugin.RenamePass(), ChangeSensePass(Sense.MAX)])
    out = pm.run(model)

    # The plugin's `forward` renamed the model; the built-in flipped the sense.
    assert out.model.name == "renamed-by-py-plugin"
    assert out.model.sense == Sense.MAX

    # Reversal dispatches back through both passes, including the plugin's C `backward`.
    solution = Solution([{}], counts=[1], obj_values=[0.0], env=out.model.environment)
    reversed_solution = out.record.backward(solution)
    assert reversed_solution is not None


def test_plugin_analysis_feeds_plugin_transform(plugin):
    """plugin A's analysis result is consumed by plugin B's transform, mediated by luna_model.

    Both are C-boundary (Rust-backed) plugin passes mixed in one ``PassManager``: the analysis runs
    first and deposits its result; the transform ``require_analysis``-es it back across the C-ABI.
    """
    model = Model("m")
    vs = model.add_variables("x", 3)
    model.objective = vs.sum()

    pm = PassManager([plugin.CountVariablesAnalysis(), plugin.TagWithVariableCount()])
    out = pm.run(model)

    # The transform tagged the model with the count plugin A computed (3 variables).
    assert out.model.name == "vars=3"


def test_plugin_analysis_dependency_is_enforced(plugin):
    """Without plugin A's analysis, plugin B's required analysis is missing, so the run fails."""
    model = Model("m")
    vs = model.add_variables("x", 2)
    model.objective = vs.sum()

    pm = PassManager([plugin.TagWithVariableCount()])
    with pytest.raises(Exception):
        pm.run(model)


def test_pure_python_analysis_consumed_by_plugin(plugin):
    """A *pure-Python* analysis produces a protobuf result that a Rust plugin transform consumes.

    Proves the producer side is language-agnostic: the pure-Python result is serialized via
    ``SerializeToString`` and read back across the C-ABI by the plugin's ``require_analysis``.
    """
    model = Model("m")
    vs = model.add_variables("x", 3)
    model.objective = vs.sum()

    pm = PassManager([PyVariableCount(), plugin.TagWithVariableCount()])
    out = pm.run(model)

    assert out.model.name == "vars=3"


def test_plugin_transform_consumes_builtin_analysis(plugin):
    """A Rust plugin transform consumes a BUILT-IN luna-model analysis via the shared .proto.

    The twin of the C++ test: luna-model's own ``MinValueForConstraintAnalysis`` produces a
    ``MinConstraintValues`` result, which the plugin's ``TagWithMinConstraint`` reads back across the
    C-ABI *by type* (the tag is derived from the result type, not passed) and tags the model with the
    number of constraint entries.
    """
    model = Model("m")
    x = model.add_variable("x", vtype=Vtype.BINARY)
    y = model.add_variable("y", vtype=Vtype.BINARY)
    model.objective = x + y
    model.add_constraint(x + y <= 1, "c1")
    model.add_constraint(x <= 1, "c2")

    pm = PassManager([MinValueForConstraintAnalysis(), plugin.TagWithMinConstraint()])
    out = pm.run(model)

    assert out.model.name == "min-constraints:2"
