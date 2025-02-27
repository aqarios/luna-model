from aq_models import MatrixTranslator

import numpy as np
import scipy.sparse as sp  # type: ignore[import-untyped]
import time

# from memory_profiler import profile

import dimod


# @profile
def ours(qubo) -> float:
    a = time.perf_counter()
    _ = MatrixTranslator.to_model(qubo)
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
    timings_c = []
    for _ in range(rep):
        qubo = sp.random(s, s, d).todense().A

        t = ours(qubo)
        timings_a.append(t)

        t = dmod(qubo)
        timings_c.append(t)

    return np.mean(timings_a), np.mean(timings_c)


rep = 7
sizes = [100, 200, 400, 600, 800, 1000, 2000]
print("| Size | Density | AqM RUST |  Dimod   |")
print("|======================================|")
for s in sizes:
    for d in [0.1, 0.5, 1]:
        ta, tb = bench(s, d, rep)
        print(f"| {s:4d} |    {int(d*100):3d}% | {ta:7.4f}s | {tb:7.4f}s |")
        # print(
        #     f"| {s:4d} |    {int(d*100):3d}% | {ta:7.4f}s | {tb:7.4f}s | {tc:7.4f}s |"
        # )
    print("|------+---------+----------+----------|")
