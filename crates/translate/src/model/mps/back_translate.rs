//! Writes a core [`Model`] as free-format MPS text.

use std::{
    fmt::{self, Write},
    path::PathBuf,
};

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bias, Bound, Comparator, Sense, Vtype};

use super::MpsTranslator;
use crate::model::writer::{Var, Vars, check_names, output, prepare, quadratic};

impl MpsTranslator {
    /// Translates a model into MPS text, written to `filepath` or returned.
    pub fn back_translate(
        model: &Model,
        filepath: Option<PathBuf>,
    ) -> LunaModelResult<Option<String>> {
        let vars = prepare(model, "MPS")?;
        // Fields are separated by whitespace.
        check_names(model, &vars, "MPS", |name| {
            name.contains(char::is_whitespace)
                .then_some("it contains whitespace")
        })?;
        let mut out = String::new();
        write_mps(&mut out, model, &vars).expect("writing to a String cannot fail");
        output(out, filepath)
    }
}

fn write_mps(out: &mut String, model: &Model, vars: &Vars) -> fmt::Result {
    let objective = objective_row(model);
    let sense = match model.sense {
        Sense::Min => "MIN",
        Sense::Max => "MAX",
    };
    writeln!(out, "NAME          {}", model.name)?;
    writeln!(out, "OBJSENSE\n {sense}")?;
    writeln!(out, "ROWS\n N  {objective}")?;
    for (name, c) in model.constraints().iter() {
        let row_type = match c.comparator {
            Comparator::Le => "L",
            Comparator::Ge => "G",
            Comparator::Eq => "E",
        };
        writeln!(out, " {row_type}  {name}")?;
    }

    write_columns(out, model, vars, &objective)?;

    writeln!(out, "RHS")?;
    for (name, c) in model.constraints().iter() {
        writeln!(
            out,
            "    RHS1      {name}  {}",
            number(c.rhs - c.lhs.offset)
        )?;
    }
    let offset = model.objective().offset;
    if offset != 0.0 {
        // The RHS of the objective row is the negated constant.
        writeln!(out, "    RHS1      {objective}  {}", number(-offset))?;
    }

    writeln!(out, "BOUNDS")?;
    for var in &vars.list {
        write_bounds(out, var)?;
    }
    write_quadratic(out, model, vars)?;
    out.push_str("ENDATA");
    Ok(())
}

/// The objective row is called `OBJ`, unless a constraint already has that name.
fn objective_row(model: &Model) -> String {
    let mut name = String::from("OBJ");
    while model.constraints().get(&name).is_ok() {
        name.push('_');
    }
    name
}

/// One block per variable in model order; integer variables sit between markers.
fn write_columns(out: &mut String, model: &Model, vars: &Vars, objective: &str) -> fmt::Result {
    // COLUMNS lists the matrix by column, the model stores it by row. Collect all
    // entries and sort them by column; the stable sort keeps each column's rows in order.
    let mut entries: Vec<(usize, &str, Bias)> = Vec::new();
    let objective_entries = model.objective().linear.iter();
    entries.extend(objective_entries.map(|(idx, c)| (vars.position(idx), objective, c)));
    for (name, c) in model.constraints().iter() {
        let row_entries = c.lhs.linear.iter();
        entries.extend(row_entries.map(|(idx, v)| (vars.position(idx), name.as_str(), v)));
    }
    entries.sort_by_key(|&(pos, _, _)| pos);
    let mut entries = entries.into_iter().peekable();

    writeln!(out, "COLUMNS")?;
    let mut in_markers = false;
    for (pos, var) in vars.list.iter().enumerate() {
        let integer = var.vtype == Vtype::Integer;
        if integer != in_markers {
            let marker = if integer { "'INTORG'" } else { "'INTEND'" };
            writeln!(out, "    MARK0000  'MARKER'                 {marker}")?;
            in_markers = integer;
        }
        let mut written = false;
        while let Some((_, row, value)) = entries.next_if(|&(p, _, _)| p == pos) {
            writeln!(out, "    {}  {row}  {}", var.name, number(value))?;
            written = true;
        }
        // A column without coefficients must still appear, or the variable is lost.
        if !written {
            writeln!(out, "    {}  {objective}  {}", var.name, number(0.0))?;
        }
    }
    if in_markers {
        writeln!(out, "    MARK0000  'MARKER'                 'INTEND'")?;
    }
    Ok(())
}

fn write_bounds(out: &mut String, var: &Var) -> fmt::Result {
    let mut bound = |kind: &str, value: Option<f64>| match value {
        Some(value) => writeln!(out, " {kind} BND1      {}  {}", var.name, number(value)),
        None => writeln!(out, " {kind} BND1      {}", var.name),
    };
    match (var.vtype, var.lower, var.upper) {
        (Vtype::Binary, _, _) => bound("BV", None),
        (_, Bound::Unbounded, Bound::Unbounded) => bound("FR", None),
        // An integer column always gets a bound entry; without one it would read back as binary.
        (Vtype::Integer, Bound::Bounded(l), Bound::Bounded(u)) if l == u => bound("FX", Some(l)),
        (Vtype::Integer, lower, upper) => {
            match lower {
                Bound::Bounded(l) => bound("LI", Some(l))?,
                Bound::Unbounded => bound("MI", None)?,
            }
            match upper {
                Bound::Bounded(u) => bound("UI", Some(u)),
                Bound::Unbounded => Ok(()),
            }
        }
        (_, lower, upper) => {
            let negative_upper = matches!(upper, Bound::Bounded(u) if u < 0.0);
            match lower {
                Bound::Unbounded => bound("MI", None)?,
                // 0 is the default, but some readers (e.g. CPLEX) lower it to -inf
                // below a negative upper bound, so write it then.
                Bound::Bounded(l) if l != 0.0 || negative_upper => bound("LO", Some(l))?,
                Bound::Bounded(_) => (),
            }
            match upper {
                Bound::Bounded(u) => bound("UP", Some(u)),
                Bound::Unbounded => Ok(()),
            }
        }
    }
}

fn write_quadratic(out: &mut String, model: &Model, vars: &Vars) -> fmt::Result {
    // QUADOBJ holds one triangle of Q in 1/2 x'Qx, so a diagonal entry is twice the coefficient.
    let mut terms = quadratic(model.objective()).peekable();
    if terms.peek().is_some() {
        writeln!(out, "QUADOBJ")?;
        for (u, v, coef) in terms {
            let entry = if u == v { 2.0 * coef } else { coef };
            writeln!(
                out,
                "    {}  {}  {}",
                vars.name(u),
                vars.name(v),
                number(entry)
            )?;
        }
    }

    // QCMATRIX holds all of Q in x'Qx, so an off-diagonal coefficient is split over both entries.
    for (name, c) in model.constraints().iter() {
        let mut terms = quadratic(&c.lhs).peekable();
        if terms.peek().is_none() {
            continue;
        }
        writeln!(out, "QCMATRIX   {name}")?;
        for (u, v, coef) in terms {
            let (u, v) = (vars.name(u), vars.name(v));
            if u == v {
                writeln!(out, "    {u}  {v}  {}", number(coef))?;
            } else {
                writeln!(out, "    {u}  {v}  {}", number(coef / 2.0))?;
                writeln!(out, "    {v}  {u}  {}", number(coef / 2.0))?;
            }
        }
    }
    Ok(())
}

/// Formats a number without allocating; whole numbers keep a `.0`, e.g. `3.0`.
fn number(n: Bias) -> Number {
    Number(n)
}

struct Number(Bias);

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.fract() == 0.0 {
            true => write!(f, "{:.1}", self.0),
            false => write!(f, "{}", self.0),
        }
    }
}
