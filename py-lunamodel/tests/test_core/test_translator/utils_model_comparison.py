"""Shared normalization and comparison utilities for MPS translator tests. Package specific things, are marked with
```PackageNote: ...``` - ```# PackageNote: Gurobi may read binary vars (integer with [0,1] bounds) as INTEGER```
and should be checked regularly."""

from __future__ import annotations

import sys
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path

from luna_model import Sense
from luna_model import Model as LunaModel

NOT_RUN_SCIP = False
try:
    from pyscipopt import Model as ScipModel
except ImportError as _:
    print(
        "SCIP is not installed and thus, the SCIP tests will not be executed",
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


MPS_DIR = Path(__file__).parent / "mps_models"
MPS_FILES = sorted(p.name for p in MPS_DIR.glob("*.mps"))

TOL = 1e-6
_OK: tuple[bool, MismatchKind | None, str] = (True, None, "")


# ─────────────────────── Types ───────────────────────────────────────────────


class NormalizedVtype(Enum):
    BINARY = "binary"
    INTEGER = "integer"
    CONTINUOUS = "continuous"


class NormalizedSense(Enum):
    LE = "Le"
    GE = "Ge"
    EQ = "Eq"


class MismatchKind(Enum):
    SENSE = "sense"
    VARIABLE_COUNT = "variable_count"
    VARIABLE_NAME = "variable_name"
    VARIABLE_TYPE = "variable_type"
    VARIABLE_BOUNDS = "variable_bounds"
    OBJECTIVE_OFFSET = "objective_offset"
    OBJECTIVE_LINEAR = "objective_linear"
    OBJECTIVE_QUADRATIC = "objective_quadratic"
    CONSTRAINT_COUNT = "constraint_count"
    CONSTRAINT_NAME = "constraint_name"
    CONSTRAINT_SENSE = "constraint_sense"
    CONSTRAINT_RHS = "constraint_rhs"
    CONSTRAINT_LINEAR = "constraint_linear"
    CONSTRAINT_QUADRATIC = "constraint_quadratic"


@dataclass
class NormalizedVar:
    name: str
    vtype: NormalizedVtype
    lb: float   # -inf for unbounded below
    ub: float   # inf for unbounded above


@dataclass
class NormalizedConstraint:
    name: str
    sense: NormalizedSense
    rhs: float
    linear: dict[str, float] = field(default_factory=dict)
    quadratic: dict[tuple[str, str], float] = field(default_factory=dict)


@dataclass
class NormalizedModel:
    source: str
    is_minimize: bool
    variables: list[NormalizedVar]
    obj_offset: float
    obj_linear: dict[str, float]
    obj_quadratic: dict[tuple[str, str], float]
    linear_constraints: list[NormalizedConstraint]
    quadratic_constraints: list[NormalizedConstraint]


# ─────────────────────── Extraction: Gurobi ──────────────────────────────────


def extract_gurobi(gp_model: "gp.Model") -> NormalizedModel:
    from gurobipy import GRB

    vtype_map = {GRB.BINARY: NormalizedVtype.BINARY, GRB.INTEGER: NormalizedVtype.INTEGER, GRB.CONTINUOUS: NormalizedVtype.CONTINUOUS}
    sense_map = {GRB.LESS_EQUAL: NormalizedSense.LE, GRB.GREATER_EQUAL: NormalizedSense.GE, GRB.EQUAL: NormalizedSense.EQ}

    variables = []
    for v in sorted(gp_model.getVars(), key=lambda v: v.VarName):
        vtype = vtype_map[v.VType]
        # PackageNote: Gurobi may read binary vars (integer with [0,1] bounds) as INTEGER
        if vtype == NormalizedVtype.INTEGER and abs(v.LB) < TOL and abs(v.UB - 1.0) < TOL:
            vtype = NormalizedVtype.BINARY
        lb = float("-inf") if v.LB <= -1e20 else v.LB
        ub = float("inf") if v.UB >= 1e20 else v.UB
        variables.append(NormalizedVar(v.VarName, vtype, lb, ub))

    # Objective
    gp_obj = gp_model.getObjective()
    obj_offset = gp_obj.getConstant() if hasattr(gp_obj, "getConstant") else 0.0

    gp_lin = gp_obj.getLinExpr() if isinstance(gp_obj, gp.QuadExpr) else gp_obj
    obj_linear: dict[str, float] = {}
    for i in range(gp_lin.size()):
        c = gp_lin.getCoeff(i)
        if abs(c) > TOL:
            obj_linear[gp_lin.getVar(i).VarName] = c

    obj_quadratic: dict[tuple[str, str], float] = {}
    if isinstance(gp_obj, gp.QuadExpr):
        for i in range(gp_obj.size()):
            key = tuple(sorted((gp_obj.getVar1(i).VarName, gp_obj.getVar2(i).VarName)))
            obj_quadratic[key] = obj_quadratic.get(key, 0.0) + gp_obj.getCoeff(i)

    # Linear constraints
    linear_constraints = []
    for gc in sorted(gp_model.getConstrs(), key=lambda c: c.ConstrName):
        row = gp_model.getRow(gc)
        linear = {}
        for i in range(row.size()):
            c = row.getCoeff(i)
            if abs(c) > TOL:
                linear[row.getVar(i).VarName] = c
        linear_constraints.append(NormalizedConstraint(
            gc.ConstrName, sense_map.get(gc.Sense, gc.Sense), gc.RHS, linear,
        ))

    # Quadratic constraints
    quadratic_constraints = []
    for gqc in sorted(gp_model.getQConstrs(), key=lambda c: c.QCName):
        qrow = gp_model.getQCRow(gqc)
        lin_expr = qrow.getLinExpr()
        linear = {}
        for i in range(lin_expr.size()):
            c = lin_expr.getCoeff(i)
            if abs(c) > TOL:
                linear[lin_expr.getVar(i).VarName] = c
        quadratic: dict[tuple[str, str], float] = {}
        for i in range(qrow.size()):
            key = tuple(sorted((qrow.getVar1(i).VarName, qrow.getVar2(i).VarName)))
            quadratic[key] = quadratic.get(key, 0.0) + qrow.getCoeff(i)
        quadratic_constraints.append(NormalizedConstraint(
            gqc.QCName, sense_map.get(gqc.QCSense, gqc.QCSense), gqc.QCRHS,
            linear, quadratic,
        ))

    sense_name = "GRB.MINIMIZE" if gp_model.ModelSense == GRB.MINIMIZE else "GRB.MAXIMIZE"
    return NormalizedModel(
        source=f"Gurobi({sense_name})",
        is_minimize=gp_model.ModelSense == GRB.MINIMIZE,
        variables=variables,
        obj_offset=obj_offset,
        obj_linear=obj_linear,
        obj_quadratic=obj_quadratic,
        linear_constraints=linear_constraints,
        quadratic_constraints=quadratic_constraints,
    )


# ─────────────────────── Extraction: SCIP ────────────────────────────────────


def extract_scip(
    scip_model: "ScipModel", var_names: set[str] | None = None,
) -> NormalizedModel:
    """Extract SCIP model. If var_names is given, only include those variables
    (SCIP may add auxiliary variables like 'qmatrixvar')."""
    vtype_map = {"BINARY": NormalizedVtype.BINARY, "INTEGER": NormalizedVtype.INTEGER, "CONTINUOUS": NormalizedVtype.CONTINUOUS}

    scip_all_vars = sorted(scip_model.getVars(), key=lambda v: str(v))
    scip_vars = (
        [v for v in scip_all_vars if str(v) in var_names]
        if var_names is not None
        else scip_all_vars
    )

    variables = []
    for v in scip_vars:
        lb_raw = v.getLbOriginal()
        ub_raw = v.getUbOriginal()
        lb = float("-inf") if lb_raw <= -1e20 else lb_raw
        ub = float("inf") if ub_raw >= 1e20 else ub_raw
        variables.append(NormalizedVar(
            str(v), vtype_map.get(str(v.vtype()), NormalizedVtype.CONTINUOUS), lb, ub,
        ))

    # Objective
    scip_obj = scip_model.getObjective()
    obj_offset = scip_model.getObjoffset()

    obj_linear: dict[str, float] = {}
    for sv in scip_vars:
        c = scip_obj[sv]
        if abs(c) > TOL:
            obj_linear[str(sv)] = c

    # Quadratic objective: SCIP may linearize via 'qmatrixvar' + 'qmatrix' constraint
    scip_conss = sorted(scip_model.getConss(), key=lambda c: str(c))
    qmatrix_cons = [c for c in scip_conss if str(c) == "qmatrix"]
    obj_quadratic: dict[tuple[str, str], float] = {}
    if qmatrix_cons:
        bilin, sqterms, _ = scip_model.getTermsQuadratic(qmatrix_cons[0])
        for u, v, coeff in bilin:
            if abs(coeff) > TOL:
                key = tuple(sorted((str(u), str(v))))
                obj_quadratic[key] = obj_quadratic.get(key, 0.0) + coeff
        for v, sqcoeff, _ in sqterms:
            if abs(sqcoeff) > TOL:
                key = (str(v), str(v))
                obj_quadratic[key] = obj_quadratic.get(key, 0.0) + sqcoeff
    else:
        try:
            for term, coeff in scip_obj.terms.items():
                vrs = [str(v) for v in term.vartuple]
                if len(vrs) == 2:
                    key = tuple(sorted(vrs))
                    obj_quadratic[key] = obj_quadratic.get(key, 0.0) + coeff
        except AttributeError:
            pass

    # Constraints (filter out internal qmatrix)
    inf = scip_model.infinity()
    conss_filtered = [c for c in scip_conss if str(c) != "qmatrix"]

    linear_constraints = []
    quadratic_constraints = []
    for sc in conss_filtered:
        name = str(sc)
        sc_lhs = scip_model.getLhs(sc)
        sc_rhs = scip_model.getRhs(sc)
        if abs(sc_lhs - sc_rhs) < TOL:
            sense, rhs = NormalizedSense.EQ, sc_rhs
        elif sc_lhs <= -inf + TOL:
            sense, rhs = NormalizedSense.LE, sc_rhs
        else:
            sense, rhs = NormalizedSense.GE, sc_lhs

        if sc.isLinear():
            vals = scip_model.getValsLinear(sc)
            linear = {str(k): v for k, v in vals.items() if abs(v) > TOL}
            linear_constraints.append(NormalizedConstraint(name, sense, rhs, linear))
        else:
            bilin, sqterms, linterms = scip_model.getTermsQuadratic(sc)
            linear: dict[str, float] = {}
            for sv, coeff in linterms:
                if abs(coeff) > TOL:
                    linear[str(sv)] = linear.get(str(sv), 0.0) + coeff
            quadratic: dict[tuple[str, str], float] = {}
            for u, v, coeff in bilin:
                if abs(coeff) > TOL:
                    key = tuple(sorted((str(u), str(v))))
                    quadratic[key] = quadratic.get(key, 0.0) + coeff
            for v, sqcoeff, _ in sqterms:
                if abs(sqcoeff) > TOL:
                    key = (str(v), str(v))
                    quadratic[key] = quadratic.get(key, 0.0) + sqcoeff
            quadratic_constraints.append(
                NormalizedConstraint(name, sense, rhs, linear, quadratic),
            )

    return NormalizedModel(
        source="SCIP",
        is_minimize=scip_model.getObjectiveSense() == "minimize",
        variables=variables,
        obj_offset=obj_offset,
        obj_linear=obj_linear,
        obj_quadratic=obj_quadratic,
        linear_constraints=linear_constraints,
        quadratic_constraints=quadratic_constraints,
    )


# ─────────────────────── Extraction: Luna ────────────────────────────────────


def extract_luna(lm_model: LunaModel) -> NormalizedModel:
    vtype_map = {"Binary": NormalizedVtype.BINARY, "Integer": NormalizedVtype.INTEGER, "Real": NormalizedVtype.CONTINUOUS}

    variables = []
    for v in sorted(lm_model.environment.variables(), key=lambda v: v.name):
        lb = float("-inf") if not isinstance(v.bounds.lower, float) else v.bounds.lower
        ub = float("inf") if not isinstance(v.bounds.upper, float) else v.bounds.upper
        variables.append(NormalizedVar(
            v.name, vtype_map[v.vtype.value], lb, ub,
        ))

    lm_obj = lm_model.objective
    obj_offset = lm_obj.get_offset()
    obj_linear = {v.name: c for v, c in lm_obj.linear_items() if abs(c) > TOL}
    obj_quadratic: dict[tuple[str, str], float] = {}
    for va, vb, c in lm_obj.quadratic_items():
        key = tuple(sorted((va.name, vb.name)))
        obj_quadratic[key] = obj_quadratic.get(key, 0.0) + c

    linear_constraints = []
    quadratic_constraints = []
    for name, con in sorted(lm_model.constraints.items(), key=lambda item: item[0]):
        sense = NormalizedSense(con.comparator.value)
        linear = {v.name: c for v, c in con.lhs.linear_items() if abs(c) > TOL}
        quadratic: dict[tuple[str, str], float] = {}
        for va, vb, c in con.lhs.quadratic_items():
            key = tuple(sorted((va.name, vb.name)))
            quadratic[key] = quadratic.get(key, 0.0) + c
        if con.lhs.has_quadratic():
            quadratic_constraints.append(
                NormalizedConstraint(name, sense, con.rhs, linear, quadratic),
            )
        else:
            linear_constraints.append(NormalizedConstraint(name, sense, con.rhs, linear))

    return NormalizedModel(
        source="Luna",
        is_minimize=lm_model.sense == Sense.MIN,
        variables=variables,
        obj_offset=obj_offset,
        obj_linear=obj_linear,
        obj_quadratic=obj_quadratic,
        linear_constraints=linear_constraints,
        quadratic_constraints=quadratic_constraints,
    )


# ─────────────────────── Comparison ──────────────────────────────────────────


def _fail(kind: MismatchKind, msg: str) -> tuple[bool, MismatchKind, str]:
    return (False, kind, msg)


def _bounds_match(a: float, b: float) -> bool:
    if a <= -1e20 and b <= -1e20:
        return True
    if a >= 1e20 and b >= 1e20:
        return True
    return abs(a - b) <= TOL


def _compare_dicts(
        a: dict[str, float],
        b: dict[str, float],
        a_name: str,
        b_name: str,
        prefix: str = "",
) -> str | None:
    """Compare two str->float dicts. Returns error message or None."""
    if set(a.keys()) != set(b.keys()):
        extra_a = set(a) - set(b)
        extra_b = set(b) - set(a)
        loc = f"{prefix}: " if prefix else ""
        return f"{loc}extra in {a_name}={extra_a}, extra in {b_name}={extra_b}"
    for name in a:
        if abs(a[name] - b[name]) > TOL:
            key = f"{prefix}.{name}" if prefix else name
            return f"{key}: {a_name}={a[name]} vs {b_name}={b[name]}"
    return None


def _compare_quad_dicts(
        a: dict[tuple[str, str], float],
        b: dict[tuple[str, str], float],
        a_name: str,
        b_name: str,
        prefix: str = "",
) -> str | None:
    """Compare two (str,str)->float dicts. Returns error message or None."""
    if set(a.keys()) != set(b.keys()):
        loc = f"{prefix}: " if prefix else ""
        return f"{loc}{a_name} keys={set(a.keys())} vs {b_name} keys={set(b.keys())}"
    for key in a:
        if abs(a[key] - b[key]) > TOL:
            k = f"{prefix}.{key}" if prefix else str(key)
            return f"{k}: {a_name}={a[key]} vs {b_name}={b[key]}"
    return None


def compare_models(
    a: NormalizedModel, b: NormalizedModel,
) -> tuple[bool, MismatchKind | None, str]:
    """Compare two normalized models for equivalence (up to quadratic)."""
    an, bn = a.source, b.source

    # --- Sense ---
    if a.is_minimize != b.is_minimize:
        a_s = "minimize" if a.is_minimize else "maximize"
        b_s = "minimize" if b.is_minimize else "maximize"
        return _fail(MismatchKind.SENSE, f"{an}={a_s} vs {bn}={b_s}")

    # --- Variables ---
    if len(a.variables) != len(b.variables):
        return _fail(
            MismatchKind.VARIABLE_COUNT,
            f"{an}={len(a.variables)} vs {bn}={len(b.variables)}",
        )
    for av, bv in zip(a.variables, b.variables):
        if av.name != bv.name:
            return _fail(MismatchKind.VARIABLE_NAME, f"{an}={av.name} vs {bn}={bv.name}")
        if av.vtype != bv.vtype:
            return _fail(
                MismatchKind.VARIABLE_TYPE,
                f"{av.name}: {an}={av.vtype} vs {bn}={bv.vtype}",
            )
        if not _bounds_match(av.lb, bv.lb) or not _bounds_match(av.ub, bv.ub):
            return _fail(
                MismatchKind.VARIABLE_BOUNDS,
                f"{av.name}: {an}=[{av.lb}, {av.ub}] vs {bn}=[{bv.lb}, {bv.ub}]",
            )

    # --- Objective ---
    if abs(a.obj_offset - b.obj_offset) > TOL:
        return _fail(
            MismatchKind.OBJECTIVE_OFFSET,
            f"{an}={a.obj_offset} vs {bn}={b.obj_offset}",
        )
    err = _compare_dicts(a.obj_linear, b.obj_linear, an, bn)
    if err:
        return _fail(MismatchKind.OBJECTIVE_LINEAR, err)
    err = _compare_quad_dicts(a.obj_quadratic, b.obj_quadratic, an, bn)
    if err:
        return _fail(MismatchKind.OBJECTIVE_QUADRATIC, err)

    # --- Linear constraints ---
    if len(a.linear_constraints) != len(b.linear_constraints):
        return _fail(
            MismatchKind.CONSTRAINT_COUNT,
            f"linear: {an}={len(a.linear_constraints)} vs {bn}={len(b.linear_constraints)}",
        )
    for ac, bc in zip(a.linear_constraints, b.linear_constraints):
        if ac.name != bc.name:
            return _fail(MismatchKind.CONSTRAINT_NAME, f"{an}={ac.name} vs {bn}={bc.name}")
        if ac.sense != bc.sense:
            return _fail(MismatchKind.CONSTRAINT_SENSE, f"{ac.name}: {an}={ac.sense} vs {bn}={bc.sense}")
        if abs(ac.rhs - bc.rhs) > TOL:
            return _fail(MismatchKind.CONSTRAINT_RHS, f"{ac.name}: {an}={ac.rhs} vs {bn}={bc.rhs}")
        err = _compare_dicts(ac.linear, bc.linear, an, bn, prefix=ac.name)
        if err:
            return _fail(MismatchKind.CONSTRAINT_LINEAR, err)

    # --- Quadratic constraints ---
    if len(a.quadratic_constraints) != len(b.quadratic_constraints):
        return _fail(
            MismatchKind.CONSTRAINT_COUNT,
            f"quadratic: {an}={len(a.quadratic_constraints)} vs {bn}={len(b.quadratic_constraints)}",
        )
    for ac, bc in zip(a.quadratic_constraints, b.quadratic_constraints):
        if ac.name != bc.name:
            return _fail(MismatchKind.CONSTRAINT_NAME, f"quad: {an}={ac.name} vs {bn}={bc.name}")
        if ac.sense != bc.sense:
            return _fail(MismatchKind.CONSTRAINT_SENSE, f"{ac.name}: {an}={ac.sense} vs {bn}={bc.sense}")
        if abs(ac.rhs - bc.rhs) > TOL:
            return _fail(MismatchKind.CONSTRAINT_RHS, f"{ac.name}: {an}={ac.rhs} vs {bn}={bc.rhs}")
        err = _compare_dicts(ac.linear, bc.linear, an, bn, prefix=ac.name)
        if err:
            return _fail(MismatchKind.CONSTRAINT_LINEAR, err)
        err = _compare_quad_dicts(ac.quadratic, bc.quadratic, an, bn, prefix=ac.name)
        if err:
            return _fail(MismatchKind.CONSTRAINT_QUADRATIC, err)

    return _OK


# ─────────────────────── Diagnostics ─────────────────────────────────────────


def model_details(kind: MismatchKind | None, model: NormalizedModel) -> str:
    lines = [f"{model.source} model details:"]
    if kind == MismatchKind.SENSE:
        lines.append(f"  sense: {'minimize' if model.is_minimize else 'maximize'}")
    elif kind in (MismatchKind.VARIABLE_COUNT, MismatchKind.VARIABLE_NAME,
                  MismatchKind.VARIABLE_TYPE, MismatchKind.VARIABLE_BOUNDS):
        for v in model.variables:
            lines.append(f"  {v.name}: vtype={v.vtype} bounds=[{v.lb}, {v.ub}]")
    elif kind in (MismatchKind.OBJECTIVE_OFFSET, MismatchKind.OBJECTIVE_LINEAR,
                  MismatchKind.OBJECTIVE_QUADRATIC):
        lines.append(f"  offset: {model.obj_offset}")
        lines.append(f"  linear: {model.obj_linear}")
        lines.append(f"  quadratic: {model.obj_quadratic}")
    elif kind in (MismatchKind.CONSTRAINT_COUNT, MismatchKind.CONSTRAINT_NAME,
                  MismatchKind.CONSTRAINT_SENSE, MismatchKind.CONSTRAINT_RHS,
                  MismatchKind.CONSTRAINT_LINEAR, MismatchKind.CONSTRAINT_QUADRATIC):
        for c in model.linear_constraints + model.quadratic_constraints:
            lines.append(f"  {c.name}: {c.sense} rhs={c.rhs} linear={c.linear} quad={c.quadratic}")
    return "\n".join(lines)
