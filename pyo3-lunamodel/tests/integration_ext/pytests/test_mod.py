import pytest
from luna_model import Environment, Model, Solution, Variable, Vtype


def test_model_is_shared_and_gil_independent(ext):
    model = Model("before")
    held = ext.HeldModel(model)

    held.set_name("from-rust")
    assert model.name == "from-rust"

    model.name = "from-python"
    assert held.name() == "from-python"

    del model
    assert held.name() == "from-python"


def test_model_created_by_extension_transfers_through_c_handle(ext):
    model = ext.new_model("from-extension")
    assert isinstance(model, Model)
    assert model.name == "from-extension"


def test_model_c_capsule_is_consumable_once():
    model = Model("capsule")
    capsule = model._m._to_c_capsule()

    type(model._m)._from_c_capsule(capsule)
    with pytest.raises(ValueError, match="already consumed"):
        type(model._m)._from_c_capsule(capsule)


def test_solution_is_shared_and_gil_independent(ext):
    env = Environment()
    Variable("x", env=env)
    solution = Solution([{"x": 1}], counts=[1], env=env)
    held = ext.HeldSolution(solution)

    held.add_real("y", [2.0])
    assert solution.variable_names == ["x", "y"]

    solution.add_var("z", [3.0], Vtype.REAL)
    assert held.variable_names() == ["x", "y", "z"]
