from typing import IO
from tqdm import tqdm  # type: ignore

from aq_models import Vtype, MatrixTranslator

from benchmarks.serialization.utils import (
    get_serialized_size_bytes,
    get_serialized_size_mb,
    serialize_aqm,
)
from benchmarks.setting import SIZES
from benchmarks.utils import BenchResult, format_result_aqm, make_qubo


def bench_serialize_aqm_xl(file: IO | None):
    result = BenchResult("Serialized AQM Size")
    result.suffix = "MB"

    sizes = [*SIZES, 4000, 8000, 16000]
    densities = [1.0]

    for size in tqdm(sizes, desc="Num. Variables", leave=False):
        aqm_for_size = []
        for density in densities:
            qubo = make_qubo(size, density)

            aqm = MatrixTranslator.to_model(qubo, vtype=Vtype.Binary)
            serialized_aqm = serialize_aqm(aqm)
            aqm_for_size.append(get_serialized_size_bytes(serialized_aqm))

        result.aqmodels.append(aqm_for_size)  # type: ignore

    # result.meta.extend([sizes, densities])
    # result.meta_labels.extend(["Size", "Density"])

    format_result_aqm(result, sizes, densities, file=file)
