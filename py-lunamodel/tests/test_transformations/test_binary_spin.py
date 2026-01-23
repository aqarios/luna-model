# import pytest
# from luna_modelimport Model, Vtype
# from luna_model.transformations import BinarySpinPass, PassManager
# 
# 
# @pytest.fixture()
# def default_pass_manager():
#     return PassManager([BinarySpinPass(Vtype.Spin, None)])
# 
# 
# def test_target_variable_exists(default_pass_manager: PassManager):
#     model = Model()
#     model.add_variable("s_x", vtype=Vtype.Spin)
#     model.add_variable("x", vtype=Vtype.Binary)
# 
#     ir = default_pass_manager.run(model)
#     assert len(ir.model.variables()) == 2
#     assert ir.model.variables()[0].name == "s_x"
#     assert ir.model.variables()[1].name == "s_x_Uk"
# 
# 
# def test_target_variable_exists_many(default_pass_manager: PassManager):
#     model = Model()
#     for i in range(100):
#         model.add_variable(f"s_x_{i}", vtype=Vtype.Spin)
#         model.add_variable(f"x_{i}", vtype=Vtype.Binary)
# 
#     ir = default_pass_manager.run(model)
# 
#     res = ir.cache["binary-spin"]
# 
#     assert all(x.count("_") == 3 for x in res.map.values())
#     assert len(ir.model.variables()) == 200
