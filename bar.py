from aqmodels import Model, Vtype

model = Model()
x = model.add_variable("x")
s = model.add_variable("s", vtype=Vtype.Spin)

model.objective = x - s


rep = model.add_variable("b")
print("*** before substitute I")
model.substitute(s, rep)
print("*** after substitute I")

print(s)
print(s.name)
print(s.vtype)

new = model.add_variable("s")
print("*** before substitute II")
model.substitute(rep, new)
print("*** after substitute II")
