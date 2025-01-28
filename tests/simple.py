from aq_models import Variable

# from aq_models import Variable, Model, Vtype, Expression

print("Creating two variables (x and y)")
x = Variable("x")
y = Variable("y")
print(x)
print(y)

print("Adding a scaler to the 'x' variable")
expr_x = x + 1
print(expr_x)
expr_xr = 1 + x
print(expr_xr)
print(expr_x == expr_xr, expr_xr == expr_x)

print("Multiplying a scaler with the 'y' variable")
expr_y = y * 1
print(expr_y)
expr_yr = 1 * y
print(expr_yr)
print(expr_y == expr_yr, expr_yr == expr_y)

print("Adding two expressions (resulting in a new expression)")
expr_xy = expr_x + expr_y
print(expr_xy)
expr_yx = expr_y + expr_x
print(expr_yx)
print(expr_xy == expr_yx, expr_yx == expr_xy)

print("Adding one expressions to the other (x += y)")
expr_x += expr_y
print(expr_x)
print(expr_x == expr_xy, expr_xy == expr_x)

print("Substracting one expressions from the other (x -= y)")
expr_x -= expr_y
print(expr_x)
print(expr_x == expr_xr, expr_xr == expr_x)

print("Adding one expressions to the other (y += x)")
expr_y += expr_x
print(expr_y)
print(expr_y == expr_yx, expr_yx == expr_y)

print("Substracting one expressions from the other (y -= x)")
expr_y -= expr_x
print(expr_y)
print(expr_y == expr_yr, expr_yr == expr_y)

print("Multiplying the expressions")
expr_xy = expr_x * expr_y


# print(f"{e=}")
# e2 = v2 + 2
# print(f"{e2=}")
# e3 = v3 - 2.0
# print(f"{e3=}")
# ep3 = e + 3
# print(f"e + 3 = {ep3=}")
# e2p2 = e2 + 2
# print(f"e2 + 2 = {e2p2=}")
# # print(f"{type(e2)=}")
# ee2 = e + e2
# print(f"e + e2 = {ee2=}")
#
# e += e2
# print(f"e + e2 = {e=}")
#
#
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
