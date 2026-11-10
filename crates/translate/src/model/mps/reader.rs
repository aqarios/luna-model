//! Parses MPS text, in free or fixed format, into a [`Problem`].

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use lunamodel_error::LunaModelResult;
use lunamodel_types::{Comparator, Sense, Vtype};

use crate::model::problem::{self, Constraint, Expr, Problem};

/// Reads free format first and falls back to fixed format, which allows spaces in names.
/// If both fail, the error of the format that got further through the file is reported.
pub fn read_mps(src: &str) -> LunaModelResult<Problem<'_>> {
    let free = match read(src, Format::Free) {
        Ok(problem) => return Ok(problem),
        Err(failure) => failure,
    };
    match read(src, Format::Fixed) {
        Ok(problem) => Ok(problem),
        Err(fixed) => {
            let (line, msg) = if fixed.0 > free.0 { fixed } else { free };
            Err(problem::error(line, msg))
        }
    }
}

#[derive(Clone, Copy)]
enum Format {
    Free,
    Fixed,
}

/// Start and end (0-based, exclusive) of the six fields of a fixed-format line.
const FIXED_FIELDS: [(usize, usize); 6] = [(1, 3), (4, 12), (14, 22), (24, 36), (39, 47), (49, 61)];

/// Splits a data line into its non-empty fields, so an omitted set name looks the same in both formats.
fn split_fields<'a>(line: &'a str, format: Format, fields: &mut Vec<&'a str>) {
    fields.clear();
    match format {
        Format::Free => fields.extend(line.split_whitespace()),
        Format::Fixed => fields.extend(
            FIXED_FIELDS
                .iter()
                .filter_map(|&(start, end)| line.get(start..end.min(line.len())))
                .map(str::trim)
                .filter(|field| !field.is_empty()),
        ),
    }
}

/// Reads the whole file in one format; an error comes with its line number.
fn read(src: &str, format: Format) -> Result<Problem<'_>, (usize, String)> {
    let mut reader = Reader {
        problem: Problem::default(),
        rows: HashMap::new(),
        has_objective: false,
        ranges: Vec::new(),
        bounded: HashSet::new(),
        integer_block: false,
        last_column: None,
    };
    let mut section = None;
    let mut fields = Vec::with_capacity(FIXED_FIELDS.len());

    for (i, line) in src.lines().enumerate() {
        let at_line = |msg| (i + 1, msg);
        if line.trim().is_empty() || line.starts_with('*') {
            continue;
        }
        // Section headers start in column 1, data lines are indented.
        if !line.starts_with([' ', '\t']) {
            match reader.header(line).map_err(at_line)? {
                Section::End => break,
                header => section = Some(header),
            }
            continue;
        }
        let Some(section) = section else {
            return Err(at_line("data before the first section".into()));
        };
        split_fields(line, format, &mut fields);
        reader.data(section, &fields).map_err(at_line)?;
    }
    Ok(reader.finish())
}

#[derive(Clone, Copy)]
enum Section {
    Name,
    Objsense,
    Rows,
    Columns,
    Rhs,
    Ranges,
    Bounds,
    /// Quadratic terms for `row`; an entry `q` is scaled by `diagonal` or `off_diagonal`.
    Quadratic {
        row: Row,
        diagonal: f64,
        off_diagonal: f64,
    },
    End,
}

/// What a row name refers to.
#[derive(Clone, Copy)]
enum Row {
    Objective,
    /// Further `N` rows; like other readers we drop them.
    Free,
    Constraint(usize),
}

struct Reader<'a> {
    problem: Problem<'a>,
    rows: HashMap<&'a str, Row>,
    has_objective: bool,
    ranges: Vec<Option<f64>>, // per constraint
    bounded: HashSet<usize>,  // variables with a BOUNDS entry
    integer_block: bool,      // between the 'INTORG' and 'INTEND' markers
    /// The lines of one column follow each other, so this saves most name lookups.
    last_column: Option<(&'a str, usize)>,
}

impl<'a> Reader<'a> {
    fn header(&mut self, line: &'a str) -> Result<Section, String> {
        let mut words = line.split_whitespace();
        let keyword = words.next().unwrap_or_default().to_ascii_uppercase();
        let arg = words.next();
        let row_arg = || arg.ok_or_else(|| format!("{keyword} needs a row name"));

        let section = match keyword.as_str() {
            "NAME" => {
                // Everything after the keyword, so fixed-format names may contain spaces.
                let name = line.trim_start()[4..].trim();
                self.problem.name = Some(name).filter(|name| !name.is_empty());
                Section::Name
            }
            "OBJSENSE" => {
                if let Some(word) = arg {
                    self.problem.sense = sense(word)?;
                }
                Section::Objsense
            }
            "ROWS" => Section::Rows,
            "COLUMNS" => Section::Columns,
            "RHS" => Section::Rhs,
            "RANGES" => Section::Ranges,
            "BOUNDS" | "BOUND" => Section::Bounds,
            // Objective 1/2 x'Qx with one triangle of Q (Gurobi and others).
            "QUADOBJ" => quadratic(Row::Objective, 0.5, 1.0),
            // Objective 1/2 x'Qx with all of Q (CPLEX).
            "QMATRIX" => quadratic(Row::Objective, 0.5, 0.5),
            // 1/2 x'Qx for the objective or a row, lower triangle of Q (MOSEK).
            "QSECTION" => quadratic(self.row(row_arg()?)?, 0.5, 1.0),
            // x'Qx for a row with all of Q (CPLEX, Gurobi).
            "QCMATRIX" => quadratic(self.row(row_arg()?)?, 1.0, 1.0),
            "ENDATA" => Section::End,
            _ => return Err(format!("Unsupported section '{}'", line.trim())),
        };
        Ok(section)
    }

    fn data(&mut self, section: Section, fields: &[&'a str]) -> Result<(), String> {
        match section {
            Section::Objsense => match fields {
                [word] => self.problem.sense = sense(word)?,
                _ => return Err(unexpected(fields)),
            },
            Section::Rows => self.row_line(fields)?,
            Section::Columns => self.column_line(fields)?,
            Section::Rhs => self.rhs_line(fields, false)?,
            Section::Ranges => self.rhs_line(fields, true)?,
            Section::Bounds => self.bound_line(fields)?,
            Section::Quadratic {
                row,
                diagonal,
                off_diagonal,
            } => self.quadratic_line(fields, row, diagonal, off_diagonal)?,
            Section::Name | Section::End => return Err(unexpected(fields)),
        }
        Ok(())
    }

    /// `type name`, e.g. `L c1`.
    fn row_line(&mut self, fields: &[&'a str]) -> Result<(), String> {
        let [kind, name] = fields else {
            return Err(unexpected(fields));
        };
        if self.rows.contains_key(name) {
            return Err(format!("duplicate row '{name}'"));
        }
        let comparator = match kind.as_bytes() {
            [b'N' | b'n'] => {
                let row = if self.has_objective {
                    Row::Free
                } else {
                    Row::Objective
                };
                self.has_objective = true;
                self.rows.insert(name, row);
                return Ok(());
            }
            [b'L' | b'l'] => Comparator::Le,
            [b'G' | b'g'] => Comparator::Ge,
            [b'E' | b'e'] => Comparator::Eq,
            _ => return Err(format!("unknown row type '{kind}'")),
        };
        self.rows
            .insert(name, Row::Constraint(self.problem.constraints.len()));
        self.problem.constraints.push(Constraint {
            name: Some(Cow::Borrowed(name)),
            lhs: Expr::default(),
            comparator,
            rhs: 0.0,
        });
        self.ranges.push(None);
        Ok(())
    }

    /// `column row value [row value]`, or a marker `name 'MARKER' 'INTORG'|'INTEND'`.
    fn column_line(&mut self, fields: &[&'a str]) -> Result<(), String> {
        if let [_, "'MARKER'", marker] = fields {
            self.integer_block = match *marker {
                "'INTORG'" => true,
                "'INTEND'" => false,
                _ => return Err(format!("unknown marker {marker}")),
            };
            return Ok(());
        }
        let [column, pairs @ ..] = fields else {
            return Err(unexpected(fields));
        };
        if pairs.len() != 2 && pairs.len() != 4 {
            return Err(unexpected(fields));
        }

        let id = match self.last_column {
            Some((last, id)) if last == *column => id,
            _ => self.problem.var_id(column),
        };
        self.last_column = Some((column, id));
        if self.integer_block {
            self.problem.vtypes[id] = Vtype::Integer;
        }
        for pair in pairs.chunks(2) {
            let value = number(pair[1])?;
            let row = self.row(pair[0])?;
            if let Some(expr) = self.expr(row) {
                expr.linear.push((id, value));
            }
        }
        Ok(())
    }

    /// `[set] row value [row value]` for RHS and RANGES; the set name is optional.
    fn rhs_line(&mut self, fields: &[&'a str], is_range: bool) -> Result<(), String> {
        let pairs = match fields.len() {
            2 | 4 => fields,
            3 | 5 => &fields[1..],
            _ => return Err(unexpected(fields)),
        };
        for pair in pairs.chunks(2) {
            let value = number(pair[1])?;
            match (self.row(pair[0])?, is_range) {
                (Row::Objective, false) => self.problem.objective.offset = -value,
                (Row::Constraint(i), false) => self.problem.constraints[i].rhs = value,
                (Row::Constraint(i), true) => self.ranges[i] = Some(value),
                _ => (), // free rows, and ranges on the objective, have no effect
            }
        }
        Ok(())
    }

    /// `type [set] column [value]`; the set name is optional.
    fn bound_line(&mut self, fields: &[&'a str]) -> Result<(), String> {
        let Some((&kind_field, rest)) = fields.split_first() else {
            return Err(unexpected(fields));
        };
        let unsupported = || format!("Unsupported bound type '{kind_field}'");
        // Bound types have two letters; compare them in upper case without allocating.
        let kind = match kind_field.as_bytes() {
            [a, b] => [a.to_ascii_uppercase(), b.to_ascii_uppercase()],
            _ => return Err(unsupported()),
        };
        let has_value = matches!(&kind, b"UP" | b"LO" | b"FX" | b"LI" | b"UI");
        let (column, value) = match (has_value, rest) {
            (true, [_, column, value] | [column, value]) => (column, number(value)?),
            (false, [column] | [_, column] | [_, column, _]) => (column, 0.0),
            _ => return Err(unexpected(fields)),
        };

        let id = self.problem.var_id(column);
        self.bounded.insert(id);
        let (lower, upper) = &mut self.problem.bounds[id];
        let vtype = &mut self.problem.vtypes[id];
        match &kind {
            b"UP" => *upper = Some(value),
            b"LO" => *lower = Some(value),
            b"FX" => (*lower, *upper) = (Some(value), Some(value)),
            b"FR" => (*lower, *upper) = (Some(f64::NEG_INFINITY), Some(f64::INFINITY)),
            b"MI" => *lower = Some(f64::NEG_INFINITY),
            b"PL" => *upper = Some(f64::INFINITY),
            b"BV" => *vtype = Vtype::Binary,
            b"LI" => (*lower, *vtype) = (Some(value), Vtype::Integer),
            b"UI" => (*upper, *vtype) = (Some(value), Vtype::Integer),
            _ => return Err(unsupported()),
        }
        Ok(())
    }

    /// `column column value`.
    fn quadratic_line(
        &mut self,
        fields: &[&'a str],
        row: Row,
        diagonal: f64,
        off_diagonal: f64,
    ) -> Result<(), String> {
        let [u, v, value] = fields else {
            return Err(unexpected(fields));
        };
        let value = number(value)?;
        let (u, v) = (self.problem.var_id(u), self.problem.var_id(v));
        let factor = if u == v { diagonal } else { off_diagonal };
        if let Some(expr) = self.expr(row) {
            expr.quadratic.push((u, v, factor * value));
        }
        Ok(())
    }

    /// Integer defaults and ranges can only be applied once the whole file is read.
    fn finish(mut self) -> Problem<'a> {
        // Integer columns without any bound entry default to 0 <= x <= 1, i.e. binary.
        for (id, vtype) in self.problem.vtypes.iter_mut().enumerate() {
            if *vtype == Vtype::Integer && !self.bounded.contains(&id) {
                *vtype = Vtype::Binary;
            }
        }

        let constraints = std::mem::take(&mut self.problem.constraints);
        for (constraint, range) in constraints.into_iter().zip(self.ranges) {
            match range {
                None => self.problem.constraints.push(constraint),
                Some(range) => self
                    .problem
                    .constraints
                    .extend(split_range(constraint, range)),
            }
        }
        self.problem
    }

    fn expr(&mut self, row: Row) -> Option<&mut Expr> {
        match row {
            Row::Objective => Some(&mut self.problem.objective),
            Row::Free => None,
            Row::Constraint(i) => Some(&mut self.problem.constraints[i].lhs),
        }
    }

    fn row(&self, name: &str) -> Result<Row, String> {
        let row = self.rows.get(name).copied();
        row.ok_or_else(|| format!("unknown row '{name}'"))
    }
}

fn quadratic(row: Row, diagonal: f64, off_diagonal: f64) -> Section {
    Section::Quadratic {
        row,
        diagonal,
        off_diagonal,
    }
}

fn sense(word: &str) -> Result<Sense, String> {
    let is = |s| word.eq_ignore_ascii_case(s);
    if is("MIN") || is("MINIMIZE") {
        Ok(Sense::Min)
    } else if is("MAX") || is("MAXIMIZE") {
        Ok(Sense::Max)
    } else {
        Err(format!("unknown objective sense '{word}'"))
    }
}

fn number(text: &str) -> Result<f64, String> {
    let value = text.parse::<f64>().ok().filter(|v| !v.is_nan());
    value.ok_or_else(|| format!("invalid number '{text}'"))
}

fn unexpected(fields: &[&str]) -> String {
    format!("unexpected line {fields:?}")
}

/// Splits a ranged row into `name_lb` (>=) and `name_ub` (<=), with the bounds SCIP and HiGHS use.
fn split_range(c: Constraint<'_>, range: f64) -> [Constraint<'_>; 2] {
    let (lower, upper) = match c.comparator {
        Comparator::Le => (c.rhs - range.abs(), c.rhs),
        Comparator::Ge => (c.rhs, c.rhs + range.abs()),
        Comparator::Eq if range >= 0.0 => (c.rhs, c.rhs + range),
        Comparator::Eq => (c.rhs + range, c.rhs),
    };
    let name = c.name.unwrap_or_default();
    let lb = Constraint {
        name: Some(format!("{name}_lb").into()),
        lhs: c.lhs.clone(),
        comparator: Comparator::Ge,
        rhs: lower,
    };
    let ub = Constraint {
        name: Some(format!("{name}_ub").into()),
        lhs: c.lhs,
        comparator: Comparator::Le,
        rhs: upper,
    };
    [lb, ub]
}
