import pytest

from luna_model import Model, Vtype
from luna_model.transformation import PassManager
from luna_model.transformation.passes import BinarySpinPass


@pytest.fixture()
def default_pass_manager():
    return PassManager([BinarySpinPass(Vtype.SPIN, None)])


# def test_target_variable_exists(default_pass_manager: PassManager):
#     model = Model()
#     model.add_variable("s_x", vtype=Vtype.SPIN)
#     model.add_variable("x", vtype=Vtype.BINARY)
#
#     ir = default_pass_manager.run(model)
#     assert len(ir.model.variables()) == 2
#     assert ir.model.variables()[0].name == "s_x"
#     assert ir.model.variables()[1].name == "s_x_Uk"
# def test_target_variable_exists_many(default_pass_manager: PassManager):
#     model = Model()
#     for i in range(100):
#         model.add_variable(f"s_x_{i}", vtype=Vtype.SPIN)
#         model.add_variable(f"x_{i}", vtype=Vtype.BINARY)
#
#     ir = default_pass_manager.run(model)
#     res = ir.cache["binary-spin"]
#     assert all(x.count("_") == 3 for x in res.map.values())
#     assert len(ir.model.variables()) == 200
# NOTE: The behaviour changed in the above tests. There are no variables contributing to the model. So nothing is done.
# This is fixed below.
def test_target_variable_exists(default_pass_manager: PassManager):
    model = Model()
    sx = model.add_variable("s_x", vtype=Vtype.SPIN)
    x = model.add_variable("x", vtype=Vtype.BINARY)
    model.objective = sx + x

    ir = default_pass_manager.run(model)
    assert len(ir.model.variables()) == 2
    assert ir.model.variables()[0].name == "s_x"
    assert ir.model.variables()[1].name == "s_x_Uk"


def test_target_variable_exists_many(default_pass_manager: PassManager):
    model = Model()
    for i in range(100):
        a = model.add_variable(f"s_x_{i}", vtype=Vtype.SPIN)
        b = model.add_variable(f"x_{i}", vtype=Vtype.BINARY)
        model.objective += a + b

    ir = default_pass_manager.run(model)
    res = ir.cache["binary-spin"]
    assert all(x.count("_") == 3 for x in res.map.values())
    assert len(ir.model.variables()) == 200
