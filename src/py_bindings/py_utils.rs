use super::{py_model::PyModel, py_sol::PySolution};
use crate::core::{Constraints, Expression, Samples, Timing};
use crate::translator::model::lp::exprtree::ExprTree;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

static DELIMITER: &str = ", ";

pub fn repr_model(model: &PyModel) -> String {
    let bm = model.borrow();
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
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(DELIMITER)
    )
}

pub fn repr_solution(sol: &PySolution) -> String {
    let mut repr = String::from("Solution(");
    repr += &format!("samples={}, ", repr_samples(&sol.samples()));
    repr += &format!("obj_values={}, ", repr_opt_numbers(&sol.obj_values));
    repr += &format!("raw_energies={}, ", repr_opt_numbers(&sol.raw_energies));
    repr += &format!("counts={}, ", repr_numbers(&sol.counts));
    repr += &format!("constraints={}, ", repr_opt_bools_vec(&sol.constraints));
    repr += &format!(
        "variable_bounds={}, ",
        repr_opt_bools_vec(&sol.variable_bounds)
    );
    repr += &format!("feasible={}, ", repr_opt_bools(&sol.feasible));
    repr += &format!(
        "best_sample_idx={}, ",
        repr_opt_number(&sol.best_sample_idx)
    );
    repr += &format!("runtime={}, ", repr_opt_timing(&sol.timing));
    repr += &format!("n_samples={}, ", sol.n_samples);
    repr += &format!("variable_names={}", repr_strings(&sol.variable_names));
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

pub fn repr_opt_numbers<T: ToString>(nums: &Vec<Option<T>>) -> String {
    format!(
        "[{}]",
        nums.iter()
            .map(|n| repr_opt_number(n))
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

pub fn repr_numbers<T: ToString>(nums: &Vec<T>) -> String {
    format!(
        "[{}]",
        nums.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(DELIMITER)
    )
}

pub fn repr_opt_bools_vec(bools: &Vec<Option<Vec<bool>>>) -> String {
    format!(
        "[{}]",
        bools
            .iter()
            .map(|ovb| match ovb {
                Some(v) => format!(
                    "[{}]",
                    v.iter()
                        .map(|b| repr_bool(b))
                        .collect::<Vec<String>>()
                        .join(DELIMITER)
                ),
                None => "None".to_string(),
            })
            .collect::<Vec<String>>()
            .join(DELIMITER)
    )
}

pub fn repr_opt_bools(bools: &Vec<Option<bool>>) -> String {
    format!(
        "[{}]",
        bools
            .iter()
            .map(|b| repr_opt_bool(b))
            .collect::<Vec<_>>()
            .join(DELIMITER)
    )
}

pub fn repr_opt_bool(b: &Option<bool>) -> String {
    match b {
        Some(b) => repr_bool(b),
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
