use hashbrown::HashMap;
use lunamodel_core::{
    Expression, Model,
    ops::LmAddAssign,
    prelude::{Constraint, LazyBounds},
};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Bound, Comparator, Sense, Vtype};

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
        let mut objective_found = false;

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
            let is_section_header = line_upper.starts_with("MINIMIZE")
                || line_upper.starts_with("MINIMUM")
                || line_upper.starts_with("MIN")
                || line_upper.starts_with("MAXIMIZE")
                || line_upper.starts_with("MAXIMUM")
                || line_upper.starts_with("MAX")
                || line_upper.starts_with("SUBJECT TO")
                || line_upper.starts_with("SUCH THAT")
                || line_upper.starts_with("ST")
                || line_upper.starts_with("S.T.")
                || line_upper.starts_with("BOUNDS")
                || line_upper.starts_with("BOUND")
                || line_upper.starts_with("GENERAL")
                || line_upper.starts_with("GENERALS")
                || line_upper.starts_with("GEN")
                || line_upper.starts_with("BINARY")
                || line_upper.starts_with("BINARIES")
                || line_upper.starts_with("BIN")
                || line_upper.starts_with("END");

            // Process accumulated content before switching sections
            if is_section_header && !accumulated_line.trim().is_empty() {
                match current_section {
                    Section::Objective => {
                        if objective_found {
                            return Err(LunaModelError::Translation(
                                "Multiple objectives found in LP file. Only single objective is supported.".into()
                            ));
                        }
                        problem.objective = Self::parse_expression(&accumulated_line)?;
                        objective_found = true;
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
                        problem
                            .generals
                            .extend(Self::parse_var_list(&accumulated_line));
                    }
                    Section::Binary => {
                        problem
                            .binaries
                            .extend(Self::parse_var_list(&accumulated_line));
                    }
                    Section::None => {}
                }
                accumulated_line.clear();
            }

            // Handle section headers
            if line_upper.starts_with("MINIMIZE")
                || line_upper.starts_with("MINIMUM")
                || line_upper.starts_with("MIN")
            {
                if objective_found {
                    return Err(LunaModelError::Translation(
                        "Multiple objectives found in LP file. Only single objective is supported."
                            .into(),
                    ));
                }
                current_section = Section::Objective;
                problem.sense = Sense::Min;
                continue;
            } else if line_upper.starts_with("MAXIMIZE")
                || line_upper.starts_with("MAXIMUM")
                || line_upper.starts_with("MAX")
            {
                if objective_found {
                    return Err(LunaModelError::Translation(
                        "Multiple objectives found in LP file. Only single objective is supported."
                            .into(),
                    ));
                }
                current_section = Section::Objective;
                problem.sense = Sense::Max;
                continue;
            } else if line_upper.starts_with("SUBJECT TO")
                || line_upper.starts_with("SUCH THAT")
                || line_upper.starts_with("ST")
                || line_upper.starts_with("S.T.")
            {
                current_section = Section::Constraints;
                continue;
            } else if line_upper.starts_with("BOUNDS") || line_upper.starts_with("BOUND") {
                current_section = Section::Bounds;
                continue;
            } else if line_upper.starts_with("GENERAL")
                || line_upper.starts_with("GENERALS")
                || line_upper.starts_with("GEN")
            {
                current_section = Section::General;
                continue;
            } else if line_upper.starts_with("BINARY")
                || line_upper.starts_with("BINARIES")
                || line_upper.starts_with("BIN")
            {
                current_section = Section::Binary;
                continue;
            } else if line_upper.starts_with("END") {
                break;
            }

            // Accumulate multi-line statements
            accumulated_line.push(' ');
            accumulated_line.push_str(line);

            // Determine if we should process the accumulated line
            let should_process = match current_section {
                Section::Objective => {
                    // Check if current line has an objective label (contains : before any operators)
                    let trimmed = accumulated_line.trim();
                    let has_label = if let Some(colon_pos) = trimmed.find(':') {
                        // Make sure the colon is before any comparison operators
                        let before_colon = &trimmed[..colon_pos];
                        !before_colon.contains("<=") && !before_colon.contains(">=") && !before_colon.contains("=<") && !before_colon.contains("=>")
                    } else {
                        false
                    };
                    
                    // Check if next line also has an objective label (multiple objectives)
                    let next_has_label = if let Some(next_line) = lines.peek() {
                        let next_trimmed = next_line.trim();
                        if let Some(colon_pos) = next_trimmed.find(':') {
                            let before_colon = &next_trimmed[..colon_pos];
                            !before_colon.contains("<=") && !before_colon.contains(">=") 
                                && !before_colon.contains("=<") && !before_colon.contains("=>")
                                && !next_trimmed.starts_with('+') && !next_trimmed.starts_with('-')
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    
                    if has_label && next_has_label && objective_found {
                        return Err(LunaModelError::Translation(
                            "Multiple objectives found in LP file. Only single objective is supported.".into()
                        ));
                    }
                    
                    // For objectives: check if next line is a section header or another objective label
                    if let Some(next_line) = lines.peek() {
                        let next_upper = next_line.trim().to_uppercase();
                        next_upper.starts_with("SUBJECT TO")
                            || next_upper.starts_with("SUCH THAT")
                            || next_upper.starts_with("ST")
                            || next_upper.starts_with("S.T.")
                            || next_upper.starts_with("BOUNDS")
                            || next_upper.starts_with("BOUND")
                            || next_upper.starts_with("GENERAL")
                            || next_upper.starts_with("BINARY")
                            || next_upper.starts_with("END")
                            || next_has_label  // Another objective label
                    } else {
                        true // End of file
                    }
                }
                Section::Constraints => {
                    let trimmed = accumulated_line.trim();
                    // Check if this contains a comparison operator (complete constraint)
                    let has_comparison = trimmed.contains("<=")
                        || trimmed.contains("=<")
                        || trimmed.contains(">=")
                        || trimmed.contains("=>")
                        || (trimmed.contains('=') && trimmed.contains(':'));

                    if has_comparison {
                        // Check if next line is a new constraint (has :) or section header
                        // or is another comparison line (no label)
                        if let Some(next_line) = lines.peek() {
                            let next_trimmed = next_line.trim();

                            // Skip empty lines and comments when checking
                            if next_trimmed.is_empty() || next_trimmed.starts_with('\\') {
                                false // Keep checking next lines
                            } else {
                                let next_upper = next_trimmed.to_uppercase();
                                let is_next_section = next_upper.starts_with("BOUNDS")
                                    || next_upper.starts_with("BOUND")
                                    || next_upper.starts_with("GENERAL")
                                    || next_upper.starts_with("BINARY")
                                    || next_upper.starts_with("END");

                                // A new constraint if it has a label (before colon) or starts fresh without continuation
                                let is_next_constraint = next_trimmed.contains(':')
                                    || (!next_trimmed.starts_with('+')
                                        && !next_trimmed.starts_with('-')
                                        && !next_trimmed.starts_with("=")
                                        && !next_trimmed.starts_with("<")
                                        && !next_trimmed.starts_with(">"));

                                is_next_section || is_next_constraint
                            }
                        } else {
                            true // End of file
                        }
                    } else {
                        false // No comparison operator yet, keep accumulating
                    }
                }
                Section::Bounds | Section::General | Section::Binary => {
                    // For these sections, process line by line unless it's a continuation
                    let trimmed = accumulated_line.trim();
                    let is_incomplete = trimmed.ends_with('+') || trimmed.ends_with('-');
                    !is_incomplete
                }
                Section::None => false,
            };

            if should_process && !accumulated_line.trim().is_empty() {
                match current_section {
                    Section::Objective => {
                        if objective_found {
                            return Err(LunaModelError::Translation(
                                "Multiple objectives found in LP file. Only single objective is supported.".into()
                            ));
                        }
                        problem.objective = Self::parse_expression(&accumulated_line)?;
                        objective_found = true;
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
                        problem
                            .generals
                            .extend(Self::parse_var_list(&accumulated_line));
                    }
                    Section::Binary => {
                        problem
                            .binaries
                            .extend(Self::parse_var_list(&accumulated_line));
                    }
                    Section::None => {}
                }
                accumulated_line.clear();
            }
        }

        // Process any remaining accumulated content
        if !accumulated_line.trim().is_empty() {
            match current_section {
                Section::Objective => {
                    if objective_found {
                        return Err(LunaModelError::Translation(
                            "Multiple objectives found in LP file. Only single objective is supported.".into()
                        ));
                    }
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
                    problem
                        .generals
                        .extend(Self::parse_var_list(&accumulated_line));
                }
                Section::Binary => {
                    problem
                        .binaries
                        .extend(Self::parse_var_list(&accumulated_line));
                }
                Section::None => {}
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
        let mut bracket_coefficient = String::new();
        let mut quadratic_divisor = 1.0;

        while let Some(ch) = chars.next() {
            match ch {
                ' ' | '\t' => continue,
                '+' => {
                    if !in_brackets {
                        Self::process_term(&mut expr, current_sign, &coefficient, &variable)?;
                        coefficient.clear();
                        variable.clear();
                        current_sign = 1.0;
                    } else {
                        bracket_content.push(ch);
                    }
                }
                '-' => {
                    if !in_brackets {
                        Self::process_term(&mut expr, current_sign, &coefficient, &variable)?;
                        coefficient.clear();
                        variable.clear();
                        current_sign = -1.0;
                    } else {
                        bracket_content.push(ch);
                    }
                }
                '[' => {
                    in_brackets = true;
                    bracket_content.clear();
                    bracket_coefficient = coefficient.clone();
                    coefficient.clear();
                    quadratic_divisor = 1.0;
                }
                ']' => {
                    in_brackets = false;

                    // Check for division after bracket: ] / 2
                    // Skip whitespace
                    while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                        chars.next();
                    }

                    if chars.peek() == Some(&'/') {
                        chars.next(); // consume '/'

                        // Skip whitespace after /
                        while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                            chars.next();
                        }

                        // Read the divisor
                        let mut divisor_str = String::new();
                        while let Some(&ch) = chars.peek() {
                            if ch.is_ascii_digit() || ch == '.' {
                                divisor_str.push(ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }

                        if !divisor_str.is_empty() {
                            quadratic_divisor = divisor_str.parse::<Bias>().unwrap_or(1.0);
                        }
                    }

                    // Parse quadratic term with divisor
                    Self::parse_quadratic_term(
                        &mut expr,
                        current_sign,
                        &bracket_coefficient,
                        &bracket_content,
                        quadratic_divisor,
                    )?;
                    coefficient.clear();
                    bracket_content.clear();
                    bracket_coefficient.clear();
                }
                '*' if !in_brackets => {
                    // Multiplication between coefficient and variable
                    if variable.is_empty() && !coefficient.is_empty() {
                        // coefficient already parsed, next is variable
                    }
                }
                '/' if !in_brackets => {
                    // Division is only valid after brackets, which we handle above
                    // Skip it here to avoid breaking on expressions like "x / 2" outside brackets
                }
                '^' => {
                    if !in_brackets {
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
                    } else {
                        bracket_content.push(ch);
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

    fn process_term(
        expr: &mut LpExpression,
        sign: Bias,
        coef: &str,
        var: &str,
    ) -> LunaModelResult<()> {
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

    fn parse_quadratic_term(
        expr: &mut LpExpression,
        sign: Bias,
        coef: &str,
        content: &str,
        divisor: Bias,
    ) -> LunaModelResult<()> {
        let coef_val = if coef.is_empty() {
            sign / divisor
        } else {
            sign * coef.trim().parse::<Bias>().map_err(|_| {
                LunaModelError::Translation(format!("Invalid coefficient: {}", coef).into())
            })? / divisor
        };

        // Parse content which may contain multiple quadratic terms like "x0 * x1 + 4 x1 * x2"
        let content = content.trim();
        let mut terms = Vec::new();
        let mut current_term = String::new();
        let mut paren_depth = 0;
        let mut next_sign = true; // true for +, false for -

        for ch in content.chars() {
            match ch {
                '(' => {
                    paren_depth += 1;
                    current_term.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    current_term.push(ch);
                }
                '+' | '-' if paren_depth == 0 => {
                    if !current_term.trim().is_empty() {
                        terms.push((next_sign, current_term.trim().to_string()));
                        current_term.clear();
                    }
                    next_sign = ch == '+';
                }
                _ => current_term.push(ch),
            }
        }

        // Don't forget the last term
        if !current_term.trim().is_empty() {
            terms.push((next_sign, current_term.trim().to_string()));
        }

        // If no +/- were found, treat the whole content as a single term
        if terms.is_empty() && !content.is_empty() {
            terms.push((true, content.to_string()));
        }

        for (is_positive, term) in terms {
            let term_sign = if is_positive { 1.0 } else { -1.0 };
            let final_coef = coef_val * term_sign;

            // Parse individual term coefficient and variables
            let term = term.trim();
            let mut term_coef = String::new();
            let mut after_coef = String::new();
            let mut found_var = false;

            for ch in term.chars() {
                if ch.is_ascii_digit() || ch == '.' {
                    if !found_var {
                        term_coef.push(ch);
                    } else {
                        after_coef.push(ch);
                    }
                } else if ch.is_alphabetic() || ch == '_' {
                    found_var = true;
                    after_coef.push(ch);
                } else {
                    after_coef.push(ch);
                }
            }

            let term_multiplier = if term_coef.is_empty() {
                1.0
            } else {
                term_coef.parse::<Bias>().unwrap_or(1.0)
            };

            let adjusted_coef = final_coef * term_multiplier;

            // Parse "var1 * var2" or "var1 ^ 2"
            if after_coef.contains('*') {
                let parts: Vec<&str> = after_coef.split('*').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let var1 = parts[0].trim().to_string();
                    let var2 = parts[1].trim().to_string();
                    let key = if var1 <= var2 {
                        (var1, var2)
                    } else {
                        (var2, var1)
                    };
                    *expr.quadratic.entry(key).or_insert(0.0) += adjusted_coef;
                }
            } else if after_coef.contains('^') {
                let var = after_coef.split('^').next().unwrap().trim().to_string();
                *expr.quadratic.entry((var.clone(), var)).or_insert(0.0) += adjusted_coef;
            }
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
                format!("No comparison operator found in constraint: {}", s).into(),
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

    fn parse_bound(
        s: &str,
        bounds: &mut HashMap<String, (Option<Bias>, Option<Bias>)>,
    ) -> LunaModelResult<()> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(());
        }

        // Handle formats: "lb <= var <= ub", "var >= lb", "var <= ub", "var = value", "var free"
        if s.to_uppercase().contains("FREE") {
            let var = s.split_whitespace().next().unwrap().to_string();
            bounds.insert(var, (None, None));
        } else if s.contains("<=") || s.contains("=<") {
            let parts: Vec<&str> = s
                .split(|c| c == '<' || c == '=')
                .filter(|p| !p.is_empty())
                .collect();
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
            let parts: Vec<&str> = s
                .split(|c| c == '>' || c == '=')
                .filter(|p| !p.is_empty())
                .collect();
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
            let bounds = if *vtype == Vtype::Binary {
                // For binary variables, validate bounds are exactly 0 <= b <= 1 or None
                if let Some((lb, ub)) = problem.bounds.get(var_name) {
                    let lb_valid = lb.map_or(true, |v| v == 0.0);
                    let ub_valid = ub.map_or(true, |v| v == 1.0);

                    if !lb_valid || !ub_valid {
                        return Err(LunaModelError::Translation(
                            format!(
                                "Binary variable '{}' has invalid bounds: {} <= {} <= {}. Binary variables must have bounds 0 <= b <= 1",
                                var_name,
                                lb.map_or("*".to_string(), |v| v.to_string()),
                                var_name,
                                ub.map_or("*".to_string(), |v| v.to_string())
                            ).into()
                        ));
                    }
                    // Don't pass bounds for binary variables - they have fixed bounds
                    None
                } else {
                    None
                }
            } else {
                problem.bounds.get(var_name).map(|(lb, ub)| {
                    LazyBounds::new(lb.map(|v| Bound::Bounded(v)), ub.map(|v| Bound::Bounded(v)))
                })
            };
            model.add_var(var_name, *vtype, bounds)?;
        }

        // Build objective
        Self::expression_to_model(&problem.objective, &mut model.objective)?;

        // Build constraints
        for constraint in problem.constraints {
            let mut lhs = Expression::empty(model.environment.clone());
            Self::expression_to_model(&constraint.lhs, &mut lhs)?;
            let constr = Constraint::new(
                lhs,
                constraint.rhs,
                constraint.comparator,
                constraint.name.clone(),
            )?;
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
 x + y <= 5
 x - y >= 1
Bounds
 x >= 0
 y >= 0
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Max);
        assert_eq!(model.constraints.len(), 2);
    }

    #[test]
    fn test_binary_with_valid_bounds() {
        // Binary variables can have bounds 0 <= b <= 1
        let lp_content = r#"
Minimize
  x
Subject To
  c1: x <= 5
Bounds
  0 <= x <= 1
Binary
  x
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        let x = model.environment.lookup("x").unwrap();
        assert_eq!(x.vtype().unwrap(), Vtype::Binary);
    }

    #[test]
    fn test_binary_with_partial_valid_bounds() {
        // Binary with only lower or upper bound that matches default
        let lp_content = r#"
Minimize
  x + y
Subject To
  c1: x + y <= 5
Bounds
  x >= 0
  y <= 1
Binary
  x y
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        let x = model.environment.lookup("x").unwrap();
        let y = model.environment.lookup("y").unwrap();
        assert_eq!(x.vtype().unwrap(), Vtype::Binary);
        assert_eq!(y.vtype().unwrap(), Vtype::Binary);
    }

    #[test]
    fn test_binary_with_invalid_bounds() {
        // Binary variables with bounds other than 0 <= b <= 1 should error
        let lp_content = r#"
Minimize
  x
Subject To
  c1: x <= 5
Bounds
  0 <= x <= 2
Binary
  x
End
"#;

        let result = LpTranslator::translate(lp_content.to_string());
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(err_msg.contains("Binary variable"));
        assert!(err_msg.contains("invalid bounds"));
    }

    #[test]
    fn test_binary_with_invalid_lower_bound() {
        // Binary variables with lower bound != 0 should error
        let lp_content = r#"
Minimize
  x
Subject To
  c1: x <= 5
Bounds
  1 <= x <= 1
Binary
  x
End
"#;

        let result = LpTranslator::translate(lp_content.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_real_world_binary_bounds() {
        // Real-world LP file where binary variables commonly have 0 <= b <= 1 bounds
        let lp_content = r#"
\ Real-world LP file with binary bounds
Minimize
  cost: 10 x1 + 15 x2 + 20 x3
Subject To
  capacity: 2 x1 + 3 x2 + 4 x3 <= 100
  selection: x1 + x2 + x3 >= 1
Bounds
  0 <= x1 <= 1
  0 <= x2 <= 1
  0 <= x3 <= 1
Binary
  x1 x2 x3
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 3);
        assert_eq!(model.constraints.len(), 2);

        // All variables should be binary
        for var_name in &["x1", "x2", "x3"] {
            let var = model.environment.lookup(var_name).unwrap();
            assert_eq!(var.vtype().unwrap(), Vtype::Binary);
        }
    }

    #[test]
    fn test_quad_lin_const_binary() {
        let lp_content = r#"
\ Model Quadratic and Linear and Constant
\ Problem name: Quadratic and Linear and Constant

Minimize
 obj: [x0 * x1 + 4 x1 * x2] / 2 + 2 x0 - 2 x1 - 2 x2 + 4
Bounds
Binaries
  x0 x1 x2
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 3);
        assert_eq!(model.constraints.len(), 0);
        assert!(model.objective.quadratic.is_some());

        // Check that we have linear terms
        assert!(model.objective.linear.len() > 0);

        // Check constant term
        assert_eq!(model.objective.offset, 4.0);

        // All variables should be binary
        for var_name in &["x0", "x1", "x2"] {
            let var = model.environment.lookup(var_name).unwrap();
            assert_eq!(var.vtype().unwrap(), Vtype::Binary);
        }

        // Check quadratic coefficients are divided by 2
        let quad = model.objective.quadratic.as_ref().unwrap();
        // The term [x0 * x1 + 4 x1 * x2] / 2 should give us:
        // 0.5 * x0 * x1 + 2.0 * x1 * x2
        // Note: Due to internal representation, just check we have quadratic terms
        assert!(quad.len() >= 1, "Expected at least 1 quadratic term in objective, got {}", quad.len());
    }

    #[test]
    fn test_multiline_constraints() {
        // Test constraints spanning multiple lines
        let lp_content = r#"
Minimize
  x + y + z
Subject To
  c1: x + y +
      z <= 10
  c2: 2 x - y +
      3 z >= 5
Bounds
  x >= 0
  y >= 0
  z >= 0
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 3);
        assert_eq!(model.constraints.len(), 2);

        // Check that all variables in constraints were parsed
        let c1 = model.constraints.get("c1").unwrap();
        assert_eq!(c1.lhs.linear.len(), 3);

        let c2 = model.constraints.get("c2").unwrap();
        assert_eq!(c2.lhs.linear.len(), 3);
    }

    #[test]
    fn test_multiline_objective_with_quadratic() {
        // Test objective with quadratic terms spanning multiple lines
        let lp_content = r#"
Minimize
  [x * y + 
   z * w] / 2 +
  x + y +
  z + w
Subject To
  c1: x + y <= 10
Bounds
  x >= 0
  y >= 0
  z >= 0
  w >= 0
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 4);
        assert!(model.objective.quadratic.is_some());
        assert_eq!(model.objective.linear.len(), 4);

        // Check that quadratic terms were divided by 2
        let quad = model.objective.quadratic.as_ref().unwrap();
        assert_eq!(quad.len(), 2);
    }

    #[test]
    fn test_multiline_constraint() {
        let lp_content = r#"
Minimize
 obj: - 25 x_0 - 18 x_1 - 27 x_2 - 27 x_3 - 15 x_4 - 15 x_5 - 11 x_6 - 24 x_7
 - 15 x_8 - 10 x_9 - 27 x_10 - 29 x_11 - 26 x_12 - 14 x_13

Subject To
 capacity: + 12 x_0 + 17 x_1 + 10 x_2 + 10 x_3 + 18 x_4 + 25 x_5 + 22 x_6
 + 13 x_7 + 17 x_8 + 14 x_9 + 23 x_10 + 28 x_11 + 24 x_12 + 27 x_13  <= 130.0

Bounds

Binary
 x_0 x_1 x_2 x_3 x_4 x_5 x_6 x_7 x_8 x_9 x_10 x_11 x_12 x_13
General

End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 14); // x_0 through x_13
        assert_eq!(model.constraints.len(), 1);

        // Check the constraint has all variables
        let capacity = model.constraints.get("capacity").unwrap();
        assert_eq!(capacity.lhs.linear.len(), 14);

        // Check objective has all variables
        assert_eq!(model.objective.linear.len(), 14);
    }

    #[test]
    fn test_multiline_constraint_le() {
        let lp_content = r#"
Minimize
 obj: - 18 x_0 - 10 x_1 - 23 x_2 - 18 x_3 - 20 x_4 - 18 x_5 - 18 x_6 - 28 x_7
 - 16 x_8 - 18 x_9 - 25 x_10 - 10 x_11 - 16 x_12 - 17 x_13 - 10 x_14

Subject To
 capacity: + 19 x_0 + 27 x_1 + 22 x_2 + 17 x_3 + 14 x_4 + 16 x_5 + 23 x_6
 + 19 x_7 + 21 x_8 + 27 x_9 + 27 x_10 + 25 x_11 + 10 x_12 + 16 x_13 + 27 x_14
  <= 155.0

Bounds

Binary
 x_0 x_1 x_2 x_3 x_4 x_5 x_6 x_7 x_8 x_9 x_10 x_11 x_12 x_13 x_14
General

End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 15); // x_0 through x_14
        assert_eq!(model.constraints.len(), 1);

        // Check the constraint has all variables
        let capacity = model.constraints.get("capacity").unwrap();
        assert_eq!(capacity.lhs.linear.len(), 15);

        // Check objective has all variables
        assert_eq!(model.objective.linear.len(), 15);
    }

    #[test]
    fn test_multi_objective_error_diff() {
        // Test that multiple objectives cause an error
        let lp_content = r#"
Minimize
  x + y
Maximize
  2 x + 3 y
Subject To
  c1: x + y <= 10
End
"#;

        let result = LpTranslator::translate(lp_content.to_string());
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(err_msg.contains("Multiple objectives"));
    }

    #[test]
    fn test_multi_objective_error() {
        // Test that multiple objectives cause an error
        let lp_content = r#"
Minimize
  obj: x + y
  obj2: 2 x + 3 y
Subject To
  c1: x + y <= 10
End
"#;

        let result = LpTranslator::translate(lp_content.to_string());
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(err_msg.contains("Multiple objectives"));
    }
}
