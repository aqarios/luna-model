use lunamodel_core::{Expression, Model, ops::LmAddAssign, prelude::{Constraint, LazyBounds}};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Bound, Comparator, Sense, Vtype};
use hashbrown::HashMap;

use super::LpTranslator;

#[derive(Debug)]
struct LpProblem {
    name: Option<String>,
    sense: Sense,
    objective: LpExpression,
    constraints: Vec<LpConstraint>,
    bounds: HashMap<String, (Option<Bias>, Option<Bias>)>,
    generals: Vec<String>,
    binaries: Vec<String>,
}

#[derive(Debug, Default)]
struct LpExpression {
    linear: HashMap<String, Bias>,
    quadratic: HashMap<(String, String), Bias>,
    constant: Bias,
}

#[derive(Debug)]
struct LpConstraint {
    name: Option<String>,
    lhs: LpExpression,
    comparator: Comparator,
    rhs: Bias,
}

impl LpTranslator {
    pub fn translate(content: String) -> LunaModelResult<Model> {
        let problem = Self::parse_lp(&content)?;
        Self::build_model(problem)
    }

    fn parse_lp(content: &str) -> LunaModelResult<LpProblem> {
        let mut lines = content.lines().peekable();
        let mut problem = LpProblem {
            name: None,
            sense: Sense::Min,
            objective: LpExpression::default(),
            constraints: Vec::new(),
            bounds: HashMap::new(),
            generals: Vec::new(),
            binaries: Vec::new(),
        };

        let mut current_section = Section::None;
        let mut accumulated_line = String::new();

        while let Some(line) = lines.next() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.starts_with('\\') || line.is_empty() {
                continue;
            }

            // Remove inline comments
            let line = if let Some(pos) = line.find('\\') {
                &line[..pos]
            } else {
                line
            };
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            // Check for section headers (case-insensitive)
            let line_upper = line.to_uppercase();
            if line_upper.starts_with("MINIMIZE") || line_upper.starts_with("MINIMUM") || line_upper.starts_with("MIN") {
                current_section = Section::Objective;
                problem.sense = Sense::Min;
                continue;
            } else if line_upper.starts_with("MAXIMIZE") || line_upper.starts_with("MAXIMUM") || line_upper.starts_with("MAX") {
                current_section = Section::Objective;
                problem.sense = Sense::Max;
                continue;
            } else if line_upper.starts_with("SUBJECT TO") || line_upper.starts_with("SUCH THAT") || line_upper.starts_with("ST") || line_upper.starts_with("S.T.") {
                current_section = Section::Constraints;
                continue;
            } else if line_upper.starts_with("BOUNDS") || line_upper.starts_with("BOUND") {
                current_section = Section::Bounds;
                continue;
            } else if line_upper.starts_with("GENERAL") || line_upper.starts_with("GENERALS") || line_upper.starts_with("GEN") {
                current_section = Section::General;
                continue;
            } else if line_upper.starts_with("BINARY") || line_upper.starts_with("BINARIES") || line_upper.starts_with("BIN") {
                current_section = Section::Binary;
                continue;
            } else if line_upper.starts_with("END") {
                break;
            }

            // Accumulate multi-line statements
            accumulated_line.push(' ');
            accumulated_line.push_str(line);

            // Check if line is complete (not ending with + or -)
            let trimmed = accumulated_line.trim();
            if !trimmed.is_empty() && !trimmed.ends_with('+') && !trimmed.ends_with('-') {
                match current_section {
                    Section::Objective => {
                        problem.objective = Self::parse_expression(&accumulated_line)?;
                    }
                    Section::Constraints => {
                        if let Some(constraint) = Self::parse_constraint(&accumulated_line)? {
                            problem.constraints.push(constraint);
                        }
                    }
                    Section::Bounds => {
                        Self::parse_bound(&accumulated_line, &mut problem.bounds)?;
                    }
                    Section::General => {
                        problem.generals.extend(Self::parse_var_list(&accumulated_line));
                    }
                    Section::Binary => {
                        problem.binaries.extend(Self::parse_var_list(&accumulated_line));
                    }
                    Section::None => {}
                }
                accumulated_line.clear();
            }
        }

        Ok(problem)
    }

    fn parse_expression(s: &str) -> LunaModelResult<LpExpression> {
        let mut expr = LpExpression::default();
        let s = s.trim();
        
        // Remove optional label (e.g., "obj:" or "c1:")
        let s = if let Some(pos) = s.find(':') {
            &s[pos + 1..]
        } else {
            s
        };

        let s = s.trim();
        let mut chars = s.chars().peekable();
        let mut current_sign = 1.0;
        let mut coefficient = String::new();
        let mut variable = String::new();
        let mut in_brackets = false;
        let mut bracket_content = String::new();

        while let Some(ch) = chars.next() {
            match ch {
                ' ' | '\t' => continue,
                '+' => {
                    Self::process_term(&mut expr, current_sign, &coefficient, &variable)?;
                    coefficient.clear();
                    variable.clear();
                    current_sign = 1.0;
                }
                '-' => {
                    Self::process_term(&mut expr, current_sign, &coefficient, &variable)?;
                    coefficient.clear();
                    variable.clear();
                    current_sign = -1.0;
                }
                '[' => {
                    in_brackets = true;
                    bracket_content.clear();
                }
                ']' => {
                    in_brackets = false;
                    // Parse quadratic term: coef var1 * var2
                    Self::parse_quadratic_term(&mut expr, current_sign, &coefficient, &bracket_content)?;
                    coefficient.clear();
                    bracket_content.clear();
                }
                '*' if !in_brackets => {
                    // Multiplication between coefficient and variable
                    if variable.is_empty() && !coefficient.is_empty() {
                        // coefficient already parsed, next is variable
                    }
                }
                '^' => {
                    // Handle x^2 for quadratic terms
                    if chars.peek() == Some(&'2') {
                        chars.next();
                        if !variable.is_empty() {
                            let var = variable.clone();
                            let coef = if coefficient.is_empty() {
                                current_sign
                            } else {
                                current_sign * coefficient.parse::<Bias>().unwrap_or(1.0)
                            };
                            *expr.quadratic.entry((var.clone(), var)).or_insert(0.0) += coef;
                            coefficient.clear();
                            variable.clear();
                            current_sign = 1.0;
                        }
                    }
                }
                _ => {
                    if in_brackets {
                        bracket_content.push(ch);
                    } else if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' {
                        if variable.is_empty() {
                            coefficient.push(ch);
                        } else {
                            variable.push(ch);
                        }
                    } else {
                        variable.push(ch);
                    }
                }
            }
        }

        // Process last term
        Self::process_term(&mut expr, current_sign, &coefficient, &variable)?;

        Ok(expr)
    }

    fn process_term(expr: &mut LpExpression, sign: Bias, coef: &str, var: &str) -> LunaModelResult<()> {
        let coef_val = if coef.is_empty() {
            sign
        } else {
            sign * coef.trim().parse::<Bias>().map_err(|_| {
                LunaModelError::Translation(format!("Invalid coefficient: {}", coef).into())
            })?
        };

        let var = var.trim();
        if var.is_empty() {
            if !coef.is_empty() {
                expr.constant += coef_val;
            }
        } else {
            *expr.linear.entry(var.to_string()).or_insert(0.0) += coef_val;
        }

        Ok(())
    }

    fn parse_quadratic_term(expr: &mut LpExpression, sign: Bias, coef: &str, content: &str) -> LunaModelResult<()> {
        let coef_val = if coef.is_empty() {
            sign
        } else {
            sign * coef.trim().parse::<Bias>().map_err(|_| {
                LunaModelError::Translation(format!("Invalid coefficient: {}", coef).into())
            })?
        };

        // Parse "var1 * var2" or "var1 ^ 2"
        let parts: Vec<&str> = content.split('*').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let var1 = parts[0].trim().to_string();
            let var2 = parts[1].trim().to_string();
            let key = if var1 <= var2 {
                (var1, var2)
            } else {
                (var2, var1)
            };
            *expr.quadratic.entry(key).or_insert(0.0) += coef_val;
        } else if content.contains('^') {
            let var = content.split('^').next().unwrap().trim().to_string();
            *expr.quadratic.entry((var.clone(), var)).or_insert(0.0) += coef_val;
        }

        Ok(())
    }

    fn parse_constraint(s: &str) -> LunaModelResult<Option<LpConstraint>> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(None);
        }

        // Extract constraint name if present
        let (name, rest) = if let Some(pos) = s.find(':') {
            (Some(s[..pos].trim().to_string()), &s[pos + 1..])
        } else {
            (None, s)
        };

        // Find comparison operator
        let (lhs_str, comparator, rhs_str) = if let Some(pos) = rest.find("<=") {
            (&rest[..pos], Comparator::Le, &rest[pos + 2..])
        } else if let Some(pos) = rest.find("=<") {
            (&rest[..pos], Comparator::Le, &rest[pos + 2..])
        } else if let Some(pos) = rest.find(">=") {
            (&rest[..pos], Comparator::Ge, &rest[pos + 2..])
        } else if let Some(pos) = rest.find("=>") {
            (&rest[..pos], Comparator::Ge, &rest[pos + 2..])
        } else if let Some(pos) = rest.find('=') {
            (&rest[..pos], Comparator::Eq, &rest[pos + 1..])
        } else {
            return Err(LunaModelError::Translation(
                format!("No comparison operator found in constraint: {}", s).into()
            ));
        };

        let lhs = Self::parse_expression(lhs_str)?;
        let rhs = rhs_str.trim().parse::<Bias>().map_err(|_| {
            LunaModelError::Translation(format!("Invalid RHS value: {}", rhs_str).into())
        })?;

        Ok(Some(LpConstraint {
            name,
            lhs,
            comparator,
            rhs,
        }))
    }

    fn parse_bound(s: &str, bounds: &mut HashMap<String, (Option<Bias>, Option<Bias>)>) -> LunaModelResult<()> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(());
        }

        // Handle formats: "lb <= var <= ub", "var >= lb", "var <= ub", "var = value", "var free"
        if s.to_uppercase().contains("FREE") {
            let var = s.split_whitespace().next().unwrap().to_string();
            bounds.insert(var, (None, None));
        } else if s.contains("<=") || s.contains("=<") {
            let parts: Vec<&str> = s.split(|c| c == '<' || c == '=').filter(|p| !p.is_empty()).collect();
            if parts.len() == 3 {
                // lb <= var <= ub
                let lb = parts[0].trim().parse::<Bias>().ok();
                let var = parts[1].trim().to_string();
                let ub = parts[2].trim().parse::<Bias>().ok();
                bounds.insert(var, (lb, ub));
            } else if parts.len() == 2 {
                // var <= ub
                let var = parts[0].trim().to_string();
                let ub = parts[1].trim().parse::<Bias>().ok();
                let entry = bounds.entry(var).or_insert((None, None));
                entry.1 = ub;
            }
        } else if s.contains(">=") || s.contains("=>") {
            let parts: Vec<&str> = s.split(|c| c == '>' || c == '=').filter(|p| !p.is_empty()).collect();
            if parts.len() == 2 {
                // var >= lb
                let var = parts[0].trim().to_string();
                let lb = parts[1].trim().parse::<Bias>().ok();
                let entry = bounds.entry(var).or_insert((None, None));
                entry.0 = lb;
            }
        } else if s.contains('=') {
            let parts: Vec<&str> = s.split('=').collect();
            if parts.len() == 2 {
                let var = parts[0].trim().to_string();
                let val = parts[1].trim().parse::<Bias>().ok();
                bounds.insert(var, (val, val));
            }
        }

        Ok(())
    }

    fn parse_var_list(s: &str) -> Vec<String> {
        s.split_whitespace()
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect()
    }

    fn build_model(problem: LpProblem) -> LunaModelResult<Model> {
        let model_name = problem.name;
        let mut model = Model::new(model_name, Some(problem.sense));

        // Collect all variables
        let mut all_vars = HashMap::new();
        
        // From objective
        for var in problem.objective.linear.keys() {
            all_vars.insert(var.clone(), Vtype::Real);
        }
        for (v1, v2) in problem.objective.quadratic.keys() {
            all_vars.insert(v1.clone(), Vtype::Real);
            all_vars.insert(v2.clone(), Vtype::Real);
        }
        
        // From constraints
        for constraint in &problem.constraints {
            for var in constraint.lhs.linear.keys() {
                all_vars.insert(var.clone(), Vtype::Real);
            }
            for (v1, v2) in constraint.lhs.quadratic.keys() {
                all_vars.insert(v1.clone(), Vtype::Real);
                all_vars.insert(v2.clone(), Vtype::Real);
            }
        }

        // Set variable types
        for var in &problem.generals {
            all_vars.insert(var.clone(), Vtype::Integer);
        }
        for var in &problem.binaries {
            all_vars.insert(var.clone(), Vtype::Binary);
        }

        // Add variables to model
        for (var_name, vtype) in &all_vars {
            let bounds = problem.bounds.get(var_name).map(|(lb, ub)| {
                LazyBounds::new(
                    lb.map(|v| Bound::Bounded(v)),
                    ub.map(|v| Bound::Bounded(v))
                )
            });
            model.add_var(var_name, *vtype, bounds)?;
        }

        // Build objective
        Self::expression_to_model(&problem.objective, &mut model.objective)?;

        // Build constraints
        for constraint in problem.constraints {
            let mut lhs = Expression::empty(model.environment.clone());
            Self::expression_to_model(&constraint.lhs, &mut lhs)?;
            let constr = Constraint::new(lhs, constraint.rhs, constraint.comparator, constraint.name.clone())?;
            model.constraints.add_constraint(constr, constraint.name)?;
        }

        Ok(model)
    }

    fn expression_to_model(lp_expr: &LpExpression, expr: &mut Expression) -> LunaModelResult<()> {
        // Add constant
        expr.offset = lp_expr.constant;

        // Add linear terms
        for (var_name, coef) in &lp_expr.linear {
            let var = expr.env.lookup(var_name)?;
            expr.add_assign((&var * *coef)?)?;
        }

        // Add quadratic terms
        for ((v1, v2), coef) in &lp_expr.quadratic {
            let var1 = expr.env.lookup(v1)?;
            let var2 = expr.env.lookup(v2)?;
            let product = (&var1 * &var2)?;
            expr.add_assign((product * *coef)?)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Section {
    None,
    Objective,
    Constraints,
    Bounds,
    General,
    Binary,
}

#[cfg(test)]
mod tests {
    use super::*;
    use lunamodel_types::Sense;

    #[test]
    fn test_parse_simple_linear() {
        let lp_content = r#"
Minimize
  x + 2 y
Subject To
  c1: x + y <= 10
Bounds
  x >= 0
  y >= 0
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Min);
        assert_eq!(model.environment.vars().len(), 2);
        assert_eq!(model.constraints.len(), 1);
        assert_eq!(model.objective.linear.len(), 2);
    }

    #[test]
    fn test_parse_quadratic() {
        let lp_content = r#"
Maximize
  x + [ x * y ]
Subject To
  c1: x + y <= 5
Bounds
  0 <= x <= 5
  0 <= y <= 5
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Max);
        assert_eq!(model.environment.vars().len(), 2);
        assert!(model.objective.quadratic.is_some());
    }

    #[test]
    fn test_parse_variable_types() {
        let lp_content = r#"
Minimize
  x + y + z
Subject To
  c1: x + y + z <= 10
Bounds
  x >= 0
  z >= 0
General
  x
Binary
  y
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 3);
        
        let x = model.environment.lookup("x").unwrap();
        let y = model.environment.lookup("y").unwrap();
        let z = model.environment.lookup("z").unwrap();
        
        assert_eq!(x.vtype().unwrap(), Vtype::Integer);
        assert_eq!(y.vtype().unwrap(), Vtype::Binary);
        assert_eq!(z.vtype().unwrap(), Vtype::Real);
    }

    #[test]
    fn test_parse_multi_line() {
        let lp_content = r#"
Minimize
  x + 
  2 y +
  3 z
Subject To
  c1: x + y <= 10
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.objective.linear.len(), 3);
    }

    #[test]
    fn test_parse_comments() {
        let lp_content = r#"
\ This is a comment
Minimize
  x + y \ inline comment
Subject To
  c1: x <= 5 \ another comment
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 2);
    }

    #[test]
    fn test_parse_comparators() {
        let lp_content = r#"
Minimize
  x
Subject To
  c1: x <= 5
  c2: y >= 2
  c3: z = 3
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.constraints.len(), 3);
    }

    #[test]
    fn test_parse_squared_terms() {
        let lp_content = r#"
Minimize
  x^2 + y
Subject To
  c1: x + y <= 10
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert!(model.objective.quadratic.is_some());
    }

    #[test]
    fn test_gurobi_format() {
        // Gurobi-style LP format
        let lp_content = r#"
Minimize
 obj: 2 x1 + 3 x2
Subject To
 c1: x1 + x2 <= 4
 c2: x1 - x2 >= 1
Bounds
 x1 free
 0 <= x2 <= 10
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Min);
        assert_eq!(model.environment.vars().len(), 2);
        assert_eq!(model.constraints.len(), 2);
    }

    #[test]
    fn test_cplex_format() {
        // CPLEX-style LP format
        let lp_content = r#"
Maximize
 2 x + 3 y
st
 x + y =< 5
 x - y => 1
Bounds
 x >= 0
 y >= 0
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Max);
        assert_eq!(model.constraints.len(), 2);
    }
}
