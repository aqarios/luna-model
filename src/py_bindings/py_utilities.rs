use super::py_expr::PyExpression;
use super::py_var::PyVariable;
use super::{py_model::PyModel, py_sol::PySolution};
use crate::core::operations::MulToExpression;
use crate::core::{Constraints, Expression, Samples, Timing};
use crate::translator::model::lp::exprtree::ExprTree;
use either::Either::Left;
use pyo3::FromPyObject;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

static DELIMITER: &str = ", ";

pub fn repr_model(model: &PyModel) -> String {
    let bm = &model.access();
    format!(
        "Model(name={}, sense={}, objective={}, constraints={})",
        bm.name,
        bm.sense,
        repr_objective(&bm.objective),
        repr_constraints(&bm.constraints)
    )
}

pub fn repr_objective(obj: &Expression) -> String {
    // using the LP Translator Expression Tree.
    ExprTree::from_expression_internal(obj)
        .unwrap()
        .optimize()
        .to_repr()
        .replace("[", "")
        .replace("]", "")
}

pub fn repr_constraints(constrs: &Constraints) -> String {
    format!(
        "[{}]",
        constrs
            .iter()
            .map(|(_, c)| c.to_string())
            .collect::<Vec<_>>()
            .join(DELIMITER)
    )
}

pub fn repr_solution(sol: &PySolution) -> String {
    let mut repr = String::from("Solution(");
    repr += &format!("samples={}, ", repr_samples(&sol.access().samples()));
    repr += &format!("obj_values={}, ", repr_opt_vec(&sol.access().obj_values));
    repr += &format!(
        "raw_energies={}, ",
        repr_opt_vec(&sol.access().raw_energies)
    );
    repr += &format!("counts={}, ", repr_numbers(&sol.access().counts));
    repr += &format!(
        "constraints={}, ",
        repr_opt_bools_vec(&sol.access().constraints)
    );
    repr += &format!(
        "variable_bounds={}, ",
        repr_opt_bools_vec(&sol.access().variable_bounds)
    );
    repr += &format!("feasible={}, ", repr_opt_bools(&sol.access().feasible));
    repr += &format!(
        "best_sample_idx={}, ",
        repr_opt_number(&sol.access().best_sample_idx)
    );
    repr += &format!("runtime={}, ", repr_opt_timing(&sol.access().timing));
    repr += &format!("n_samples={}, ", sol.access().n_samples);
    repr += &format!(
        "variable_names={}, ",
        repr_strings(&sol.access().variable_names)
    );
    repr += &format!("sense={}", &sol.access().sense.to_string());
    repr += ")";
    repr
}

pub fn repr_samples(samples: &Samples) -> String {
    format!(
        "[{}]",
        samples
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(DELIMITER)
    )
}

pub fn repr_opt_number<T: ToString>(num: &Option<T>) -> String {
    match num {
        Some(v) => v.to_string(),
        None => "None".to_string(),
    }
}

pub fn repr_opt_vec<T: ToString>(vec: &Option<Vec<T>>) -> String {
    match vec {
        Some(v) => repr_vec(v),
        None => "None".to_string(),
    }
}

pub fn repr_vec<T: ToString>(vec: &Vec<T>) -> String {
    format!(
        "[{}]",
        vec.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(DELIMITER)
    )
}

pub fn repr_numbers<T: ToString>(nums: &Vec<T>) -> String {
    format!(
        "[{}]",
        nums.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(DELIMITER)
    )
}

pub fn repr_opt_bools_vec(bools: &Option<Vec<Vec<bool>>>) -> String {
    match &bools {
        Some(b) => format!(
            "[{}]",
            b.iter()
                .map(|ovb| format!(
                    "[{}]",
                    ovb.iter()
                        .map(|b| repr_bool(b))
                        .collect::<Vec<String>>()
                        .join(DELIMITER)
                ),)
                .collect::<Vec<String>>()
                .join(DELIMITER)
        ),
        None => "None".to_string(),
    }
}

pub fn repr_opt_bools(bools: &Option<Vec<bool>>) -> String {
    match bools {
        Some(b) => format!(
            "[{}]",
            b.iter()
                .map(|b| repr_bool(b))
                .collect::<Vec<_>>()
                .join(DELIMITER)
        ),
        None => "None".to_string(),
    }
}

pub fn repr_bool(b: &bool) -> String {
    let mut bs: Vec<char> = b.to_string().chars().collect();
    bs[0] = bs[0].to_uppercase().nth(0).unwrap();
    bs.into_iter().collect()
}

pub fn repr_opt_timing(timing: &Option<Timing>) -> String {
    match timing {
        None => "None".to_string(),
        Some(t) => repr_timing(t),
    }
}

pub fn repr_timing(timing: &Timing) -> String {
    format!(
        "Timing(start={}, end={}, qpu={})",
        OffsetDateTime::from(timing.start).format(&Rfc3339).unwrap(),
        OffsetDateTime::from(timing.end).format(&Rfc3339).unwrap(),
        timing
            .qpu
            .and_then(|f| Some(f.to_string()))
            .or(Some("None".to_string()))
            .unwrap()
    )
}

pub fn repr_strings(strs: &Vec<String>) -> String {
    format!("[{}]", strs.join(DELIMITER))
}

#[derive(FromPyObject)]
pub enum Replacement {
    Expr(PyExpression),
    Var(PyVariable),
}

impl Replacement {
    pub fn as_expr(self) -> PyExpression {
        match self {
            Replacement::Expr(expr) => expr,
            Replacement::Var(var) => PyExpression(Left(var.0.mul(1.0))),
        }
    }
}
