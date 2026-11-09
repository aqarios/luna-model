"""LP translator tests against dimod and the solvers Gurobi, SCIP and CPLEX."""

from pathlib import Path
from random import Random

import pytest
from dimod import lp as dimod_lp

from luna_model.errors import TranslationError
from luna_model.translator import LpTranslator
from tests.test_core.utils import generate_cqms

from .utils_model_comparison import (
    NOT_RUN_CPLEX,
    NOT_RUN_GUROBI,
    NOT_RUN_SCIP,
    NormalizedModel,
    ScipModel,
    compare_models,
    cplex,
    extract_cplex,
    extract_gurobi,
    extract_luna,
    extract_scip,
    gp,
    model_details,
)

# Each seed is one reproducible test case of CQMS_PER_SEED random CQMs.
SEEDS = range(10)
CQMS_PER_SEED = 10

GUROBI = pytest.param("gurobi", marks=pytest.mark.skipif(NOT_RUN_GUROBI, reason="Gurobi is required"))
SCIP = pytest.param("scip", marks=pytest.mark.skipif(NOT_RUN_SCIP, reason="SCIP is required"))
CPLEX = pytest.param("cplex", marks=pytest.mark.skipif(NOT_RUN_CPLEX, reason="CPLEX is required"))


def cqm_lp_files(seed: int, tmp_path: Path) -> list[Path]:
    """LP files written by dimod for random CQMs."""
    paths = []
    for i, cqm in enumerate(generate_cqms(CQMS_PER_SEED, Random(seed))):
        path = tmp_path / f"cqm_{i}.lp"
        path.write_text(dimod_lp.dumps(cqm))
        paths.append(path)
    return paths


def load(solver: str, path: Path):
    """Reads `path` into the solver's own model object."""
    if solver == "gurobi":
        return gp.read(str(path))
    if solver == "cplex":
        model = cplex.Cplex()
        model.set_results_stream(None)
        model.set_log_stream(None)
        model.set_warning_stream(None)
        model.read(str(path))
        return model
    model = ScipModel()
    model.hideOutput()
    model.readProblem(str(path))
    return model


def save(solver: str, model, path: Path):
    """Writes the solver's model as LP, in the solver's own dialect."""
    if solver == "scip":
        model.writeProblem(str(path))
    else:
        model.write(str(path))


def normalize(solver: str, model, var_names: set[str]) -> NormalizedModel:
    if solver == "gurobi":
        return extract_gurobi(model)
    if solver == "cplex":
        return extract_cplex(model)
    # SCIP adds auxiliary variables for quadratic objectives; keep only the model's own.
    return extract_scip(model, var_names)


def var_names(model) -> set[str]:
    return {v.name for v in model.environment.variables()}


def assert_same(expected: NormalizedModel, actual: NormalizedModel):
    ok, kind, msg = compare_models(expected, actual)
    assert ok, f"[{kind.value}] {msg}\n{model_details(kind, actual)}"


##################################### Solvers #########################################


# SCIP writes a quadratic objective as an extra variable plus a constraint, so Luna reading
# SCIP's file gets an equivalent but different model. Gurobi and CPLEX keep it as written.
@pytest.mark.parametrize("solver", [GUROBI, CPLEX])
@pytest.mark.parametrize("seed", SEEDS)
def test_luna_reads_solver_lp_like_the_solver(solver: str, seed: int, tmp_path: Path):
    """Luna reads an LP file written by the solver into the model the solver has."""
    for path in cqm_lp_files(seed, tmp_path):
        model = load(solver, path)
        solver_lp = tmp_path / f"{solver}_{path.name}"
        save(solver, model, solver_lp)

        lm = LpTranslator.to_lm(solver_lp)
        assert_same(normalize(solver, model, var_names(lm)), extract_luna(lm))


@pytest.mark.parametrize("solver", [GUROBI, SCIP, CPLEX])
@pytest.mark.parametrize("seed", SEEDS)
def test_solver_reads_luna_lp_unchanged(solver: str, seed: int, tmp_path: Path):
    """The solver reads Luna's LP output into the same model as the original file."""
    for path in cqm_lp_files(seed, tmp_path):
        lm = LpTranslator.to_lm(path)
        luna_lp = tmp_path / f"luna_{path.name}"
        LpTranslator.from_lm(lm, luna_lp)

        names = var_names(lm)
        original = normalize(solver, load(solver, path), names)
        assert_same(original, normalize(solver, load(solver, luna_lp), names))


##################################### Dimod ###########################################


@pytest.mark.parametrize("seed", SEEDS)
def test_lp_file_str_path(seed: int, tmp_path: Path):
    for path in cqm_lp_files(seed, tmp_path):
        lm_from_contents = LpTranslator.to_lm(path.read_text())
        lm_from_path = LpTranslator.to_lm(path)
        lm_from_path_as_str = LpTranslator.to_lm(str(path))

        assert lm_from_contents.equal_contents(lm_from_path)
        assert lm_from_path.equal_contents(lm_from_path_as_str)


@pytest.mark.parametrize("seed", SEEDS)
def test_cqm_to_model_to_cqm(seed: int):
    for cqm in generate_cqms(CQMS_PER_SEED, Random(seed)):
        cqm_lp = dimod_lp.dumps(cqm)
        cqm = dimod_lp.loads(cqm_lp)
        model = LpTranslator.to_lm(cqm_lp)
        cqm_back = dimod_lp.loads(LpTranslator.from_lm(model))
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


def test_invalid_var_name():
    cqm = generate_cqms(1, Random(0))[0]
    lp_str = dimod_lp.dumps(cqm).replace("x_0", "0x")
    with pytest.raises(TranslationError):
        _ = LpTranslator.to_lm(lp_str)
