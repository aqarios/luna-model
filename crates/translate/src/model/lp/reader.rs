//! Parses LP text into a [`Problem`].

use std::borrow::Cow;

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Comparator, Sense, Vtype};

use super::tokenizer::{Spanned, Token, tokenize};
use crate::model::problem::{Constraint, Expr, Problem, error};

const MULTIPLE_OBJECTIVES: &str =
    "Multiple objectives found in LP file. Only a single objective is supported.";

pub fn read_lp(src: &str) -> LunaModelResult<Problem<'_>> {
    let mut parser = Parser {
        lines: src.lines().collect(),
        tokens: tokenize(src)?,
        pos: 0,
        problem: Problem::default(),
    };
    parser.problem.name = problem_name(src);
    parser.sections()?;
    Ok(parser.problem)
}

/// The name from a `\ Problem name: foo` comment, as LP writers emit it.
fn problem_name(src: &str) -> Option<&str> {
    src.lines()
        .filter_map(|line| line.trim_start().strip_prefix('\\'))
        .find_map(|comment| comment.trim().strip_prefix("Problem name:"))
        .map(str::trim)
}

#[derive(Clone, Copy)]
enum Section {
    Objective(Sense),
    Constraints,
    Bounds,
    Generals,
    Binaries,
    End,
    Unsupported,
}

/// Recognises a section header. A header is always alone on its line.
fn section(line: &str) -> Option<Section> {
    let line = line.split('\\').next().unwrap_or_default().trim();
    let is = |words: &[&str]| words.iter().any(|w| w.eq_ignore_ascii_case(line));

    let section = if is(&["minimize", "minimum", "min"]) {
        Section::Objective(Sense::Min)
    } else if is(&["maximize", "maximum", "max"]) {
        Section::Objective(Sense::Max)
    } else if is(&["subject to", "such that", "st", "s.t."]) {
        Section::Constraints
    } else if is(&["bounds", "bound"]) {
        Section::Bounds
    } else if is(&["generals", "general", "gen", "integers"]) {
        Section::Generals
    } else if is(&["binaries", "binary", "bin"]) {
        Section::Binaries
    } else if is(&["end"]) {
        Section::End
    } else if is(&[
        "semi-continuous",
        "semis",
        "semi",
        "sos",
        "lazy constraints",
        "user cuts",
        "general constraints",
        "g.c.",
        "pwlobj",
        "multi-objectives",
        "scenario",
    ]) {
        Section::Unsupported
    } else {
        return None;
    };
    Some(section)
}

fn is_infinity(name: &str) -> bool {
    name.eq_ignore_ascii_case("inf") || name.eq_ignore_ascii_case("infinity")
}

/// Whether LP readers could take `name` for a keyword instead of a name.
pub fn is_keyword(name: &str) -> bool {
    section(name).is_some() || name.eq_ignore_ascii_case("free") || is_infinity(name)
}

/// `value cmp var` means `var flip(cmp) value`.
fn flip(cmp: Comparator) -> Comparator {
    match cmp {
        Comparator::Le => Comparator::Ge,
        Comparator::Ge => Comparator::Le,
        Comparator::Eq => Comparator::Eq,
    }
}

struct Parser<'a> {
    lines: Vec<&'a str>,
    tokens: Vec<Spanned<'a>>,
    pos: usize,
    problem: Problem<'a>,
}

impl<'a> Parser<'a> {
    fn sections(&mut self) -> LunaModelResult<()> {
        let mut has_objective = false;
        while self.pos < self.tokens.len() {
            let Some(section) = self.header() else {
                return Err(self.expected("a section header"));
            };
            if let Section::Unsupported = section {
                let line = self.lines[self.tokens[self.pos].line - 1].trim();
                return Err(error(self.line(), format!("Unsupported section '{line}'")));
            }
            self.skip_line();

            match section {
                Section::Objective(sense) => {
                    if has_objective {
                        return Err(error(self.line(), MULTIPLE_OBJECTIVES.into()));
                    }
                    has_objective = true;
                    self.problem.sense = sense;
                    self.label();
                    self.problem.objective = self.expr()?;
                    if let Some(name) = self.label() {
                        let msg = format!("{MULTIPLE_OBJECTIVES} Found a second one: '{name}'");
                        return Err(error(self.line(), msg));
                    }
                }
                Section::Constraints => {
                    while !self.at_section_end() {
                        let constraint = self.constraint()?;
                        self.problem.constraints.push(constraint);
                    }
                }
                Section::Bounds => {
                    while !self.at_section_end() {
                        self.bound()?;
                    }
                }
                Section::Generals => self.var_list(Vtype::Integer)?,
                Section::Binaries => self.var_list(Vtype::Binary)?,
                Section::End => break,
                Section::Unsupported => unreachable!("rejected above"),
            }
        }
        Ok(())
    }

    /// Parses `[name:] expr cmp number`.
    fn constraint(&mut self) -> LunaModelResult<Constraint<'a>> {
        let name = self.label().map(Cow::Borrowed);
        let lhs = self.expr()?;
        let comparator = self.comparator()?;
        let rhs = self.number()?;
        Ok(Constraint {
            name,
            lhs,
            comparator,
            rhs,
        })
    }

    /// Parses `name free`, `name cmp number`, or `number cmp name [cmp number]`.
    fn bound(&mut self) -> LunaModelResult<()> {
        match self.peek() {
            Some(Token::Name(name)) if !is_infinity(name) => {
                let id = self.var()?;
                if matches!(self.peek(), Some(Token::Name(w)) if w.eq_ignore_ascii_case("free")) {
                    self.pos += 1;
                    self.problem.bounds[id] = (Some(f64::NEG_INFINITY), Some(f64::INFINITY));
                } else {
                    let cmp = self.comparator()?;
                    let value = self.number()?;
                    self.set_bound(id, cmp, value);
                }
            }
            _ => {
                let value = self.number()?;
                let cmp = self.comparator()?;
                let id = self.var()?;
                self.set_bound(id, flip(cmp), value);
                if let Some(Token::Cmp(_)) = self.peek() {
                    let cmp = self.comparator()?;
                    let value = self.number()?;
                    self.set_bound(id, cmp, value);
                }
            }
        }
        Ok(())
    }

    /// Applies `var cmp value` to the variable's bounds.
    fn set_bound(&mut self, id: usize, cmp: Comparator, value: f64) {
        let (lower, upper) = &mut self.problem.bounds[id];
        match cmp {
            Comparator::Le => *upper = Some(value),
            Comparator::Ge => *lower = Some(value),
            Comparator::Eq => (*lower, *upper) = (Some(value), Some(value)),
        }
    }

    /// Gives every name until the next section the type `vtype`.
    fn var_list(&mut self, vtype: Vtype) -> LunaModelResult<()> {
        while !self.at_section_end() {
            let id = self.var()?;
            self.problem.vtypes[id] = vtype;
        }
        Ok(())
    }

    /// Parses `[sign] term (sign term)*`. An expression may be empty.
    fn expr(&mut self) -> LunaModelResult<Expr> {
        let mut expr = Expr::default();
        let mut sign = match self.sign() {
            Some(sign) => sign,
            None if self.at_term() => 1.0,
            None => return Ok(expr),
        };
        loop {
            self.term(sign, &mut expr)?;
            match self.sign() {
                Some(next) => sign = next,
                None => return Ok(expr),
            }
        }
    }

    /// Parses `[number] name [^2 | * name]`, a constant, or `[ expr ] [/ number]`.
    fn term(&mut self, sign: f64, expr: &mut Expr) -> LunaModelResult<()> {
        match self.peek() {
            Some(Token::LBracket) => {
                self.pos += 1;
                let inner = self.expr()?;
                if !self.eat(Token::RBracket) {
                    return Err(self.expected("']'"));
                }
                let divisor = if self.eat(Token::Slash) {
                    self.number()?
                } else {
                    1.0
                };
                expr.add_scaled(inner, sign / divisor);
            }
            Some(Token::Num(coef)) => {
                self.pos += 1;
                match self.peek() {
                    Some(Token::Name(name)) if self.at_term() => {
                        self.pos += 1;
                        self.factor(name, sign * coef, expr)?;
                    }
                    _ => expr.offset += sign * coef,
                }
            }
            Some(Token::Name(name)) if self.at_term() => {
                self.pos += 1;
                self.factor(name, sign, expr)?;
            }
            _ => return Err(self.expected("a term")),
        }
        Ok(())
    }

    /// Parses what follows a variable: `^2`, `* name`, or nothing.
    fn factor(&mut self, name: &'a str, coef: f64, expr: &mut Expr) -> LunaModelResult<()> {
        let u = self.problem.var_id(name);
        if self.eat(Token::Caret) {
            if self.number()? != 2.0 {
                return Err(error(self.line(), "only the power 2 is supported".into()));
            }
            expr.quadratic.push((u, u, coef));
        } else if self.eat(Token::Star) {
            let v = self.var()?;
            expr.quadratic.push((u, v, coef));
        } else {
            expr.linear.push((u, coef));
        }
        Ok(())
    }

    /// Parses `[sign] (number | inf | infinity)`.
    fn number(&mut self) -> LunaModelResult<f64> {
        let sign = self.sign().unwrap_or(1.0);
        let value = match self.peek() {
            Some(Token::Num(value)) => value,
            Some(Token::Name(name)) if is_infinity(name) => f64::INFINITY,
            _ => return Err(self.expected("a number")),
        };
        self.pos += 1;
        Ok(sign * value)
    }

    fn comparator(&mut self) -> LunaModelResult<Comparator> {
        match self.peek() {
            Some(Token::Cmp(cmp)) => {
                self.pos += 1;
                Ok(cmp)
            }
            _ => Err(self.expected("a comparison operator")),
        }
    }

    fn var(&mut self) -> LunaModelResult<usize> {
        match self.peek() {
            Some(Token::Name(name)) => {
                self.pos += 1;
                Ok(self.problem.var_id(name))
            }
            _ => Err(self.expected("a variable")),
        }
    }

    /// Consumes a `+` or `-` and returns its sign.
    fn sign(&mut self) -> Option<f64> {
        if self.eat(Token::Plus) {
            Some(1.0)
        } else if self.eat(Token::Minus) {
            Some(-1.0)
        } else {
            None
        }
    }

    /// Consumes a `name:` label.
    fn label(&mut self) -> Option<&'a str> {
        let Some(Token::Name(name)) = self.peek() else {
            return None;
        };
        if !self.at_label() {
            return None;
        }
        self.pos += 2;
        Some(name)
    }

    fn at_label(&self) -> bool {
        let next = self.tokens.get(self.pos + 1).map(|t| t.token);
        matches!(self.peek(), Some(Token::Name(_))) && next == Some(Token::Colon)
    }

    /// Whether the current token starts a term, rather than a label or a section.
    fn at_term(&self) -> bool {
        match self.peek() {
            Some(Token::Num(_) | Token::LBracket) => true,
            Some(Token::Name(_)) => !self.at_label() && !self.at_section_end(),
            _ => false,
        }
    }

    fn at_section_end(&self) -> bool {
        self.pos == self.tokens.len() || self.header().is_some()
    }

    /// The section whose header line starts at the current token.
    fn header(&self) -> Option<Section> {
        let token = self.tokens.get(self.pos)?;
        let starts_line = self.pos == 0 || self.tokens[self.pos - 1].line != token.line;
        if starts_line {
            section(self.lines[token.line - 1])
        } else {
            None
        }
    }

    fn skip_line(&mut self) {
        let line = self.line();
        while self.tokens.get(self.pos).is_some_and(|t| t.line == line) {
            self.pos += 1;
        }
    }

    fn eat(&mut self, token: Token) -> bool {
        let found = self.peek() == Some(token);
        if found {
            self.pos += 1;
        }
        found
    }

    fn peek(&self) -> Option<Token<'a>> {
        self.tokens.get(self.pos).map(|t| t.token)
    }

    /// The line of the current token, or of the last one at the end of the file.
    fn line(&self) -> usize {
        let token = self.tokens.get(self.pos).or(self.tokens.last());
        token.map_or(1, |t| t.line)
    }

    fn expected(&self, what: &str) -> LunaModelError {
        let found = self
            .peek()
            .map_or("end of file".to_string(), |t| format!("{t:?}"));
        error(self.line(), format!("expected {what}, found {found}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_sections() {
        let p = read_lp(
            "\\ Problem name: demo\n\
             Maximize\n obj: 2 x + [ x * y ] / 2 + 3\n\
             Subject To\n c1:\n x - y >= -1\n x + y <= 4\n\
             Bounds\n x free\n 3 <= y <= 5\n\
             Generals\n y\n\
             End\n",
        )
        .unwrap();

        assert_eq!(p.name, Some("demo"));
        assert_eq!(p.sense, Sense::Max);
        assert_eq!(p.vars, ["x", "y"]);
        assert_eq!(p.objective.offset, 3.0);
        assert_eq!(p.objective.linear, [(0, 2.0)]);
        assert_eq!(p.objective.quadratic, [(0, 1, 0.5)]);
        assert_eq!(p.constraints.len(), 2);
        assert_eq!(p.constraints[0].name.as_deref(), Some("c1"));
        assert_eq!(p.constraints[0].lhs.linear, [(0, 1.0), (1, -1.0)]);
        assert_eq!(p.constraints[0].comparator, Comparator::Ge);
        assert_eq!(p.constraints[0].rhs, -1.0);
        assert_eq!(p.constraints[1].name.as_deref(), None);
        assert_eq!(p.bounds[0], (Some(f64::NEG_INFINITY), Some(f64::INFINITY)));
        assert_eq!(p.bounds[1], (Some(3.0), Some(5.0)));
        assert_eq!(p.vtypes, [Vtype::Real, Vtype::Integer]);
    }
}
