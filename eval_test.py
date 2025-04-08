import dimod
import numpy as np

from random import Random
from aqmodels import (
    Environment,
    SampleSetTranslator,
    Timer,
    Variable,
    Vtype,
    SolutionCreationError,
    Model,
)
from dimod import BinaryQuadraticModel, SampleSet, Vartype, as_samples
from dwave.samplers import SimulatedAnnealingSampler


def random_int(rand: Random):
    return rand.randint(0, 2**16 - 1)


def generate_bqms(
    n_models: int, rand: Random, n_vars_max: int = 100
) -> list[BinaryQuadraticModel]:
    out = []
    for _ in range(n_models):
        n_vars = rand.randint(3, n_vars_max)
        density = rand.random() * (1 - 1 / n_vars)
        num_interactions = int(density * n_vars**2 / 2)
        vartype = Vartype.BINARY if rand.randint(0, 1) == 0 else Vartype.SPIN
        bqm = dimod.generators.gnm_random_bqm(
            [f"x{i}" for i in range(n_vars)],
            num_interactions,
            vartype,
            random_state=random_int(rand),
        )
        out.append(bqm)
    return out


rand = Random(42)
bqms = generate_bqms(1, rand, n_vars_max=5)
print(bqms)

bqm = bqms[0]
bqm_np = bqm.to_numpy_vectors()
print(bqm_np)

print(bqm.to_numpy_matrix(list(range(bqm.num_variables))))

Q = np.zeros(shape=(bqm.num_variables, bqm.num_variables))
for i, b in enumerate(bqm_np.linear_biases):
    Q[i, i] = b

# for i, b in enumerate(bqm_np.quadratic):
#     Q[i, i] = b

print(Q)
