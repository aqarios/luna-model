import sys
from aq_models import MatrixTranslator
import numpy as np
import scipy.sparse as sp
import time


def bench(s: int, d: float, rep: int):
    timings_a = []
    # eq = True
    for _ in range(rep):
        qubo = sp.random(s, s, d).todense().A
        qubo = qubo + qubo.T

        a = time.perf_counter()
        _ = MatrixTranslator.to_model(qubo)
        b = time.perf_counter()
        timings_a.append(b - a)

    ta = np.mean(timings_a)
    return ta


rep = 1
# sizes = [100, 200, 400, 600, 800, 1000, 2000, 4000]
# sizes = [100, 200, 400, 600, 800, 1000, 2000]
# sizes = [100, 200, 400, 600]
# print("| Size | Density | AqM RUST |")
# print("|===========================|")
# for s in sizes:
#     for d in [0.1, 0.5, 1]:
#         ta = bench(s, d, rep)
#         print(f"| {s:4d} |    {int(d*100):3d}% | {ta:7.4f}s |")
#     print("|------+---------+----------|")

s = int(sys.argv[1])
d = float(sys.argv[2])
ta = bench(s, d, rep)
print(f"| {s:4d} |    {int(d*100):3d}% | {ta:7.4f}s |")
