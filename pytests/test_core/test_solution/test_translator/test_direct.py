from pathlib import Path
from luna_model._core import TranslationTarget
import numpy as np
from luna_model import Model, Solution, Timer
from luna_model.translator import AwsTranslator
from numpy.typing import NDArray
from pyscipopt import Model as ScipModel

from pytests.test_core.test_solution.test_translator.test_ibm_translator import (
    compute_result,
    controlled_qp,
)
from pytests.test_core.utils import make_seed, random, random_int

from .fixtures import *  # noqa: F403
from ..test_from_dict import vars


def make_scip_model(zib_model):
    lp_str = zib_model.to(TranslationTarget.Lp)
    lp_filepath = Path(__file__).parent / "model.lp"
    with open(lp_filepath, "w") as f:
        f.write(lp_str)

    scip_model = ScipModel()
    scip_model.hideOutput()
    scip_model.readProblem(lp_filepath)
    scip_model.optimize()
    return scip_model


def test_solfrom_aws(zib_model, aws_result, np_result, dwave_result):
    rand = Random(make_seed())

    print("scip")
    _ = Solution.from_(make_scip_model(zib_model))

    print("qctrl")
    sample_len = rand.randint(2, 5)
    num_samples = rand.randint(1, max(sample_len // 2, 1))
    fake_result, _ = fake_qctrl_result(
        rand, sample_len, random(random_int(rand)), num_samples
    )
    _ = Solution.from_(fake_result)

    print("aws")
    _ = Solution.from_(aws_result)

    print("np")
    res, energies = np_result
    _ = Solution.from_(res, energies=energies)

    print("ibm")
    qp = controlled_qp()
    res = compute_result(qp)
    _ = Solution.from_(res, quadratic_program=qp)

    print("dwave")
    _ = Solution.from_(dwave_result.sampleset)

    print("from_dict")
    (x, y, z), _ = vars(3, Vtype.Binary)
    sample = {x: 0, y: 0, z: 1}
    _ = Solution.from_(sample)
    print("from_dicts")
    (x, y, z), _ = vars(3, Vtype.Binary)
    samples = [
        {x: 0, y: 0, z: 1},  # -1
        {x: 1, y: 1, z: 1},  # -1
        {x: 1, y: 0, z: 0},  # 1
        {x: 0, y: 1, z: 1},  # -2
    ]
    _ = Solution.from_(samples)


# def test_solution_from_aws(aws_model: Model, aws_result: dict[str, NDArray]):
#     ssol = Solution.from_(aws_result, env=aws_model.environment)
#     tsol = AwsTranslator.to_aq(aws_result, env=aws_model.environment)
#     (sol_agg, indices, _) = np.unique(
#         aws_result["samples"], return_index=True, return_counts=True, axis=0
#     )
#
#     assert ssol.samples.tolist() == sol_agg.tolist()
#     assert ssol.samples.tolist() == tsol.samples.tolist()
#     for i, result in enumerate(ssol.results):
#         assert result.raw_energy == aws_result["energies"][indices[i]]
#         assert result.obj_value is None
#         assert result.constraints is None
#         assert result.feasible is None
#
#     for sr, tr in zip(ssol.results, tsol.results, strict=True):
#         assert sr.raw_energy == tr.raw_energy
#         assert sr.obj_value == tr.obj_value
#         assert sr.constraints == tr.constraints
#         assert sr.feasible == tr.feasible
