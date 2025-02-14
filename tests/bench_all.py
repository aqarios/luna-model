from aq_models import MatrixTranslator
from aq_models import MatrixTranslatorV2
import numpy as np
import scipy.sparse as sp
import time

from memory_profiler import profile

import dimod

# @profile
def ours(qubo) -> float:
    a = time.perf_counter()
    _ = MatrixTranslator.to_model(qubo)
    b = time.perf_counter()
    return b - a

def oursv2(qubo) -> float:
    a = time.perf_counter()
    _ = MatrixTranslatorV2.to_model(qubo)
    b = time.perf_counter()
    return b - a

# @profile
def dmod(qubo) -> float:
    a = time.perf_counter()
    _ = dimod.BinaryQuadraticModel(qubo, "BINARY")
    b = time.perf_counter()
    return b - a



def bench(s: int, d: float, rep: int):
    timings_a = []
    timings_b = []
    timings_c = []
    # eq = True
    for _ in range(rep):
        qubo = sp.random(s, s, d).todense().A

        # st = time.perf_counter()
        # qubo = qubo + qubo.T
        # se = time.perf_counter()

        # tracemalloc.start()

        # a = time.perf_counter()
        # _ = MatrixTranslator.to_model(qubo)
        # b = time.perf_counter()
        t = ours(qubo)
        timings_a.append(t)

        t = oursv2(qubo)
        timings_b.append(t)

        # print("OURS", tracemalloc.get_traced_memory())

        # tracemalloc.stop()
        # tracemalloc.start()

        t = dmod(qubo)
        timings_c.append(t)

        # print("DIMOD", tracemalloc.get_traced_memory())

    ta = np.mean(timings_a)
    tb = np.mean(timings_b)
    tc = np.mean(timings_c)
    return ta, tb, tc


rep = 7
sizes = [100, 200, 400, 600, 800, 1000, 2000]
print("| Size | Density | AqM RUST |  AqM V2  |  Dimod   |")
print("|=================================================|")
for s in sizes:
    for d in [0.1, 0.5, 1]:
        ta, tb, tc = bench(s, d, rep)
        print(f"| {s:4d} |    {int(d*100):3d}% | {ta:7.4f}s | {tb:7.4f}s | {tc:7.4f}s |")
    print("|------+---------+----------+----------+----------|")
