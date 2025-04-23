import pytest
from random import Random
from aqmodels import CqmTranslator
from dimod import lp as dimod_lp
from pytests.test_core.test_translator.test_lp_translator import check_dimod_expr
from pytests.test_core.utils import generate_cqms, make_seed


NUM_CQMS: int = 100


@pytest.mark.translator
def test_cqm_to_model_to_cqm():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        cqm = dimod_lp.loads(dimod_lp.dumps(cqm))
        model = CqmTranslator.to_model(cqm)
        cqm_back = CqmTranslator.from_model(model)
        check_dimod_expr(cqm.objective, cqm_back.objective)
        for name, constr in cqm.constraints.items():
            constr_back = cqm_back.constraints[name]
            check_dimod_expr(constr.lhs, constr_back.lhs)
            assert constr.rhs == constr_back.rhs
            assert type(constr) is type(constr_back)
