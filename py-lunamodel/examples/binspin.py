from luna_model import Model, Vtype
from luna_model.transformation import PassManager
from luna_model.transformation.passes import BinarySpinPass

model = Model()
sx = model.add_variable("s_x", vtype=Vtype.SPIN)
x = model.add_variable("x", vtype=Vtype.BINARY)
model.objective = sx + x

pm = PassManager([BinarySpinPass(Vtype.SPIN, None)])
print(pm)
print("-" * 60)
print(model)
ir = pm.run(model)
print("-" * 60)
print(ir.model)
print("-" * 60)
print(ir.model.variables())

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
