from aq_models import MatrixTranslator
import numpy as np
import scipy.sparse as sp
import time


rep = 7
# sizes = [100, 200, 400, 600, 800, 1000, 2000, 4000]
sizes = [100, 200, 400, 600, 800, 1000, 2000]
print("| Size | Density | AqM RUST |")
print("|===========================|")
for s in sizes:
    for d in [0.1, 0.5, 1]:
        timings_a = []
        eq = True
        for _ in range(rep):
            qubo = sp.random(s, s, d).todense().A
            qubo = qubo + qubo.T

            a = time.perf_counter()
            m1 = MatrixTranslator.to_model(qubo)
            b = time.perf_counter()
            timings_a.append(b - a)

        ta = np.mean(timings_a)
        print(f"| {s:4d} |    {int(d*100):3d}% | {ta:7.4f}s |")
    print("|------+---------+----------|")

