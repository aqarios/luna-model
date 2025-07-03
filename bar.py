from aqmodels import Model, Vtype

original = Model()
x = original.add_variable("x")
y = original.add_variable("y")
z = original.add_variable("z")
s = original.add_variable("s", vtype=Vtype.Spin)

original.objective = 2 * x - 3 * s + 4 * x * s + 5 * x * y * s

model = Model()
x = model.add_variable("x")
y = model.add_variable("y")
z = model.add_variable("z")
s = model.add_variable("s", vtype=Vtype.Spin)

model.objective = 2 * x - 3 * s + 4 * x * s + 5 * x * y * s


rep = model.add_variable("b")
print("*** before substitute I")
model.substitute(s, rep)
print("*** after substitute I")

print(model)

print(repr(model.environment))

new = model.add_variable("s", vtype=Vtype.Spin)
print("*** before substitute II")
model.substitute(rep, new)
print("*** after substitute II")

print(model)

print(original.equal_contents(model))
