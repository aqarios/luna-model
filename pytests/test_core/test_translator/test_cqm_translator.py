from random import Random

import pytest
from dimod import Binary, Integer, Real, ConstrainedQuadraticModel
from dimod import lp as dimod_lp

from aqmodels.translator import CqmTranslator
from pytests.test_core.test_translator.test_lp_translator import check_dimod_expr
from pytests.test_core.utils import generate_cqms, make_seed

NUM_CQMS: int = 100


@pytest.mark.translator
def test_cqm_to_aq_to_cqm():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        cqm = dimod_lp.loads(dimod_lp.dumps(cqm))
        model = CqmTranslator.to_aq(cqm)
        cqm_back = CqmTranslator.from_aq(model)
        check_dimod_expr(cqm.objective, cqm_back.objective)
        for name, constr in cqm.constraints.items():
            constr_back = cqm_back.constraints[name]
            check_dimod_expr(constr.lhs, constr_back.lhs)
            assert constr.rhs == constr_back.rhs
            assert type(constr) is type(constr_back)


@pytest.mark.translator
def test_invalid_var_name():
    x = Binary("0")
    y = Binary("1")
    i = Integer("2", lower_bound=0, upper_bound=10)
    r = Real("3", lower_bound=0.0, upper_bound=5.0)
    cqm = ConstrainedQuadraticModel()
    objective = x * y + 2 * i - 3 * r + i * i
    cqm.set_objective(objective)
    cqm.add_constraint(x + y + i <= 5, label="constraint1")
    with pytest.raises(ValueError, match="Label '0' cannot be output to an LP file"):
        _ = CqmTranslator.to_aq(cqm)
