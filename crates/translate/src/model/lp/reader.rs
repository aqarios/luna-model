use hashbrown::{HashMap, HashSet};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Comparator, Sense};

use super::tokenizer::{Token, tokenize};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Section {
    None,
    Objective(Sense),
    Constraints,
    Bounds,
    General,
    Binary,
    End,
    Unsupported,
}

impl Section {
    fn try_header(line: &str) -> Option<Section> {
        let lu = line.trim().to_uppercase();
        if lu == "MINIMIZE" || lu == "MINIMUM" || lu == "MIN" {
            Some(Section::Objective(Sense::Min))
        } else if lu == "MAXIMIZE" || lu == "MAXIMUM" || lu == "MAX" {
            Some(Section::Objective(Sense::Max))
        } else if lu == "SUBJECT TO"
            || lu == "SUCH THAT"
            || lu == "ST"
            || lu == "S.T."
        {
            Some(Section::Constraints)
        } else if lu == "BOUNDS" || lu == "BOUND" {
            Some(Section::Bounds)
        } else if lu == "GENERAL" || lu == "GENERALS" || lu == "GEN" || lu == "INTEGERS" {
            Some(Section::General)
        } else if lu == "BINARY" || lu == "BINARIES" || lu == "BIN" {
            Some(Section::Binary)
        } else if lu == "END" {
            Some(Section::End)
        } else if lu == "MULTI-OBJECTIVES"
            || lu == "LAZY CONSTRAINTS"
            || lu == "USER CUTS"
            || lu == "SOS"
            || lu == "GENERAL CONSTRAINTS"
            || lu == "GENERAL CONSTRAINT"
            || lu == "G.C."
            || lu == "SCENARIO"
        {
            Some(Section::Unsupported)
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct LpExpression(pub Vec<Token>);

#[derive(Debug)]
pub struct LpConstraint {
    pub(super) name: Option<String>,
    pub(super) lhs: LpExpression,
    pub(super) comparator: Comparator,
    pub(super) rhs: Bias,
}

#[derive(Debug, Default)]
pub struct LpProblem {
    pub(super) name: Option<String>,
    pub(super) sense: Sense,
    pub(super) objective: LpExpression,
    pub(super) constraints: Vec<LpConstraint>,
    pub(super) bounds: HashMap<String, (Option<Bias>, Option<Bias>)>,
    // integers
    pub(super) generals: HashSet<String>,
    pub(super) binaries: HashSet<String>,
    // all vars
    pub(super) vars: HashSet<String>,
}

pub fn read_lp(content: &str) -> LunaModelResult<LpProblem> {
    let mut lines = content.lines().peekable();
    let mut prob = LpProblem::default();

    let mut section = Section::None;
    let mut accumulated_line = String::new();
    let mut line_cache = String::new();
    let mut obj_found = false;
    let mut obj_section_found = false;

    while let Some(line) = lines.next() {
        let line = line.trim();

        if is_comment(&line) {
            // check if it contains the problem name.
            if line.to_uppercase().contains("PROBLEM NAME") {
                let name = line.split(':').collect::<Vec<_>>()[1].trim().to_string();
                prob.name = Some(name);
            }
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let line = remove_inline_comments(&line);
        if line.is_empty() {
            continue;
        }

        // If we have a new section header we process the accumulated line(s).
        if let Some(next_section) = Section::try_header(&line) {
            accumulated_line.push(' ');
            accumulated_line.push_str(&line_cache);
            line_cache = String::new();
            if !accumulated_line.trim().is_empty() {
                match section {
                Section::Unsupported => return Err(LunaModelError::Translation(format!("Unsupported section '{line}'").into())),
                Section::None | Section::End => (),
                Section::Objective(sense) => match obj_section_found {
                    true => return Err(LunaModelError::Translation("Multiple objectives found in LP file. Only a single objective is supported.".into())),
                    false => {
                        obj_section_found = true;
                        prob.sense = sense;
                        parse_expr(&mut prob.objective, &mut prob.vars, &accumulated_line)?;
                    }
                },
                Section::Constraints => parse_constr(&mut prob.constraints, &mut prob.vars, &accumulated_line)?,
                Section::Bounds => parse_bounds(&mut prob.bounds, &mut prob.vars, &accumulated_line)?,
                Section::General => parse_var_list(&mut prob.vars, &mut prob.generals, &accumulated_line),
                Section::Binary => parse_var_list(&mut prob.vars, &mut prob.binaries, &accumulated_line),
            }
                accumulated_line.clear();
            }
            section = next_section;

            if next_section == Section::End {
                // Reached the End.
                break;
            }
            continue;
        }

        // Depending on the current section we might need to filter some stuff from the beginning.
        match section {
            Section::None => (),
            Section::Objective(_) => {
                let line = if let Some(colpos) = line.find(':') {
                    match obj_found {
                        true => return Err(LunaModelError::Translation("Multiple objectives found in LP file. Only a single objective is supported.".into())),
                        false => obj_found = true,
                    }
                    &line[(colpos + 1)..]
                } else {
                    line.trim()
                };
                accumulated_line.push(' ');
                accumulated_line.push_str(line);
            }
            Section::Constraints => {
                let mut nxt = line_cache.clone();
                nxt.push(' ');
                nxt.push_str(line);

                // is this constraint complete?
                if is_complete_constraint(&nxt) {
                    accumulated_line.push('\n');
                    accumulated_line.push_str(&nxt);
                    line_cache = String::new();
                } else {
                    line_cache = nxt;
                }
            }
            Section::Bounds => {
                accumulated_line.push('\n');
                accumulated_line.push_str(&line);
            }
            Section::General | Section::Binary => {
                accumulated_line.push(' ');
                accumulated_line.push_str(line);
            }
            Section::End => {
                accumulated_line.push(' ');
                accumulated_line.push_str(line);
            }
            Section::Unsupported => {
                return Err(LunaModelError::Translation(
                    format!("Unsupported section '{line}'").into(),
                ));
            }
        }
    }

    if !accumulated_line.trim().is_empty() {
        return Err(LunaModelError::Translation(
            format!("unexpected rest in LP file: {accumulated_line}").into(),
        ));
    }

    Ok(prob)
}

fn is_complete_constraint(line: &str) -> bool {
    if line.contains("==") {
        line.split("==").collect::<Vec<_>>().len() == 2
    } else if line.contains("<=") {
        line.split("<=").collect::<Vec<_>>().len() == 2
    } else if line.contains(">=") {
        line.split(">=").collect::<Vec<_>>().len() == 2
    } else if line.contains("=") {
        line.split("=").collect::<Vec<_>>().len() == 2
    } else if line.contains("<") {
        line.split("<").collect::<Vec<_>>().len() == 2
    } else if line.contains(">") {
        line.split(">").collect::<Vec<_>>().len() == 2
    } else {
        false
    }
}

fn is_comment(line: &str) -> bool {
    line.starts_with('\\')
}

fn remove_inline_comments(line: &str) -> &str {
    let line = if let Some(pos) = line.find('\\') {
        &line[..pos]
    } else {
        line
    };
    line.trim()
}

fn parse_expr(
    expr: &mut LpExpression,
    vars: &mut HashSet<String>,
    line: &str,
) -> LunaModelResult<()> {
    let tokens = tokenize(line)?;
    tokens.iter().for_each(|t| {
        if let Token::Variable(v) = t {
            vars.insert(v.clone());
        }
    });
    expr.0 = tokens;
    Ok(())
}

fn parse_constr(
    constraints: &mut Vec<LpConstraint>,
    vars: &mut HashSet<String>,
    line: &str,
) -> LunaModelResult<()> {
    let lines = line.split('\n');
    for constr in lines {
        let constr = constr.trim();
        if constr.is_empty() {
            continue;
        }

        let (name, rest) = match constr.find(':') {
            Some(pos) => (
                Some(constr[..pos].trim().to_string()),
                constr[pos + 1..].trim(),
            ),
            None => (None, constr.trim()),
        };

        let (lhs, comp, rhs) = if let Some(pos) = rest.find("==") {
            (&rest[..pos], Comparator::Eq, &rest[pos + 2..])
        } else if let Some(pos) = rest.find("<=") {
            (&rest[..pos], Comparator::Le, &rest[pos + 2..])
        } else if let Some(pos) = rest.find(">=") {
            (&rest[..pos], Comparator::Ge, &rest[pos + 2..])
        } else if let Some(pos) = rest.find("=") {
            (&rest[..pos], Comparator::Eq, &rest[pos + 1..])
        } else if let Some(pos) = rest.find("<") {
            (&rest[..pos], Comparator::Le, &rest[pos + 1..])
        } else if let Some(pos) = rest.find(">") {
            (&rest[..pos], Comparator::Ge, &rest[pos + 1..])
        } else {
            return Err(LunaModelError::Translation(
                format!("No comparison operator found in constraint: {}", constr).into(),
            ));
        };

        let lhs_tokens = tokenize(lhs)?;
        lhs_tokens.iter().for_each(|t| {
            if let Token::Variable(v) = t {
                vars.insert(v.clone());
            }
        });

        let lhs = LpExpression(lhs_tokens);
        let rhs = rhs.trim().parse::<Bias>().map_err(|_| {
            LunaModelError::Translation(
                format!("Invalid RHS value '{}' for constraint '{constr}'", rhs).into(),
            )
        })?;
        constraints.push(LpConstraint {
            name,
            lhs,
            rhs,
            comparator: comp,
        });
    }

    Ok(())
}

fn parse_bounds(
    bounds: &mut HashMap<String, (Option<Bias>, Option<Bias>)>,
    vars: &mut HashSet<String>,
    line: &str,
) -> LunaModelResult<()> {
    let lines = line.split('\n');
    for bound in lines {
        let bound = bound.trim();
        if bound.is_empty() {
            continue;
        }

        if bound.to_uppercase().contains("FREE") {
            let var = bound.split_whitespace().next().unwrap().to_string();
            vars.insert(var.clone());
            bounds.insert(var, (None, None));
        } else if bound.contains("<=") {
            let parts = bound
                .split(|c| c == '<' || c == '=')
                .filter(|p| !p.is_empty())
                .collect::<Vec<_>>();
            match parts.len() {
                2 => {
                    // var <= ub
                    let var = parts[0].trim().to_string();
                    vars.insert(var.clone());
                    let ub = parts[1].trim().parse::<Bias>().ok();
                    let entry = bounds.entry(var).or_insert((None, None));
                    entry.1 = ub;
                }
                3 => {
                    // lb <= var <= ub
                    let lb = parts[0].trim().parse::<Bias>().ok();
                    let var = parts[1].trim().to_string();
                    vars.insert(var.clone());
                    let ub = parts[2].trim().parse::<Bias>().ok();
                    bounds.insert(var, (lb, ub));
                }
                n => {
                    return Err(LunaModelError::Translation(
                        format!("invalid bounds with '{n}' parts, expected '2' or '3': {bound}")
                            .into(),
                    ));
                }
            }
        } else if bound.contains(">=") {
            let parts = bound
                .split(|c| c == '>' || c == '=')
                .filter(|p| !p.is_empty())
                .collect::<Vec<_>>();
            match parts.len() {
                2 => {
                    // var >= lb
                    let var = parts[0].trim().to_string();
                    vars.insert(var.clone());
                    let lb = parts[1].trim().parse::<Bias>().ok();
                    let entry = bounds.entry(var).or_insert((None, None));
                    entry.0 = lb;
                }
                n => {
                    return Err(LunaModelError::Translation(
                        format!("invalid bounds with '{n}' parts, expected '2': {bound}").into(),
                    ));
                }
            }
        } else if bound.contains("=") {
            let parts = bound
                .split('=')
                .filter(|p| !p.is_empty())
                .collect::<Vec<_>>();
            match parts.len() {
                2 => {
                    // var = val
                    let var = parts[0].trim().to_string();
                    vars.insert(var.clone());
                    let val = parts[1].trim().parse::<Bias>().ok();
                    bounds.insert(var, (val, val));
                }
                n => {
                    return Err(LunaModelError::Translation(
                        format!("invalid bounds with '{n}' parts, expected '2': {bound}").into(),
                    ));
                }
            }
        } else {
            return Err(LunaModelError::Translation(
                format!("unexpected bounds entry: {bound}").into(),
            ));
        }
    }

    Ok(())
}

fn parse_var_list(
    allvars: &mut HashSet<String>, 
    vars: &mut HashSet<String>,
    line: &str
    ) {
    line.split_whitespace()
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .for_each(|v| {
            allvars.insert(v.clone());
            vars.insert(v.clone());
        });
}
