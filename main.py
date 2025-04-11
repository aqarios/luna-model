from random import Random

import dimod
import numpy as np
from dimod import BinaryQuadraticModel
from dimod import Vartype

from pytests.test_core.utils import make_seed


def generate_bqms(
        n_models: int, rand: Random, n_vars_max: int = 100
) -> list[BinaryQuadraticModel]:
    out = []
    for _ in range(n_models):
        n_vars = 5
        density = 0.5
        num_interactions = int(density * n_vars ** 2 / 2)
        vartype = Vartype.BINARY if rand.randint(0, 1) == 0 else Vartype.SPIN
        bqm = dimod.generators.gnm_random_bqm(
            [f"x{i}" for i in range(n_vars)],
            num_interactions,
            vartype
        )
        out.append(bqm)
    return out


def main():
    rand = Random(make_seed())
    bqm = generate_bqms(1, rand)[0]

    bqm_ser = bqm.to_serializable()
    # print(bqm_ser)

    print(bqm.variables.to_serializable())

    bqm_np = bqm.to_numpy_vectors()
    # print(bqm_np)
    print(bqm_np.linear_biases, bqm_np.linear_biases.dtype)
    quad = bqm_np.quadratic
    print(quad.biases, quad.biases.dtype)
    print(quad.col_indices, quad.col_indices.dtype)
    print(quad.row_indices, quad.row_indices.dtype)
    print(bqm_np.offset)
    print(bqm.vartype, bqm.vartype.name)
    # print(bqm_np.labels)
    quadratic = (np.arange(0, 10), np.arange(1, 11), -np.ones(10))
    print(quadratic)

    BinaryQuadraticModel.from_numpy_vectors()


if __name__ == "__main__":
    main()

bqm_dict = {
    "type": "BinaryQuadraticModel",
    "version": {"bqm_schema": "3.0.0"},
    "use_bytes": False,
    "index_type": "int32",
    "bias_type": "float64",
    "num_variables": 5,
    "num_interactions": 6,
    "variable_labels": ["x0", "x1", "x2", "x3", "x4"],
    "variable_type": "SPIN",
    "offset": 0.4794129832092663,
    "info": {},
    "linear_biases": [
        0.7067441462557746,
        0.4099609056892407,
        0.035120698086226976,
        0.7086037561731827,
        0.8987605861196875,
    ],
    "quadratic_biases": [
        0.21607724082121438,
        0.0011395776252206558,
        0.15973043887098148,
        0.5959966346365454,
        0.40291538167911967,
        0.3013224394276319,
    ],
    "quadratic_head": [0, 0, 0, 0, 1, 1],
    "quadratic_tail": [1, 2, 3, 4, 2, 3],
}


def to_bqm(offset, linear, quad, rows, cols, vtype, vars):
    vartype = vtype.upper()
    bqm = BinaryQuadraticModel.from_numpy_vectors(
        linear, (rows, cols, quad), offset, vartype, variable_order=vars
    )
    return bqm


bqm = dimod.generators.gnm_random_bqm(5, 10, "BINARY")
