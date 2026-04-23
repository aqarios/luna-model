use lunamodel_core::{
    ArcEnv, ConstraintCollection, Expression, Model,
    ops::{LmAddAssign, LmMulAssign, LmPow},
    prelude::{Constraint, LazyBounds},
};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bound, Vtype};

use crate::model::lp::reader::{LpConstraint, LpExpression, LpProblem};

use super::tokenizer::Token;

pub fn build_model(prob: LpProblem) -> LunaModelResult<Model> {
    let mut env = ArcEnv::default();
    build_vars(&mut env, &prob)?;

    let expr = build_expr(&env, prob.objective)?;
    let constrs = build_constraints(&env, prob.constraints)?;
    let mut model = Model::with_env(prob.name, Some(prob.sense), env);
    model.objective = expr;
    model.constraints = constrs;
    Ok(model)
}

fn build_vars(env: &mut ArcEnv, prob: &LpProblem) -> LunaModelResult<()> {
    // First, let's create all the variables.
    let mut sortedvars: Vec<_> = prob.vars.iter().collect();
    sortedvars.sort();
    for varname in sortedvars {
        // check if the binaries contain this variable.
        // else check if the integers/generals contain this variable.
        // else it is a real/continuous variable
        if prob.binaries.contains(varname) {
            // we need to make sure that the variable (if contained in the bounds)
            // does not have unexpected bounds. Expected bounds are lb = 0, ub = 1.
            check_binary_bounds(prob.bounds.get(varname), varname)?;
            // all fine, so we can add this binary variable to the environment.
            env.write_arc().insert(varname, Vtype::Binary, None)?;
        } else if prob.generals.contains(varname) {
            // we need to make sure that the variable (if contained in the bounds)
            // does not have unexpected bounds. Expected bounds are lb = int, ub = int.
            let bounds = check_integer_bounds(prob.bounds.get(varname), varname)?;
            // all fine, so we can add this binary variable to the environment.
            env.write_arc()
                .insert(varname, Vtype::Integer, Some(bounds))?;
        } else {
            // We have a continuous/real variable.
            let bounds = extract_real_bounds(prob.bounds.get(varname));
            env.write_arc().insert(varname, Vtype::Real, Some(bounds))?;
        }
    }

    Ok(())
}

fn extract_real_bounds(bounds: Option<&(Option<f64>, Option<f64>)>) -> LazyBounds {
    match bounds {
        Some((lb, ub)) => {
            let lb = match lb {
                Some(lb) => Bound::Bounded(*lb),
                None => Bound::Bounded(0.0),
            };
            let ub = match ub {
                Some(ub) => Bound::Bounded(*ub),
                None => Bound::Unbounded,
            };
            LazyBounds::new(Some(lb), Some(ub))
        }
        None => LazyBounds::new(Some(Bound::Bounded(0.0)), Some(Bound::Unbounded)),
    }
}

fn check_integer_bounds(
    bounds: Option<&(Option<f64>, Option<f64>)>,
    varname: &str,
) -> LunaModelResult<LazyBounds> {
    match bounds {
        Some((lb, ub)) => {
            let lb = check_int_bound(lb, varname)?.unwrap_or(Bound::Bounded(0.0));
            let ub = check_int_bound(ub, varname)?.unwrap_or(Bound::Unbounded);
            Ok(LazyBounds::new(Some(lb), Some(ub)))
        }
        None => Ok(LazyBounds::new(
            Some(Bound::Bounded(0.0)),
            Some(Bound::Unbounded),
        )),
    }
}

fn check_int_bound(bound: &Option<f64>, varname: &str) -> LunaModelResult<Option<Bound>> {
    if let Some(l) = bound {
        // TODO(team): Maybe use the near integral fun.
        if l.fract() >= 0.0 + f64::EPSILON {
            return Err(LunaModelError::Translation(
                format!("Invalid bound for variable '{varname}'. Is {l}, expected integer").into(),
            ));
        }
        Ok(Some(Bound::Bounded(*l)))
    } else {
        Ok(None)
    }
}

fn check_binary_bounds(
    bounds: Option<&(Option<f64>, Option<f64>)>,
    varname: &str,
) -> LunaModelResult<()> {
    if let Some((lb, ub)) = bounds {
        if let Some(b) = lb
            && *b != 0.0
        {
            return Err(LunaModelError::Translation(
                format!("Invalid lower bound for binary variable '{varname}'. Is {b}, expected 0")
                    .into(),
            ));
        }
        if let Some(b) = ub
            && *b != 1.0
        {
            return Err(LunaModelError::Translation(
                format!("Invalid upper bound for binary variable '{varname}'. Is {b}, expected 1")
                    .into(),
            ));
        }
    }
    Ok(())
}

fn build_constraints(
    env: &ArcEnv,
    constrs: Vec<LpConstraint>,
) -> LunaModelResult<ConstraintCollection> {
    let mut collection = ConstraintCollection::default();
    for c in constrs {
        let cname = c.name.clone();
        let constr = build_constraint(c, env)?;
        collection.add_constraint(constr, cname)?;
    }
    Ok(collection)
}

fn build_constraint(constr: LpConstraint, env: &ArcEnv) -> LunaModelResult<Constraint> {
    let lhs = build_expr(env, constr.lhs)?;
    Constraint::new(lhs, constr.rhs, constr.comparator, constr.name)
}

fn build_expr(env: &ArcEnv, obj: LpExpression) -> LunaModelResult<Expression> {
    let tokens = obj.0;
    build_expr_inner(tokens, env)
}

fn build_expr_inner(tokens: Vec<Token>, env: &ArcEnv) -> LunaModelResult<Expression> {
    let mut final_term = Expression::empty(env.clone());
    let mut curr_term = Expression::constant(env.clone(), 1.0);
    let mut curr_term_is_base = true;

    let mut tks = tokens.iter().peekable();
    while let Some(token) = tks.next() {
        match token {
            Token::Plus => {
                if !curr_term_is_base {
                    final_term.add_assign(&curr_term)?;
                }
                // THE NEXT TERM HAS A POSITIVE SIGN.
                curr_term = Expression::constant(env.clone(), 1.0);
                curr_term_is_base = true;
            }
            Token::Minus => {
                if !curr_term_is_base {
                    final_term.add_assign(&curr_term)?;
                }
                // THE NEXT TERM HAS A NEGATIVE SIGN.
                curr_term = Expression::constant(env.clone(), -1.0);
                curr_term_is_base = true;
            }
            Token::Number(coef) => {
                // curr_items.push(Item::Number(coef))
                // check if the next item would be a caret.
                if let Some(next) = tks.peek()
                    && **next == Token::Caret
                {
                    _ = tks.next();
                    match tks.next() {
                        Some(Token::Number(pow)) => curr_term.mul_assign(coef.powf(*pow))?,
                        _ => {
                            return Err(LunaModelError::Translation(
                                "expected a number after the caret (^) symbol".into(),
                            ));
                        }
                    }
                } else {
                    // else
                    curr_term.mul_assign(coef)?;
                }
                curr_term_is_base = false;
            }
            Token::Variable(varname) => {
                let var = env.lookup(varname)?;
                if let Some(next) = tks.peek()
                    && **next == Token::Caret
                {
                    _ = tks.next();
                    match tks.next() {
                        Some(Token::Number(pow)) => {
                            if *pow != 2.0 {
                                return Err(LunaModelError::Translation(
                                    format!("LP files do not support powering a variable with a power != 2.0, is '{}' a number after the caret (^) symbol", *pow).into(),
                                ));
                            }
                            curr_term.mul_assign(var.pow(*pow as usize)?)?
                        }
                        _ => {
                            return Err(LunaModelError::Translation(
                                "expected a number after the caret (^) symbol".into(),
                            ));
                        }
                    }
                } else {
                    // else
                    curr_term.mul_assign(var)?;
                }
                curr_term_is_base = false;
            }
            Token::LBracket => {
                // collect tokens until Token::RBracket.
                let mut bracket_tokens = Vec::new();
                for inner in tks.by_ref() {
                    if *inner == Token::RBracket {
                        break;
                    }
                    bracket_tokens.push(inner.clone());
                }
                curr_term.mul_assign(build_expr_inner(bracket_tokens, env)?)?;
                curr_term_is_base = false;
            }
            // Token::RBracket => in_bracket = false,
            // Token::Star | Token::Caret | Token::LBracket | Token::RBracket => (), // just continue
            Token::Star | Token::Caret | Token::RBracket => (), // just continue
        }
    }

    if !curr_term_is_base {
        final_term.add_assign(&curr_term)?;
    }

    Ok(final_term)
}
