from aq_models import MatrixTranslator
import numpy as np
# import scipy.sparse as sp


# qubo = sp.random(4, 4, 0.5).todense().A
# qubo = qubo + qubo.T

print("---")
#   x0, x1, x2
# x0 0,  1,  0
# x1 1,  0,  1
# x2 0,  1,  0
qubo = np.array([
    [0, 1, 0],
    [1, 0, 1],
    [0, 1, 0],
], dtype=np.float64)

model = MatrixTranslator.to_model(qubo)
print(str(model))
print("should be", "2 * x_0 * x_1 + 2 * x_1 * x_2")


print("---")
#   x0, x1, x2
# x0 1,  0,  0
# x1 0,  2,  0
# x2 0,  0,  3
qubo = np.array([
    [1, 0, 0],
    [0, 2, 0],
    [0, 0, 3],
], dtype=np.float64)

model = MatrixTranslator.to_model(qubo)
print(str(model))
print("should be", "x_0 + 2 * x_1 + 3 * x_2")

print("---")
#   x0, x1, x2
# x0 1,  1,  0
# x1 1,  2,  0
# x2 0,  0,  3
qubo = np.array([
    [1, 1, 0],
    [1, 2, 0],
    [0, 0, 3],
], dtype=np.float64)

model = MatrixTranslator.to_model(qubo)
print(str(model))

