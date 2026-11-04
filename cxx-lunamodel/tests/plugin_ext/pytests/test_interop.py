"""End-to-end test of the Python<->C++ interop casters (luna_model/nanobind.hpp).

Plain C++ functions — not passes — receive and return luna_model Python objects directly, driven
from Python. This exercises the general interop membrane (the nanobind analog of rs-lunamodel's
FromPyObject/IntoPyObject): a luna_model.Model crosses into C++ as an lm::Model and back, via the
`_to_c_capsule` / `_from_c_capsule` transport. Importing the module binds the C API table, so no
`luna_model.register` is needed — these are ordinary functions, not plugins.
"""

import luna_cxx_plugin_ext as ext
from luna_model import Environment, Model, Variable, Vtype


def test_cxx_function_receives_python_model():
    model = Model("m")
    vs = model.add_variables("x", 3)
    model.objective = vs.sum()  # make the variables participate so num_variables counts them

    # The C++ function extracts the lm::Model from the passed luna_model.Model and reads it.
    assert ext.model_variable_count(model) == 3


def test_cxx_function_returns_python_model():
    # The C++ function builds an lm::Model and returns it as a luna_model.Model.
    model = ext.make_named_model("from-cxx")

    assert isinstance(model, Model)
    assert model.name == "from-cxx"


def test_cxx_roundtrips_environment():
    # Default-constructible wrapper: luna_model.Environment -> lm::Env -> luna_model.Environment.
    model = Model("m")
    out = ext.roundtrip_environment(model.environment)
    assert isinstance(out, Environment)


def test_cxx_roundtrips_variable():
    # Non-default-constructible wrapper (Variable): proves the optional-storage caster round-trips.
    model = Model("m")
    var = model.add_variable("x", vtype=Vtype.BINARY)
    out = ext.roundtrip_variable(var)
    assert isinstance(out, Variable)


def test_python_enum_to_c_enum():
    # A Python luna_model.Vtype converts to the C LmVtype via the enum caster.
    assert ext.vtype_value(Vtype.BINARY) == 0  # LmVtype_Binary


def test_c_enum_to_python_enum():
    # A C LmVtype converts back to the Python luna_model.Vtype.
    assert ext.binary_vtype() == Vtype.BINARY


def test_c_api_table_layout_matches_host():
    host_size, compiled_size, base_size, has_last_base_entry = ext.c_api_table_layout()
    # The host's table size (Rust) equals the C header's sizeof: one layout on both sides.
    assert host_size == compiled_size
    assert base_size <= host_size
    assert has_last_base_entry
