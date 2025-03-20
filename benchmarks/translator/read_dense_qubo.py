from typing import IO
from numpy.typing import NDArray
from tqdm import tqdm  # type: ignore[import-untyped]

from aq_models import MatrixTranslator, Vtype
import dimod

from benchmarks.setting import DENSITIES, REPETITIONS, SIZES
from benchmarks.utils import BenchResult, format_result, make_qubo, timeit


@timeit(REPETITIONS)
def _aqm(qubo: NDArray):
    _ = MatrixTranslator.to_model(qubo, vtype=Vtype.Binary)


@timeit(REPETITIONS)
def _dmd(qubo: NDArray):
    _ = dimod.BinaryQuadraticModel(qubo, "BINARY")


def bench_read_dense_qubo(file: IO | None):
    result = BenchResult("Read Dense Qubo to Model")

    for size in tqdm(SIZES, desc="Num. Variables", leave=False):
        aqm_for_size = []
        dmd_for_size = []
        for density in tqdm(DENSITIES, desc="Density", leave=False):
            qubo = make_qubo(size, density)
            aqm_for_size.append(_aqm(qubo))
            dmd_for_size.append(_dmd(qubo))

        result.aqmodels.append(aqm_for_size)  # type: ignore
        result.dimod.append(dmd_for_size)  # type: ignore

    # result.meta.extend([SIZES, DENSITIES])
    # result.meta_labels.extend(["Size", "Density"])

    format_result(result, file)
