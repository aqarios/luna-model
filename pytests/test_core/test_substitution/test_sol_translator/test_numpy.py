import pytest
import numpy as np
from aqmodels.translator import NumpyTranslator
from .common import do_checks  # type: ignore[reportMissingImports]


@pytest.fixture
def np_solution() -> tuple[np.typing.NDArray, np.typing.NDArray]:
    return (
        np.array(
            [
                [0, 1, -1, +1, 4, 3],
            ]
        ),
        np.array([3.14]),
    )


def test_dwave_sol_with_substituted_model(
    np_solution: tuple[np.typing.NDArray, np.typing.NDArray],
):
    do_checks(NumpyTranslator, np_solution)
