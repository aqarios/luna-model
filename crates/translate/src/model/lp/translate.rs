// use lp_parser_rs::{
//     model::{
//         Coefficient, ComparisonOp, Constraint as LpConstraint, Objective, Sense as LpSense,
//         VariableType,
//     },
//     problem::LpProblem,
// };

use lunamodel_core::{Expression, Model, ops::LmAddAssign, prelude::Constraint};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Comparator, Sense, Vtype};

use super::LpTranslator;

impl LpTranslator {
    pub fn translate(content: String) -> LunaModelResult<Model> {
        dbg!(&content);
        unimplemented!("implmenet lp.translate");

        // let problem = LpProblem::parse(&content);
        // //.map_err(|e| LunaModelError::Translation(e.to_string().into()))?;
        // //
        // let problem = match problem {
        //     Ok(p) => p,
        //     Err(e) => {
        //         dbg!(e);
        //         todo!()
        //     }
        // };

        // let model_name = problem.name.map(|n| n.to_string());
        // let model_sense = match problem.sense {
        //     LpSense::Minimize => Sense::Min,
        //     LpSense::Maximize => Sense::Max,
        // };
        // let mut model = Model::new(model_name, Some(model_sense));
        // for (name, var) in problem.variables {
        //     let vtype = match var.var_type {
        //         VariableType::Binary => Vtype::Binary,
        //         VariableType::Integer => Vtype::Integer,
        //         // VariableType:: => Vtype::Integer,
        //         _ => panic!("encountered vartype '{}' for var '{}'", var.var_type, name),
        //     };
        //     // TODO: bounds.
        //     _ = model.add_var(name, vtype, None);
        // }
        // if problem.objectives.len() != 1 {
        //     return Err(LunaModelError::Translation(
        //         "Lp file must contain exactly one objective".into(),
        //     ));
        // }
        // for (_, Objective { coefficients, .. }) in problem.objectives.iter() {
        //     coefficients_to_expr(&mut model.objective, coefficients)?;
        // }

        // for (_, constr) in problem.constraints {
        //     match constr {
        //         LpConstraint::Standard {
        //             name,
        //             coefficients,
        //             operator,
        //             rhs,
        //         } => {
        //             let comparator = match operator {
        //                 ComparisonOp::EQ => Comparator::Eq,
        //                 ComparisonOp::LTE => Comparator::Le,
        //                 ComparisonOp::GTE => Comparator::Le,
        //                 ComparisonOp::GT | ComparisonOp::LT => panic!("not supported"),
        //             };
        //             let mut lhs = Expression::empty(model.environment.clone());
        //             coefficients_to_expr(&mut lhs, &coefficients)?;
        //             let constr = Constraint::new(lhs, rhs, comparator, Some(name.to_string()))?;
        //             model
        //                 .constraints
        //                 .add_constraint(constr, Some(name.to_string()))?;
        //         }
        //         _ => panic!("not allowed"),
        //     }
        // }
        //
        // Ok(model)
    }
}

// fn coefficients_to_expr<'a>(
//     expr: &mut Expression,
//     coefs: &Vec<Coefficient<'a>>,
// ) -> LunaModelResult<()> {
//     for Coefficient { name, value } in coefs {
//         let var = &expr.env.lookup(name)?;
//         expr.add_assign((var * *value)?)?;
//     }
//     Ok(())
// }
