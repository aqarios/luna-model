import sys
import tempfile
from pathlib import Path
from random import Random

import pytest
from dimod import lp as dimod_lp
from luna_model import Sense
from luna_model.errors import TranslationError
from luna_model.translator import LpTranslator

from tests.test_core.utils import generate_cqms, make_seed

NOT_RUN_SCIP = False
try:
    from pyscipopt import Model as ScipModel
except ImportError as _:
    print(
        "SCIP is not installed and thus, the Gurobi tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_SCIP = True

NOT_RUN_GUROBI = False
try:
    import gurobipy as gp
except ImportError as _:
    print(
        "Gurobi is not installed and thus, the Gurobi tests will not be executed",
        file=sys.stdout,
    )
    NOT_RUN_GUROBI = True

NOT_RUN_CPLEX = True
# TODO: fix CPLEX test logic. MPS is unreliable
# try:
#     import cplex  # type: ignore
# except ImportError as _:
#     print(
#         "Cplex is not installed and thus, the CPLEX tests will not be executed",
#         file=sys.stdout,
#     )
#     NOT_RUN_CPLEX = True
# if sys.version_info == (3, 12):
#     NOT_RUN_CPLEX = True

NUM_CQMS: int = 100
GP_SENSE_MIN: int = 1
GP_SENSE_MAX: int = 0


def test_lp_file_str_path():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        # SETUP
        tmp_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")
        dimod_lp.dump(cqm, tmp_lp.file)  # type: ignore
        tmp_lp.flush()
        tmp_lp.seek(0)

        lm_from_contents = LpTranslator.to_lm(tmp_lp.file.read())
        lm_from_path = LpTranslator.to_lm(Path(tmp_lp.file.name))
        lm_from_path_as_str = LpTranslator.to_lm(str(tmp_lp.file.name))

        assert lm_from_contents.equal_contents(lm_from_path)
        assert lm_from_path.equal_contents(lm_from_path_as_str)


##################################### Dimod ###########################################


def test_cqm_to_model_to_cqm():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        cqm_lp = dimod_lp.dumps(cqm)
        cqm = dimod_lp.loads(cqm_lp)
        # print(f"cqm_lp =\n{cqm_lp}\n")
        model = LpTranslator.to_lm(cqm_lp)
        cqm_lp_back = LpTranslator.from_lm(model)
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


##################################### Dimod ###########################################

##################################### Gurobi ##########################################


@pytest.mark.skipif(NOT_RUN_GUROBI, reason="Gurobi is required for test")
def test_gurobi_to_model_to_gurobi():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        # We use CQM's assuming the LP file is correctly formatted for Gurobi.
        # SETUP
        tmp_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")
        dimod_lp.dump(cqm, tmp_lp.file)  # type: ignore
        tmp_lp.flush()
        tmp_lp.seek(0)
        gp_model = gp.read(tmp_lp.name)
        # ACTUAL
        # build cplex base model (ground truth)
        tmp_lp.seek(0)
        tmp_lp.truncate()
        gp_model.write(tmp_lp.name)
        tmp_lp.flush()
        tmp_lp.seek(0)
        # build luna_model
        tmp_lp.seek(0)
        lmodel = LpTranslator.to_lm(tmp_lp.file.read())
        lp_str = LpTranslator.from_lm(lmodel)
        # write to lp file
        tmp_lp.seek(0)
        tmp_lp.truncate()
        tmp_lp.write(lp_str)
        tmp_lp.flush()
        tmp_lp.seek(0)
        # build cplex model back
        gp_model_back = gp.read(tmp_lp.name)
        assert gp_models_are_equal(gp_model, gp_model_back)


@pytest.mark.skipif(NOT_RUN_GUROBI, reason="Gurobi is required for test")
def test_gurobi_and_lm_lp_read_equality():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        # We use CQM's assuming the LP file is correctly formatted for Gurobi.
        # SETUP
        tmp_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")
        dimod_lp.dump(cqm, tmp_lp.file)  # type: ignore
        tmp_lp.flush()
        tmp_lp.seek(0)

        gp_model = gp.read(tmp_lp.name)
        tmp_lp.seek(0)
        lm_model = LpTranslator.to_lm(tmp_lp.file.read())

        # Check that the sense is equal
        assert gp_model.ModelSense == GP_SENSE_MIN
        assert lm_model.sense == Sense.Min

        gp_objective = gp_model.getObjective()
        if isinstance(gp_objective, gp.QuadExpr):
            gp_lin_obj = gp_objective.getLinExpr()
            print("----------- LINEAR -----------", file=sys.stderr)
            for i in range(gp_lin_obj.size()):
                v_name = gp_lin_obj.getVar(i).VarName
                gp_coef = gp_lin_obj.getCoeff(i)
                lm_coef = lm_model.objective.get_linear(lm_model.environment.get_variable(v_name))
                assert gp_coef == lm_coef

            print("----------- QUADRATIC -----------", file=sys.stderr)
            for i in range(gp_objective.size()):
                v_name_1 = gp_objective.getVar1(i).VarName
                v_name_2 = gp_objective.getVar2(i).VarName
                gp_coef = gp_objective.getCoeff(i)
                lm_coef = lm_model.objective.get_quadratic(
                    lm_model.environment.get_variable(v_name_1),
                    lm_model.environment.get_variable(v_name_2),
                )
                assert gp_coef == lm_coef


##################################### Gurobi ##########################################

###################################### SCIP ###########################################


@pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required for test")
def test_scip_to_model_to_scip():
    rand = Random(make_seed())
    cqms = generate_cqms(NUM_CQMS, rand)
    for cqm in cqms:
        # We use CQM's assuming the LP file is correctly formatted.
        # SETUP
        tmp_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")
        lm_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")

        dimod_lp.dump(cqm, tmp_lp.file)  # type: ignore
        tmp_lp.flush()
        tmp_lp.seek(0)

        scip_model = ScipModel()
        scip_model.readProblem(tmp_lp.name)

        lp_str = LpTranslator.from_lm(LpTranslator.to_lm(tmp_lp.file.read()))
        assert lp_str is not None
        lm_lp.write(lp_str)
        lm_lp.flush()
        tmp_lp.seek(0)

        # build cplex model back
        scip_model_back = ScipModel()
        scip_model_back.readProblem(tmp_lp.name)

        is_equal, msg = scip_models_are_equal(scip_model, scip_model_back)
        assert is_equal, msg


###################################### SCIP ###########################################

###################################### CPLEX ##########################################


@pytest.mark.skipif(NOT_RUN_CPLEX, reason="CPLEX is required for test")
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
        # build lmodel
        tmp_lp.seek(0)
        lmodel = LpTranslator.to_lm(tmp_lp.file.read())
        lp_str = LpTranslator.from_lm(lmodel)
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


###################################### CPLEX ##########################################


def check_dimod_expr(cqm, cqm_back):
    assert cqm.offset == cqm_back.offset
    for u in cqm.variables:
        assert cqm.get_linear(u) == cqm_back.get_linear(u), f"linear not equal for '{u}'"
        for v in cqm.variables:
            if u == v:
                continue
            assert cqm.get_quadratic(u, v, default=0) == cqm_back.get_quadratic(u, v, default=0), (
                f"quadratic not equal for '{u=}' and '{v=}'"
            )


def lin_expr_equal(e1, e2):
    """Compare two linear expressions (same terms, same coefficients)."""
    terms1 = sorted([(e1.getVar(i).VarName, e1.getCoeff(i)) for i in range(e1.size())])
    terms2 = sorted([(e2.getVar(i).VarName, e2.getCoeff(i)) for i in range(e2.size())])
    return terms1 == terms2


def quad_expr_equal(e1: gp.QuadExpr, e2: gp.QuadExpr):
    """Compare full quadratic expressions (linear + quadratic parts)."""
    # Compare linear part
    e1_lin = e1.getLinExpr()
    e2_lin = e2.getLinExpr()
    lin1 = sorted([(e1_lin.getVar(i).VarName, e1_lin.getCoeff(i)) for i in range(e1_lin.size())])
    lin2 = sorted([(e2_lin.getVar(i).VarName, e2_lin.getCoeff(i)) for i in range(e2_lin.size())])
    if lin1 != lin2:
        return False

    # Compare quadratic part
    # if not isinstance(e1, gp.QuadExpr) and not isinstance(e2, gp.QuadExpr):
    #     return True  # no quadratic parts in either
    # if not isinstance(e1, gp.QuadExpr) or not isinstance(e2, gp.QuadExpr):
    #     return False  # one has quadratic, one doesn't

    quad1 = sorted(
        [tuple(sorted((e1.getVar1(i).VarName, e1.getVar2(i).VarName)) + [e1.getCoeff(i)]) for i in range(e1.size())]
    )
    quad2 = sorted(
        [tuple(sorted((e2.getVar1(i).VarName, e2.getVar2(i).VarName)) + [e2.getCoeff(i)]) for i in range(e2.size())]
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
    if isinstance(m1_obj, gp.QuadExpr) and isinstance(m2_obj, gp.QuadExpr) and not quad_expr_equal(m1_obj, m2_obj):
        return False
    if isinstance(m1_obj, gp.LinExpr) and isinstance(m2_obj, gp.LinExpr) and not lin_expr_equal(m1_obj, m2_obj):
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


def scip_models_are_equal(model1: ScipModel, model2: ScipModel) -> tuple[bool, str]:
    if model1.getObjectiveSense() != model2.getObjectiveSense():
        return (
            False,
            f"objective sense not equal: {model1.getObjectiveSense()} and {model2.getObjectiveSense()}",
        )

    if model1.getObjoffset() != model2.getObjoffset():
        return (
            False,
            f"offset not equal: {model1.getObjoffset()} and {model2.getObjoffset()}",
        )

    m1_variables = model1.getVars()
    m2_variables = model2.getVars()

    m1_var_lookup = {str(var): str(var.vtype()) for var in m1_variables}
    m2_var_lookup = {str(var): str(var.vtype()) for var in m2_variables}
    if m1_var_lookup != m2_var_lookup:
        return False, f"vars not equal: {m1_var_lookup}, {m2_var_lookup}"

    m1_expr = model1.getObjective()
    m2_expr = model2.getObjective()

    for m1_var, m2_var in zip(m1_variables, m2_variables):
        m1_value_m1_var = m1_expr[m1_var]
        m2_value_m2_var = m2_expr[m2_var]
        if m1_value_m1_var != m2_value_m2_var:
            return False, f"({m1_var}) => {m1_value_m1_var} vs {m2_value_m2_var}"

    m1_conss_lookup = {
        str(con): model1.getValsLinear(con) if con.isLinear() else model1.getTermsQuadratic(con)
        for con in model1.getConss()
    }
    m2_conss_lookup = {
        str(con): model2.getValsLinear(con) if con.isLinear() else model2.getTermsQuadratic(con)
        for con in model2.getConss()
    }

    for m1_name, m1_item in m1_conss_lookup.items():
        m2_item = m2_conss_lookup.get(m1_name)
        if m2_item is None:
            return (
                False,
                f"constraint for name '{m1_name}' does not exist in second model",
            )
        if isinstance(m1_item, tuple):
            m1_q, m1_s, m1_l = m1_item
            m2_q, m2_s, m2_l = m2_item

            m1_dict = {}
            for u, v, b in m1_q:
                m1_dict[(str(u), str(v))] = b
                m1_dict[(str(v), str(u))] = b
            for u, b in m1_l:
                m1_dict[str(u)] = b

            m2_dict = {}
            for u, v, b in m2_q:
                m2_dict[(str(u), str(v))] = b
                m2_dict[(str(v), str(u))] = b
            for u, b in m2_l:
                m2_dict[str(u)] = b

            if m1_dict != m2_dict:
                return False, f"{m1_dict} != {m2_dict}"

            if m1_s != list():
                return False, f"{m1_s} != []"
            if m2_s != list():
                return False, f"{m2_s} != []"

        elif m1_item != m2_item:
            return False, ""

    return True, ""


def test_invalid_var_name():
    rand = Random(make_seed())
    cqm = generate_cqms(1, rand)[0]

    # SETUP
    tmp_lp = tempfile.NamedTemporaryFile(mode="w+", suffix=".lp")
    dimod_lp.dump(cqm, tmp_lp.file)  # type: ignore
    tmp_lp.flush()
    tmp_lp.seek(0)

    lp_str = tmp_lp.file.read().replace("x_0", "0x")
    with pytest.raises(TranslationError):
        _ = LpTranslator.to_lm(lp_str)
