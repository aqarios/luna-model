from aqmodels.translator import LpTranslator

in_file = """
\\ LP format example

Maximize
  x + y + z
  + x + y + z +
  x * x
Subject To
  c0: x + y = 1
  c1: x + 5 y + 2 z <= 10
  qc0: x + y + [ x ^ 2 - 2 x * y + 3 y ^ 2 ] <= 5
Bounds
  0 <= x <= 5
  z >= 2
Generals
  x y z
End

"""

model = LpTranslator.to_model(in_file)
print("-----")
print(model)
print("-----")
out_file = LpTranslator.from_model(model)
print(out_file)
model2 = LpTranslator.to_model(out_file)
print("-----")
print(model)
print("-----")
