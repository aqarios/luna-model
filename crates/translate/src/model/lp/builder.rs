//! Builds a core [`Model`] from a parsed [`LpProblem`].

use lunamodel_core::{
    ArcEnv, ConstraintCollection, Expression, Model,
    ops::{LmAddAssign, LmMulAssign},
    prelude::{Constraint, LazyBounds, VarRef},
};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bound, Vtype};

use super::reader::{LpExpr, LpProblem};

pub fn build_model(prob: LpProblem) -> LunaModelResult<Model> {
    let mut env = ArcEnv::default();
    let vars = build_vars(&mut env, &prob)?;

    let objective = build_expr(&env, &vars, &prob.objective)?;
    let mut constraints = ConstraintCollection::default();
    for c in &prob.constraints {
        let name = c.name.map(str::to_string);
        let lhs = build_expr(&env, &vars, &c.lhs)?;
        let constraint = Constraint::new(lhs, c.rhs, c.comparator, name.clone())?;
        constraints.add_constraint(constraint, name)?;
    }

    let mut model = Model::with_env(prob.name.map(str::to_string), Some(prob.sense), env);
    model.set_objective(objective, None)?;
    model.set_constraints(constraints)?;
    Ok(model)
}

/// Creates the variables in file order; index `i` is the parser's variable id `i`.
fn build_vars(env: &mut ArcEnv, prob: &LpProblem) -> LunaModelResult<Vec<VarRef>> {
    let mut vars = Vec::with_capacity(prob.vars.len());
    for ((name, &bounds), &vtype) in prob.vars.iter().zip(&prob.bounds).zip(&prob.vtypes) {
        let bounds = match vtype {
            Vtype::Binary => {
                check_binary_bounds(name, bounds)?;
                None
            }
            _ => Some(lazy_bounds(bounds)),
        };
        vars.push(env.insert(name, vtype, bounds)?);
    }
    Ok(vars)
}

/// Applies the LP defaults (`0 <= x <= +inf`); infinite values mean unbounded.
fn lazy_bounds((lower, upper): (Option<f64>, Option<f64>)) -> LazyBounds {
    let bound = |b: f64| match b.is_infinite() {
        true => Bound::Unbounded,
        false => Bound::Bounded(b),
    };
    let lower = bound(lower.unwrap_or(0.0));
    let upper = bound(upper.unwrap_or(f64::INFINITY));
    LazyBounds::new(Some(lower), Some(upper))
}

/// Binary variables may only repeat their implicit bounds `0 <= x <= 1`.
fn check_binary_bounds(
    name: &str,
    (lower, upper): (Option<f64>, Option<f64>),
) -> LunaModelResult<()> {
    for (bound, side, expected) in [(lower, "lower", 0.0), (upper, "upper", 1.0)] {
        if let Some(b) = bound
            && b != expected
        {
            let msg = format!(
                "Invalid {side} bound for binary variable '{name}'. Is {b}, expected {expected}"
            );
            return Err(LunaModelError::Translation(msg.into()));
        }
    }
    Ok(())
}

fn build_expr(env: &ArcEnv, vars: &[VarRef], lp: &LpExpr) -> LunaModelResult<Expression> {
    let mut expr = Expression::constant(env.clone(), lp.offset);

    // `Linear` is a sorted vector, so adding terms in index order only appends.
    let mut linear: Vec<_> = lp
        .linear
        .iter()
        .map(|&(id, c)| (vars[id].id(), c))
        .collect();
    linear.sort_unstable_by_key(|&(var, _)| var);
    for term in linear {
        expr.linear += term;
    }

    // Quadratic terms go through the core operators, which know e.g. that x * x = x for binaries.
    for &(u, v, c) in &lp.quadratic {
        let mut term = Expression::constant(env.clone(), c);
        term.mul_assign(&vars[u])?;
        term.mul_assign(&vars[v])?;
        expr.add_assign(&term)?;
    }
    Ok(expr)
}
