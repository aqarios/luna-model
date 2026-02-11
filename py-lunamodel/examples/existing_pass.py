from luna_model._lm import PyTransformationPass

from luna_model import Model, Sense, Variable
from luna_model.transformation.analysis import AnalysisCache
from luna_model.transformation.pass_manager import PassManager
from luna_model.transformation.passes import ChangeSensePass  # , MaxBiasAnalysis
from luna_model.transformation.passes.binary_spin import BinarySpinPass
from luna_model.transformation.transform import TransformationPass
from luna_model import Vtype

lm = Model("Model To transform")
lm.set_sense(sense=Sense.MAX)
with lm.environment:
    x = Variable("x", vtype=Vtype.SPIN)
    y = Variable("y", vtype=Vtype.SPIN)
lm.objective = x * 20 * y

print(lm)

# m = MaxBiasAnalysis()
c = ChangeSensePass(Sense.MIN)
print(type(c))
print(isinstance(c, TransformationPass))
print(isinstance(c, PyTransformationPass))

print(c.run(lm, AnalysisCache()))

# pm = PassManager([m, c])
pm = PassManager([c, BinarySpinPass(vtype=Vtype.BINARY)])

print("=== PassManager ===")  # noqa: T201
print(pm)  # noqa: T201
ir = pm.run(lm)

print("=== Model Before Transformation ===")  # noqa: T201
print(lm)  # noqa: T201

print("=== Model After Transformation ===")  # noqa: T201
print(ir.model)  # noqa: T201
