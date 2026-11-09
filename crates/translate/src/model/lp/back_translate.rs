//! Writes a core [`Model`] as LP text.

use std::{
    fmt::{self, Write},
    path::PathBuf,
};

use lunamodel_core::{Expression, Model};
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bound, Comparator, Vtype};

use super::{LpTranslator, reader::is_keyword, tokenizer::is_name};
use crate::model::writer::{Vars, check_names, output, prepare, quadratic};

impl LpTranslator {
    /// Translates a model into LP text, written to `filepath` or returned.
    pub fn back_translate(
        model: &Model,
        filepath: Option<PathBuf>,
    ) -> LunaModelResult<Option<String>> {
        let vars = prepare(model, "LP")?;
        check_names(model, &vars, "LP", |name| {
            if !is_name(name) {
                Some("it contains characters LP does not allow")
            } else if is_keyword(name) {
                Some("it is an LP keyword")
            } else {
                None
            }
        })?;
        let mut out = String::new();
        write_lp(&mut out, model, &vars).expect("writing to a String cannot fail");
        output(out, filepath)
    }
}

/// Lines are wrapped before this length, as some LP readers limit it.
const MAX_LINE: usize = 88;

fn write_lp(out: &mut String, model: &Model, vars: &Vars) -> fmt::Result {
    writeln!(out, "\\ Model {}", model.name)?;
    writeln!(out, "\\ Problem name: {}", model.name)?;
    writeln!(out, "\n{}", model.sense)?;
    let mut line = Line::start(out);
    expression(&mut line, model.objective(), vars, true)?;
    line.end();

    writeln!(out, "Subject To")?;
    for (name, c) in model.constraints().iter() {
        let comparator = match c.comparator {
            Comparator::Le => "<=",
            Comparator::Ge => ">=",
            Comparator::Eq => "=",
        };
        let mut line = Line::start(out);
        line.push(format_args!("{name}:"))?;
        expression(&mut line, &c.lhs, vars, false)?;
        // LP allows no constant on the left-hand side.
        line.push(format_args!("{comparator} {}", c.rhs - c.lhs.offset))?;
        line.end();
    }

    write_bounds(out, vars)?;
    write_var_list(out, "Binaries", vars, Vtype::Binary)?;
    write_var_list(out, "Generals", vars, Vtype::Integer)?;
    out.push_str("End");
    Ok(())
}

/// Writes `linear + [ quadratic ] + constant`.
fn expression(line: &mut Line, expr: &Expression, vars: &Vars, is_objective: bool) -> fmt::Result {
    let mut first = true;
    if is_objective {
        // Every variable appears in the objective, in model order, so reading the file
        // back gives the same variable order.
        for var in &vars.list {
            line.term(first, expr.linear[var.idx], format_args!(" {}", var.name))?;
            first = false;
        }
    } else {
        for (idx, coef) in expr.linear.iter() {
            line.term(first, coef, format_args!(" {}", vars.name(idx)))?;
            first = false;
        }
    }

    let mut terms = quadratic(expr).peekable();
    if terms.peek().is_some() {
        line.push(format_args!("{}", if first { "[" } else { "+ [" }))?;
        // The objective is written as [ 2 Q ] / 2, as LP readers expect.
        let factor = if is_objective { 2.0 } else { 1.0 };
        let mut first_in_bracket = true;
        for (u, v, coef) in terms {
            let product = format_args!(" {} * {}", vars.name(u), vars.name(v));
            line.term(first_in_bracket, factor * coef, product)?;
            first_in_bracket = false;
        }
        line.push(format_args!("{}", if is_objective { "] / 2" } else { "]" }))?;
        first = false;
    }

    if is_objective && expr.offset != 0.0 {
        line.term(first, expr.offset, format_args!(""))?;
    }
    Ok(())
}

fn write_bounds(out: &mut String, vars: &Vars) -> fmt::Result {
    let mut bounded = vars
        .list
        .iter()
        .filter(|v| v.vtype != Vtype::Binary)
        .peekable();
    if bounded.peek().is_none() {
        return Ok(());
    }
    writeln!(out, "Bounds")?;
    for var in bounded {
        let name = &var.name;
        match (var.lower, var.upper) {
            (Bound::Unbounded, Bound::Unbounded) => writeln!(out, " {name} free")?,
            (Bound::Bounded(l), Bound::Unbounded) => writeln!(out, " {name} >= {l}")?,
            (Bound::Unbounded, Bound::Bounded(u)) => writeln!(out, " -inf <= {name} <= {u}")?,
            (Bound::Bounded(l), Bound::Bounded(u)) => writeln!(out, " {l} <= {name} <= {u}")?,
        }
    }
    Ok(())
}

fn write_var_list(out: &mut String, header: &str, vars: &Vars, vtype: Vtype) -> fmt::Result {
    let mut names = vars.list.iter().filter(|v| v.vtype == vtype).peekable();
    if names.peek().is_none() {
        return Ok(());
    }
    writeln!(out, "{header}")?;
    let mut line = Line::start(out);
    for var in names {
        line.push(format_args!("{}", var.name))?;
    }
    line.end();
    Ok(())
}

/// Writes space-separated tokens, starting a new line before [`MAX_LINE`].
struct Line<'a> {
    out: &'a mut String,
    len: usize,
    token: String,
}

impl<'a> Line<'a> {
    fn start(out: &'a mut String) -> Self {
        Self {
            out,
            len: 0,
            token: String::new(),
        }
    }

    fn push(&mut self, token: fmt::Arguments) -> fmt::Result {
        self.token.clear();
        self.token.write_fmt(token)?;
        if self.len > 0 && self.len + 1 + self.token.len() > MAX_LINE {
            self.out.push('\n');
            self.len = 0;
        }
        self.out.push(' ');
        self.out.push_str(&self.token);
        self.len += 1 + self.token.len();
        Ok(())
    }

    /// A term `coef name`; every term but the first gets an explicit sign.
    fn term(&mut self, first: bool, coef: f64, name: fmt::Arguments) -> fmt::Result {
        match (first, coef < 0.0) {
            (true, _) => self.push(format_args!("{coef}{name}")),
            (false, false) => self.push(format_args!("+ {coef}{name}")),
            (false, true) => self.push(format_args!("- {}{name}", -coef)),
        }
    }

    fn end(self) {
        self.out.push('\n');
    }
}
