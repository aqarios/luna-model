use lunamodel_core::{
    ArcEnv, ConstraintCollection, Expression, Model,
    ops::{LmAddAssign, LmMulAssign, LmSubAssign},
    prelude::{Constraint, LazyBounds},
};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Bound, Vtype};
use std::collections::HashMap;

use super::reader::{MpsConstraint, MpsProblem};

pub fn build_model(prob: MpsProblem) -> LunaModelResult<Model> {
    let mut env = ArcEnv::default();
    build_vars(&mut env, &prob)?;

    let objective = build_objective(&env, &prob)?;
    let constraints = build_constraints(&env, prob.constraints, &prob.constraint_quadratic)?;

    let mut model = Model::with_env(prob.name, Some(prob.sense), env);
    model.objective = objective;
    model.constraints = constraints;

    Ok(model)
}

fn build_vars(env: &mut ArcEnv, prob: &MpsProblem) -> LunaModelResult<()> {
    let mut sorted_vars: Vec<_> = prob.vars.iter().collect();
    sorted_vars.sort();

    for var_name in sorted_vars {
        if prob.binaries.contains(var_name) {
            let bounds = prob
                .bounds
                .get(var_name)
                .and_then(|opt| opt.as_ref().copied());
            check_binary_bounds(bounds, var_name)?;
            env.write_arc().insert(var_name, Vtype::Binary, None)?;
        } else if prob.integers.contains(var_name) {
            let bounds = prob
                .bounds
                .get(var_name)
                .and_then(|opt| opt.as_ref().copied());
            let is_binary_bounds = check_integer_bounds(bounds, var_name)?;
            if is_binary_bounds {
                env.write_arc().insert(var_name, Vtype::Binary, None)?;
            } else {
                env.write_arc().insert(var_name, Vtype::Integer, bounds)?;
            }
        } else {
            let bounds = prob
                .bounds
                .get(var_name)
                .and_then(|opt| opt.as_ref().copied());
            env.write_arc().insert(var_name, Vtype::Real, bounds)?;
        }
    }

    Ok(())
}

fn check_integer_bounds(bounds: Option<LazyBounds>, var_name: &str) -> LunaModelResult<bool> {
    if let Some(LazyBounds { lower, upper }) = bounds {
        check_int_bound(lower, var_name)?;
        check_int_bound(upper, var_name)?;
        if let Some(Bound::Bounded(l)) = lower
            && let Some(Bound::Bounded(u)) = upper
            && l == 0.0
            && u == 1.0
        {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

fn check_int_bound(bound: Option<Bound>, var_name: &str) -> LunaModelResult<()> {
    if let Some(Bound::Bounded(val)) = bound {
        if val.fract().abs() >= f64::EPSILON {
            return Err(LunaModelError::Translation(
                format!(
                    "Invalid bound for integer variable '{}'. Is {}, expected integer",
                    var_name, val
                )
                .into(),
            ));
        }
        Ok(())
    } else {
        Ok(())
    }
}

fn check_binary_bounds(bounds: Option<LazyBounds>, var_name: &str) -> LunaModelResult<()> {
    if let Some(LazyBounds { lower, upper }) = bounds {
        if let Some(Bound::Bounded(b)) = lower
            && (b - 0.0).abs() >= f64::EPSILON {
                return Err(LunaModelError::Translation(
                    format!(
                        "Invalid lower bound for binary variable '{}'. Is {}, expected 0",
                        var_name, b
                    )
                    .into(),
                ));
            }
        if let Some(Bound::Bounded(b)) = upper
            && (b - 1.0).abs() >= f64::EPSILON {
                return Err(LunaModelError::Translation(
                    format!(
                        "Invalid upper bound for binary variable '{}'. Is {}, expected 1",
                        var_name, b
                    )
                    .into(),
                ));
            }
    }
    Ok(())
}

fn build_objective(env: &ArcEnv, prob: &MpsProblem) -> LunaModelResult<Expression> {
    let mut objective = Expression::empty(env.clone());

    // Add constant
    objective.sub_assign(prob.objective_constant)?;

    // Add linear terms
    for (var_name, coef) in &prob.objective {
        let var = env.lookup(var_name)?;
        let mut term = Expression::constant(env.clone(), *coef);
        term.mul_assign(var)?;
        objective.add_assign(&term)?;
    }

    // Add quadratic terms from QUADOBJ
    for (var1_name, var2_name, coef) in &prob.objective_quadratic {
        let var1 = env.lookup(var1_name)?;
        let var2 = env.lookup(var2_name)?;

        // Create quadratic term: coef * var1 * var2
        let mut quad_term = Expression::constant(env.clone(), *coef);
        quad_term.mul_assign(var1)?;
        quad_term.mul_assign(var2)?;
        objective.add_assign(&quad_term)?;
    }

    Ok(objective)
}

fn build_constraints(
    env: &ArcEnv,
    constraints: Vec<MpsConstraint>,
    constraint_quadratic: &HashMap<String, Vec<(String, String, Bias)>>,
) -> LunaModelResult<ConstraintCollection> {
    let mut collection = ConstraintCollection::default();

    for mps_constraint in constraints {
        let quad_terms = constraint_quadratic.get(&mps_constraint.name);
        let constraint = build_constraint(env, mps_constraint, quad_terms)?;
        collection.add_constraint(constraint, None)?;
    }

    Ok(collection)
}

fn build_constraint(
    env: &ArcEnv,
    mps_constraint: MpsConstraint,
    quad_terms: Option<&Vec<(String, String, Bias)>>,
) -> LunaModelResult<Constraint> {
    let mut lhs = Expression::empty(env.clone());

    // Add linear terms
    for (var_name, coef) in &mps_constraint.coefficients {
        let var = env.lookup(var_name)?;
        let mut term = Expression::constant(env.clone(), *coef);
        term.mul_assign(var)?;
        lhs.add_assign(&term)?;
    }

    // Add quadratic terms from QCMATRIX
    if let Some(qterms) = quad_terms {
        for (var1_name, var2_name, coef) in qterms {
            let var1 = env.lookup(var1_name)?;
            let var2 = env.lookup(var2_name)?;

            // Create quadratic term: coef * var1 * var2
            let mut quad_term = Expression::constant(env.clone(), *coef);
            quad_term.mul_assign(var1)?;
            quad_term.mul_assign(var2)?;
            lhs.add_assign(&quad_term)?;
        }
    }

    let comparator = mps_constraint.row_type.to_comparator()?;
    let rhs = mps_constraint.rhs;

    Constraint::new(lhs, rhs, comparator, Some(mps_constraint.name))
}
