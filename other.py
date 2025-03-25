import aqmodels
import aqmodels as aqm


with aqm.Environment():
    v = aqmodels.Variable("v")


print(v)

from aqmodels.translator import MatrixTranslator


print(MatrixTranslator)

from aqmodels import translator

print(translator)

print(aqmodels.translator.MatrixTranslator)


# with aqm.Environment():
#     v = aqm.Variable("a")
#
#     np.count_nonzero
