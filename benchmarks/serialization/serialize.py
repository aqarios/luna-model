from typing import IO

import dimod
from tqdm import tqdm  # type: ignore

from aq_models import MatrixTranslator, Model, Vtype
from benchmarks.serialization.utils import (
    get_serialized_size_bytes,
    serialize_aqm,
    serialize_cqm,
)
from benchmarks.setting import DENSITIES, REPETITIONS, SIZES
from benchmarks.utils import BenchResult, format_result, make_qubo, timeit


@timeit(REPETITIONS)
def _serialize_aqm(model: Model):
    serialize_aqm(model)


@timeit(REPETITIONS)
def _serialize_dmd(model: dimod.ConstrainedQuadraticModel):
    serialize_cqm(model)


def bench_serialize_cqm(file: IO | None):
    result = BenchResult("Serialize Binary Quadratic Model")

    for size in tqdm(SIZES, desc="Num. Variables", leave=False):
        aqm_for_size = []
        dmd_for_size = []
        for density in tqdm(DENSITIES, desc="Density", leave=False):
            qubo = make_qubo(size, density)

            aqm = MatrixTranslator.to_model(qubo, vtype=Vtype.Binary)
            dmd = dimod.ConstrainedQuadraticModel.from_cqm(
                dimod.BinaryQuadraticModel(qubo, "BINARY")
            )

            aqm_for_size.append(_serialize_aqm(aqm))
            dmd_for_size.append(_serialize_dmd(dmd))

        result.aqmodels.append(aqm_for_size)  # type: ignore
        result.dimod.append(dmd_for_size)  # type: ignore

    format_result(result, file=file)


def bench_serialize_cqm_size(file: IO | None):
    result = BenchResult("Size of Serialized Binary Quadratic Model")
    result.suffix = "MB"

    for size in tqdm(SIZES, desc="Num. Variables", leave=False):
        aqm_for_size = []
        dmd_for_size = []
        for density in tqdm(DENSITIES, desc="Density", leave=False):
            qubo = make_qubo(size, density)

            aqm = MatrixTranslator.to_model(qubo, vtype=Vtype.Binary)
            dmd = dimod.ConstrainedQuadraticModel.from_cqm(
                dimod.BinaryQuadraticModel(qubo, "BINARY")
            )

            serialized_aqm = serialize_aqm(aqm)
            serialized_dmd = serialize_cqm(dmd)

            aqm_for_size.append(get_serialized_size_bytes(serialized_aqm))
            dmd_for_size.append(get_serialized_size_bytes(serialized_dmd))

        result.aqmodels.append(aqm_for_size)  # type: ignore
        result.dimod.append(dmd_for_size)  # type: ignore

    format_result(result, file=file)
