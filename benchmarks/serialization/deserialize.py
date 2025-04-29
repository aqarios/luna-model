import json
from typing import IO, Any

from aqmodels import Model, Vtype, QuboTranslator
import dimod
from tqdm import tqdm  # type: ignore

from benchmarks.serialization.utils import serialize_aqm, serialize_bqm
from benchmarks.setting import DENSITIES, REPETITIONS, SIZES
from benchmarks.utils import BenchResult, format_result, make_qubo, timeit


@timeit(REPETITIONS)
def _deserialize_aqm(data: bytes):
    _ = Model.deserialize(data)


@timeit(REPETITIONS)
def _deserialize_dmd(data: Any):
    data_dict = json.loads(data)
    _ = dimod.BinaryQuadraticModel.from_serializable(data_dict)


def bench_deserialize_bqm(file: IO | None):
    result = BenchResult("Deserialize Binary Quadratic Model")

    for size in tqdm(SIZES, desc="Num. Variables", leave=False):
        aqm_for_size = []
        dmd_for_size = []

        for density in tqdm(DENSITIES, desc="Density", leave=False):
            qubo = make_qubo(size, density)

            aqm = QuboTranslator.to_aq(qubo, vtype=Vtype.Binary)
            dmd = dimod.BinaryQuadraticModel(qubo, "BINARY")

            ser_aqm = serialize_aqm(aqm)
            aqm_for_size.append(_deserialize_aqm(ser_aqm))

            ser_dmd = serialize_bqm(dmd)
            dmd_for_size.append(_deserialize_dmd(ser_dmd))

        result.aqmodels.append(aqm_for_size)  # type: ignore
        result.dimod.append(dmd_for_size)  # type: ignore

    # result.meta.extend([SIZES, DENSITIES])
    # result.meta_labels.extend(["Size", "Density"])

    format_result(result, file=file)
