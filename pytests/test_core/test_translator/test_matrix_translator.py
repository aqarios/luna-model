import pytest
from itertools import product

import numpy as np
import scipy.sparse as sp  # type: ignore[import-untyped]
from numpy.typing import NDArray

from aq_models import MatrixTranslator
from ..utils import make_seed


@pytest.fixture
def qubo(request) -> NDArray:
    size, density = request.param
    np.random.seed(make_seed())
    out = sp.random(size, size, density).todense()
    out += out.T
    return out


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense(qubo: NDArray):
    model = MatrixTranslator.to_model(qubo)
    back = MatrixTranslator.to_dense(model)
    assert np.allclose(qubo, back)
