import tempfile
from random import Random

import gurobipy as gp
import pytest
import sys
from dimod import lp as dimod_lp

from aqmodels import LpTranslator
from pytests.test_core.utils import generate_cqms, make_seed

NOT_RUN_CPLEX = True
try:
    import cplex
except ImportError as _:
    print(
        "Cplex is not installed and thus, the CPLEX tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_CPLEX = True

NUM_CQMS: int = 100


@pytest.mark.translator
def test_cqm_to_model_to_cqm():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        cqm_lp = dimod_lp.dumps(cqm)
        cqm = dimod_lp.loads(cqm_lp)
        # print(f"cqm_lp =\n{cqm_lp}\n")
        model = LpTranslator.to_model(cqm_lp)
        cqm_lp_back = LpTranslator.from_model(model)
        # print(f"cqm_lp_back =\n{cqm_lp_back}\n")
        cqm_back = dimod_lp.loads(cqm_lp_back)
        # this is ugly, but the constraints are weird in dimod.
        # E.g. this: -29 * x_0 + 12 * x_2 <= 0.0
        # will be translated to: x_0 + 12 * x_2 <= -29
        # Also, we remove any 0 values, thus we need to check the actual contents.
        check_dimod_expr(cqm.objective, cqm_back.objective)
        for name, constr in cqm.constraints.items():
            constr_back = cqm_back.constraints[name]
            check_dimod_expr(constr.lhs, constr_back.lhs)
            assert constr.rhs == constr_back.rhs
            assert type(constr) is type(constr_back)


@pytest.mark.translator
def test_gurobi_to_model_to_gurobi():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        # We use CQM's assuming the LP file is correctly formatted for Gurobi.
        # SETUP
        tmp_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")
        dimod_lp.dump(cqm, tmp_lp.file)  # type: ignore
        tmp_lp.flush()
        gp_model = gp.read(tmp_lp.name)
        # ACTUAL
        # build cplex base model (ground truth)
        tmp_lp.seek(0)
        tmp_lp.truncate()
        gp_model.write(tmp_lp.name)
        tmp_lp.flush()
        tmp_lp.seek(0)
        # build aqmodel
        tmp_lp.seek(0)
        aqmodel = LpTranslator.to_model(tmp_lp.file.read())
        lp_str = LpTranslator.from_model(aqmodel)
        # write to lp file
        tmp_lp.seek(0)
        tmp_lp.truncate()
        tmp_lp.write(lp_str)
        tmp_lp.flush()
        tmp_lp.seek(0)
        # build cplex model back
        gp_model_back = gp.read(tmp_lp.name)
        assert gp_models_are_equal(gp_model, gp_model_back)


@pytest.mark.skipif(NOT_RUN_CPLEX, reason="CPLEX is required for test")
@pytest.mark.translator
def test_cplex_to_model_to_cplex():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        # We use CQM's assuming the LP file is correctly formatted for Gurobi.
        # SETUP
        tmp_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")
        tmp_mps = tempfile.NamedTemporaryFile(mode="w+", suffix=".mps")
        dimod_lp.dump(cqm, tmp_lp.file)  # type: ignore
        tmp_lp.flush()
        tmp_lp.seek(0)
        cpx_model = cplex.Cplex(tmp_lp.name)
        cpx_model.set_log_stream(None)
        # ACTUAL
        # build cplex base model (ground truth)
        tmp_lp.seek(0)
        cpx_model.write(tmp_lp.name)
        # store the mps file string from the base
        cpx_model.write(tmp_mps.name)
        tmp_mps.seek(0)
        cpx_mps_str = tmp_mps.read()
        # build aqmodel
        tmp_lp.seek(0)
        aqmodel = LpTranslator.to_model(tmp_lp.file.read())
        lp_str = LpTranslator.from_model(aqmodel)
        print(lp_str)
        # write to lp file
        tmp_lp.seek(0)
        tmp_lp.write(lp_str)
        tmp_lp.seek(0)
        # build cplex model back
        cpx_model_back = cplex.Cplex(tmp_lp.name)
        # store the mps file string from the back
        cpx_model_back.write(tmp_mps.name)
        tmp_mps.seek(0)
        cpx_back_mps_str = tmp_mps.read()
        # compare the two MPS strings
        assert cpx_mps_str == cpx_back_mps_str


def check_dimod_expr(cqm, cqm_back):
    assert cqm.offset == cqm_back.offset
    for u in cqm.variables:
        assert cqm.get_linear(u) == cqm_back.get_linear(u), (
            f"linear not equal for '{u}'"
        )
        for v in cqm.variables:
            if u == v:
                continue
            assert cqm.get_quadratic(u, v, default=0) == cqm_back.get_quadratic(
                u, v, default=0
            ), f"quadratic not equal for '{u=}' and '{v=}'"


def lin_expr_equal(e1, e2):
    """Compare two linear expressions (same terms, same coefficients)."""
    terms1 = sorted([(e1.getVar(i).VarName, e1.getCoeff(i)) for i in range(e1.size())])
    terms2 = sorted([(e2.getVar(i).VarName, e2.getCoeff(i)) for i in range(e2.size())])
    return terms1 == terms2


def quad_expr_equal(e1: gp.QuadExpr, e2: gp.QuadExpr):
    """Compare full quadratic expressions (linear + quadratic parts)."""
    # Compare linear part
    # lin1 = sorted([(e1.getVar1(i).VarName, e1.getCoeff(i)) for i in range(e1.size())])
    # lin2 = sorted([(e2.getVar1(i).VarName, e2.getCoeff(i)) for i in range(e2.size())])
    # if lin1 != lin2:
    #     return False

    # Compare quadratic part
    # if not isinstance(e1, gp.QuadExpr) and not isinstance(e2, gp.QuadExpr):
    #     return True  # no quadratic parts in either
    # if not isinstance(e1, gp.QuadExpr) or not isinstance(e2, gp.QuadExpr):
    #     return False  # one has quadratic, one doesn't

    quad1 = sorted(
        [
            tuple(
                sorted((e1.getVar1(i).VarName, e1.getVar2(i).VarName))
                + [e1.getCoeff(i)]
            )
            for i in range(e1.size())
        ]
    )
    quad2 = sorted(
        [
            tuple(
                sorted((e2.getVar1(i).VarName, e2.getVar2(i).VarName))
                + [e2.getCoeff(i)]
            )
            for i in range(e2.size())
        ]
    )
    return quad1 == quad2


def gp_models_are_equal(m1: gp.Model, m2: gp.Model) -> bool:
    # Compare model sense
    if m1.ModelSense != m2.ModelSense:
        return False

    # Compare variables
    v1s = sorted(m1.getVars(), key=lambda v: v.VarName)
    v2s = sorted(m2.getVars(), key=lambda v: v.VarName)
    if len(v1s) != len(v2s):
        return False
    for v1, v2 in zip(v1s, v2s):
        if not (
                v1.VarName == v2.VarName
                and abs(v1.LB - v2.LB) < 1e-6
                and abs(v1.UB - v2.UB) < 1e-6
                and v1.VType == v2.VType
        ):
            return False

    # Compare objective
    m1_obj = m1.getObjective()
    m2_obj = m2.getObjective()
    if type(m1_obj) is not type(m2_obj):
        return False
    if (
            isinstance(m1_obj, gp.QuadExpr)
            and isinstance(m2_obj, gp.QuadExpr)
            and not quad_expr_equal(m1_obj, m2_obj)
    ):
        return False
    if (
            isinstance(m1_obj, gp.LinExpr)
            and isinstance(m2_obj, gp.LinExpr)
            and not lin_expr_equal(m1_obj, m2_obj)
    ):
        return False

    # Compare constraints
    c1s = sorted(m1.getConstrs(), key=lambda c: c.ConstrName)
    c2s = sorted(m2.getConstrs(), key=lambda c: c.ConstrName)
    if len(c1s) != len(c2s):
        return False

    for c1, c2 in zip(c1s, c2s):
        expr1 = m1.getRow(c1)
        expr2 = m2.getRow(c2)
        if not lin_expr_equal(expr1, expr2):
            return False
        if c1.Sense != c2.Sense or abs(c1.RHS - c2.RHS) > 1e-6:
            return False

    return True
