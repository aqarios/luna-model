import numpy as np
import pytest
from luna_model.translator import AwsTranslator

from .common import do_checks  # type: ignore[reportMissingImports]


@pytest.fixture()
def aws_solution() -> dict[str, np.typing.NDArray]:
    return {
        "samples": np.array(
            [
                [0, 1, -1, +1, 4, 3],
            ]
        ),
        "energies": np.array([3.14]),
    }


def test_aws_sol_with_substituted_model(aws_solution: dict[str, np.typing.NDArray]):
    do_checks(AwsTranslator, aws_solution)
