from typing import IO
from tqdm import tqdm  # type: ignore

from aq_models import Model, Vtype, MatrixTranslator
import dimod

from benchmarks.serialization.utils import (
    get_serialized_size_bytes,
    get_serialized_size_mb,
    serialize_aqm,
    serialize_bqm,
)
from benchmarks.setting import DENSITIES, REPETITIONS, SIZES
from benchmarks.utils import BenchResult, format_result, make_qubo, timeit


@timeit(REPETITIONS)
def _serialize_aqm(model: Model):
    serialize_aqm(model)


@timeit(REPETITIONS)
def _serialize_dmd(model: dimod.BinaryQuadraticModel):
    serialize_bqm(model)


def bench_serialize_bqm(file: IO | None):
    result = BenchResult("Serialize Binary Quadratic Model")

    for size in tqdm(SIZES, desc="Num. Variables", leave=False):
        aqm_for_size = []
        dmd_for_size = []
        for density in tqdm(DENSITIES, desc="Density", leave=False):
            qubo = make_qubo(size, density)

            aqm = MatrixTranslator.to_model(qubo, vtype=Vtype.Binary)
            dmd = dimod.BinaryQuadraticModel(qubo, "BINARY")

            aqm_for_size.append(_serialize_aqm(aqm))
            dmd_for_size.append(_serialize_dmd(dmd))

        result.aqmodels.append(aqm_for_size)  # type: ignore
        result.dimod.append(dmd_for_size)  # type: ignore

    format_result(result, file=file)


def bench_serialize_bqm_size(file: IO | None):
    result = BenchResult("Size of Serialized Binary Quadratic Model")
    result.suffix = "MB"
    # result.aqm_alt = "AQM (in MB)"
    # result.dimod_alt = "Dimod (in MB)"

    for size in tqdm(SIZES, desc="Num. Variables", leave=False):
        aqm_for_size = []
        dmd_for_size = []
        for density in tqdm(DENSITIES, desc="Density", leave=False):
            qubo = make_qubo(size, density)

            aqm = MatrixTranslator.to_model(qubo, vtype=Vtype.Binary)
            dmd = dimod.BinaryQuadraticModel(qubo, "BINARY")

            serialized_aqm = serialize_aqm(aqm)
            serialized_dmd = serialize_bqm(dmd)

            aqm_for_size.append(get_serialized_size_bytes(serialized_aqm))
            dmd_for_size.append(get_serialized_size_bytes(serialized_dmd))

        result.aqmodels.append(aqm_for_size)  # type: ignore
        result.dimod.append(dmd_for_size)  # type: ignore

    # result.meta.extend([SIZES, DENSITIES])
    # result.meta_labels.extend(["Size", "Density"])

    format_result(result, file=file)
