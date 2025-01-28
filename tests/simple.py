from aq_models import Variable

# from aq_models import Variable, Model, Vtype, Expression

v = Variable("my_variable")
v2 = Variable("my_other")
v3 = Variable("my_third")

e = v * 1
print(f"{e=}")
e2 = v2 + 2
print(f"{e2=}")
e3 = v3 - 2.0
print(f"{e3=}")
ep3 = e + 3
print(f"e + 3 = {ep3=}")
e2p2 = e2 + 2
print(f"e2 + 2 = {e2p2=}")
# print(f"{type(e2)=}")
ee2 = e + e2
print(f"e + e2 = {ee2=}")

e += e2
print(f"e + e2 = {e=}")


# e = v + 4.0
# print(f"add scaler to variable: {e}")
# e2 = v * 2.0
# print(f"mul scaler to variable: {e2}")
# # linear: v * 2.0
# # constant: + 4.0
#
# print("-----------")
#
# m = Model("my_model")
# print(m)
# m += 2.0
# m *= 3.0
# m += 5.0 * 2.0 + 3.0
# print(m)
#
# print("-----------")
#
# m = Model("my_negative_model")
# print(m)
# m -= 2.0
# m *= 3.0
# m -= 5.0 * 2.0 + 3.0
# print(m)
#
#
# # x, y = m.add_vars(["x", "y"], vtype=Vtype.Real)
# # m += ((2 * x) + (5 * y)) + (x * y)
#
# # x = m.add_var("x", Vtype.Real)
# # y = m.add_var("y", Vtype.Real)
#
#
# # Expression {
# #     linear: 2 * x + 5 * y,
# #     quadratic: x * y,
# #     constant: 0,
# #     higher_order: {}
# # }
#
# # print("variable name of x =", x.name)


# Is this possible?
# with Environment() as env:
#     v = Variable("a", env)
