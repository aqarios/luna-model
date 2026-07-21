"""End-to-end test of a Python-wrapped plugin extension, driven entirely from Python.

The user imports the plugin, registers it, and builds a ``PassManager`` mixing the plugin pass with
a built-in pass — never touching Rust, C, or the descriptor. Reversal is exercised too.
"""

import importlib

import pytest

import luna_model
from luna_model import Model, Sense, Solution
from luna_model.transformation import PassManager
from luna_model.transformation.passes import ChangeSensePass


@pytest.fixture(scope="session")
def plugin():
    module = importlib.import_module("luna_plugin_ext")
    # The one registration call the user makes; installs the plugin's reversal handlers.
    luna_model.register(module)
    return module


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
