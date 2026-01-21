use lunamodel_core::{
    ArcEnv, Expression, Model,
    ops::{LmAddAssign, LmMulAssign, LmPow},
};
use lunamodel_error::{LunaModelError, LunaModelResult};

use crate::model::lp::reader::LpProblem;

use super::tokenizer::Token;

pub fn build_model(prob: LpProblem) -> LunaModelResult<Model> {
    let env = ArcEnv::default();

    let mut model = Model::with_env(prob.name, Some(prob.sense), env);

    todo!("build_model")
}

fn build_vars(env: &mut ArcEnv, prob: LpProblem) -> LunaModelResult<()> {
    // First, let's create all the variables.
    for varname in prob.vars.into_iter() {
        // check if the binaries contain this variable.
        // else check if the integers/generals contain this variable.
        // else it is a real/continuous variable
        if prob.binaries.contains(&varname) {
            // we need to make sure that the variable (if contained in the bounds)
            // does not have unexpected bounds. Expected bounds a lb = 0, ub = 1.
            if let Some((lb, ub)) = prob.bounds.get(&varname) {
                if let Some(b) = lb
                    && *b != 0.0
                {
                    return Err(LunaModelError::Translation(
                        format!("Invalid lower bound for variable '{varname}'. Is {b}, expected 0")
                            .into(),
                    ));
                }
                if let Some(b) = ub
                    && *b != 1.0
                {
                    return Err(LunaModelError::Translation(
                        format!("Invalid upper bound for variable '{varname}'. Is {b}, expected 1")
                            .into(),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn build_expr(env: &ArcEnv, tokens: Vec<Token>) -> LunaModelResult<Expression> {
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
                                    format!("LP files do not support powering a variable with a power != 2.0, is '{}' a number after the caret (^) symbol").into(),
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
            // Token::LBracket => (),
            // Token::RBracket => (),
            Token::Star | Token::Caret | Token::LBracket | Token::RBracket => (), // just continue
        }
    }

    if !curr_term_is_base {
        final_term.add_assign(&curr_term)?;
    }

    Ok(final_term)
}
