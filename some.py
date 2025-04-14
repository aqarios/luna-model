from pathlib import Path
from aqmodels.translator import LpTranslator

in_file = Path("./lp_file.lp")
out_file = Path("./out.lp")

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
