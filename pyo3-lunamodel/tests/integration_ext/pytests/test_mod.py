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


def test_environment_is_shared_and_gil_independent(ext):
    environment = Environment()
    held = ext.HeldEnvironment(environment)

    held.insert_real("from_rust")
    assert environment.num_variables == 1

    Variable("from_python", env=environment)
    assert held.len() == 2


def test_environment_returned_from_model_retains_identity(ext):
    model = Model("environment")
    environment = ext.model_environment(model)

    Variable("shared", env=environment)
    assert model.environment.num_variables == 1
    assert model.environment.id == environment.id


def test_model_objective_is_a_live_gil_independent_view(ext):
    model = Model("objective")
    held = ext.HeldExpression(model.objective)

    held.add_bias(2.0)
    assert model.objective.get_offset() == 2.0

    model.objective = 7.0
    assert held.offset() == 7.0

    del model
    assert held.offset() == 7.0


def test_model_constraints_are_a_live_gil_independent_view(ext):
    model = Model("constraints")
    x = model.add_variable("x")
    held = ext.HeldConstraintCollection(model.constraints)

    model.add_constraint(x <= 1, "upper")
    assert held.len() == 1

    held.add_constraint(x >= 0, "lower")
    assert model.num_constraints == 2

    model.constraints = type(model.constraints)()
    assert held.len() == 0


def test_constraint_is_shared_and_gil_independent(ext):
    environment = Environment()
    x = Variable("x", env=environment)
    constraint = x <= 1
    held = ext.HeldConstraint(constraint)

    held.set_name("from-rust")
    assert constraint.name == "from-rust"

    constraint.name = "from-python"
    assert held.name() == "from-python"
