from pathlib import Path
from aqmodels.translator import LpTranslator


model = LpTranslator.to_model(Path("./lp_file.lp"))
print(model)
