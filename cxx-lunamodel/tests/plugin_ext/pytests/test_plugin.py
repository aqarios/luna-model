"""End-to-end test of a C++-authored plugin, driven entirely from Python.

The passes are written in C++ with the header-only ``lm::plugin`` SDK and exposed via nanobind
(``luna_cxx_plugin_ext``). The user imports the plugin, registers it, and builds a ``PassManager``
mixing C++ passes with a built-in one — never touching the C ABI or the descriptor. This mirrors the
pyo3 plugin test exactly, proving the cxx SDK is on par with the Rust one.
"""

import importlib

import pytest

import luna_model
from luna_model import Model, Sense, Solution
from luna_model.transformation import PassManager
from luna_model.transformation.passes import ChangeSensePass


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
