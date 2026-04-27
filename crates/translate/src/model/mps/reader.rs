use lunamodel_core::prelude::LazyBounds;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Bound, Comparator, Sense};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Section {
    None,
    Name,
    Objsense,
    Rows,
    Columns,
    Rhs,
    Bounds,
    Quadobj,  // Quadratic objective terms
    Qcmatrix, // Quadratic constraint terms
    End,
}

impl Section {
    fn try_parse(line: &str) -> Option<Section> {
        let upper = line.trim().to_uppercase();
        if upper.starts_with("NAME") {
            Some(Section::Name)
        } else if upper.starts_with("OBJSENSE") {
            Some(Section::Objsense)
        } else if upper == "ROWS" {
            Some(Section::Rows)
        } else if upper == "COLUMNS" {
            Some(Section::Columns)
        } else if upper == "RHS" {
            Some(Section::Rhs)
        } else if upper == "BOUNDS" || upper == "BOUND" {
            Some(Section::Bounds)
        } else if upper == "QUADOBJ" {
            Some(Section::Quadobj)
        } else if upper.starts_with("QCMATRIX") {
            Some(Section::Qcmatrix)
        } else if upper == "ENDATA" {
            Some(Section::End)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RowType {
    Objective,
    LessEqual,
    GreaterEqual,
    Equal,
}

impl RowType {
    fn from_char(c: char) -> LunaModelResult<Self> {
        match c {
            'N' | 'n' => Ok(RowType::Objective),
            'L' | 'l' => Ok(RowType::LessEqual),
            'G' | 'g' => Ok(RowType::GreaterEqual),
            'E' | 'e' => Ok(RowType::Equal),
            _ => Err(LunaModelError::Translation(
                format!("Invalid row type '{}'", c).into(),
            )),
        }
    }

    pub fn to_comparator(self) -> LunaModelResult<Comparator> {
        match self {
            RowType::LessEqual => Ok(Comparator::Le),
            RowType::GreaterEqual => Ok(Comparator::Ge),
            RowType::Equal => Ok(Comparator::Eq),
            RowType::Objective => Err(LunaModelError::Translation(
                "Objective row cannot be converted to comparator".into(),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MpsRow {
    pub row_type: RowType,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct MpsConstraint {
    pub name: String,
    pub row_type: RowType,
    pub coefficients: HashMap<String, Bias>,
    pub rhs: Bias,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundType {
    LowerBound,
    UpperBound,
    Fixed,
    Free,
    Binary,
    LowerInteger,
    UpperInteger,
    MinusInfinity,
    PlusInfinity,
}

impl BoundType {
    fn try_parse(s: &str) -> Option<Self> {
        let upper = s.to_uppercase();
        match upper.as_str() {
            "LO" => Some(BoundType::LowerBound),
            "UP" => Some(BoundType::UpperBound),
            "FX" => Some(BoundType::Fixed),
            "FR" => Some(BoundType::Free),
            "BV" => Some(BoundType::Binary),
            "LI" => Some(BoundType::LowerInteger),
            "UI" => Some(BoundType::UpperInteger),
            "MI" => Some(BoundType::MinusInfinity),
            "PL" => Some(BoundType::PlusInfinity),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct MpsProblem {
    pub name: Option<String>,
    pub sense: Sense,
    pub objective_name: Option<String>,
    pub objective_constant: Bias,
    pub objective: HashMap<String, Bias>,
    pub objective_quadratic: Vec<(String, String, Bias)>, // (var1, var2, coef) for QUADOBJ
    pub rows: HashMap<String, MpsRow>,
    pub constraints: Vec<MpsConstraint>,
    pub constraint_quadratic: HashMap<String, Vec<(String, String, Bias)>>, // constraint_name -> quadratic terms
    pub bounds: HashMap<String, Option<LazyBounds>>,
    pub integers: HashSet<String>,
    pub binaries: HashSet<String>,
    pub vars: HashSet<String>,
}

pub fn read_mps(content: &str) -> LunaModelResult<MpsProblem> {
    let mut problem = MpsProblem::default();
    let mut section = Section::None;
    let lines = content.lines();

    let mut rows_map: HashMap<String, MpsRow> = HashMap::new();
    let mut column_data: HashMap<String, HashMap<String, Bias>> = HashMap::new();
    let mut rhs_data: HashMap<String, Bias> = HashMap::new();
    let mut in_marker_section = false;
    let mut marked_cols: HashSet<String> = HashSet::new();
    let mut current_qcmatrix_constraint: Option<String> = None; // Track which constraint for QCMATRIX
    let mut did_find_obj_sense: bool = false;

    for line in lines {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('*') {
            continue;
        }

        // Check for section change
        if let Some(new_section) = Section::try_parse(line) {
            section = new_section;

            // Handle NAME section inline
            if section == Section::Name {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    problem.name = Some(parts[1].to_string());
                }
            }

            // Handle OBJSENSE section inline
            if section == Section::Objsense {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1
                    && let Some(sense) = parse_sense(parts[1])
                {
                    problem.sense = sense;
                    did_find_obj_sense = true;
                }
            }

            // Handle QCMATRIX section - extract constraint name
            if section == Section::Qcmatrix {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    current_qcmatrix_constraint = Some(parts[1].to_string());
                } else {
                    return Err(LunaModelError::Translation(
                        "QCMATRIX section requires constraint name".into(),
                    ));
                }
            }

            continue;
        }

        match section {
            Section::None => {
                return Err(LunaModelError::Translation(
                    format!("Unexpected content before first section: {}", line).into(),
                ));
            }
            Section::Name => {
                // NAME already handled above
            }
            Section::Objsense => {
                if !did_find_obj_sense {
                    if let Some(sense) = parse_sense(&line.trim().to_uppercase()) {
                        problem.sense = sense;
                    } else {
                        return Err(LunaModelError::Translation(
                            format!("Unexpected sense: {}", line).into(),
                        ));
                    }
                }
            }
            Section::Rows => {
                parse_row_line(line, &mut rows_map)?;
            }
            Section::Columns => {
                parse_column_line(
                    line,
                    &mut column_data,
                    &mut in_marker_section,
                    &mut problem.integers,
                    &mut marked_cols,
                )?;
            }
            Section::Rhs => {
                parse_rhs_line(line, &mut rhs_data)?;
            }
            Section::Bounds => {
                parse_bounds_line(
                    line,
                    &mut problem.bounds,
                    &mut problem.binaries,
                    &mut problem.integers,
                )?;
            }
            Section::Quadobj => {
                // Parse quadratic objective terms: var1 var2 coefficient
                parse_quadratic_line(line, &mut problem.objective_quadratic, true)?;
            }
            Section::Qcmatrix => {
                // Parse quadratic constraint terms: var1 var2 coefficient
                if let Some(ref constraint_name) = current_qcmatrix_constraint {
                    let qterms = problem
                        .constraint_quadratic
                        .entry(constraint_name.clone())
                        .or_insert_with(Vec::new);
                    parse_quadratic_line(line, qterms, false)?;
                } else {
                    return Err(LunaModelError::Translation(
                        "QCMATRIX data without constraint name".into(),
                    ));
                }
            }
            Section::End => {
                break;
            }
        }
    }

    for colname in marked_cols {
        problem.bounds.entry(colname).or_insert_with(|| {
            Some(LazyBounds::new(
                Some(Bound::Bounded(0.0)),
                Some(Bound::Bounded(1.0)),
            ))
        });
    }

    // Build the problem from parsed data
    build_problem(&mut problem, rows_map, column_data, rhs_data)?;

    Ok(problem)
}

fn parse_row_line(line: &str, rows_map: &mut HashMap<String, MpsRow>) -> LunaModelResult<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(LunaModelError::Translation(
            format!("Invalid ROWS line: {}", line).into(),
        ));
    }

    let row_type_char = parts[0].chars().next().unwrap();
    let row_type = RowType::from_char(row_type_char)?;
    let row_name = parts[1].to_string();

    rows_map.insert(
        row_name.clone(),
        MpsRow {
            row_type,
            name: row_name,
        },
    );

    Ok(())
}

fn parse_column_line(
    line: &str,
    column_data: &mut HashMap<String, HashMap<String, Bias>>,
    in_marker_section: &mut bool,
    integers: &mut HashSet<String>,
    marked_cols: &mut HashSet<String>,
) -> LunaModelResult<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.is_empty() {
        return Ok(());
    }

    // Check for integer markers
    if parts.len() >= 3 && parts[1].contains("MARKER") {
        if parts[2].contains("INTORG") {
            *in_marker_section = true;
        } else if parts[2].contains("INTEND") {
            *in_marker_section = false;
        }
        return Ok(());
    }

    if parts.len() < 3 {
        return Err(LunaModelError::Translation(
            format!("Invalid COLUMNS line: {}", line).into(),
        ));
    }

    let var_name = parts[0].to_string();

    if *in_marker_section {
        integers.insert(var_name.clone());
        marked_cols.insert(var_name.clone());
        // let entry = bounds
        //     .entry(var_name.clone())
        //     .or_insert(Some(LazyBounds::default()));
        // let bnds: &mut LazyBounds = entry.as_mut().unwrap();
        // if bnds.lower.is_none() {
        //     bnds.lower = Some(Bound::Bounded(0.0));
        // }
        // if bnds.upper.is_none() {
        //     bnds.upper = Some(Bound::Bounded(1.0));
        // }
    }

    // Parse first coefficient pair
    let row_name1 = parts[1].to_string();
    let coef1: Bias = parts[2].parse().map_err(|_| {
        LunaModelError::Translation(format!("Invalid coefficient: {}", parts[2]).into())
    })?;

    column_data
        .entry(var_name.clone())
        .or_default()
        .insert(row_name1, coef1);

    // Parse optional second coefficient pair
    if parts.len() >= 5 {
        let row_name2 = parts[3].to_string();
        let coef2: Bias = parts[4].parse().map_err(|_| {
            LunaModelError::Translation(format!("Invalid coefficient: {}", parts[4]).into())
        })?;

        column_data
            .entry(var_name)
            .or_default()
            .insert(row_name2, coef2);
    }

    Ok(())
}

fn parse_rhs_line(line: &str, rhs_data: &mut HashMap<String, Bias>) -> LunaModelResult<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 3 {
        return Err(LunaModelError::Translation(
            format!("Invalid RHS line: {}", line).into(),
        ));
    }

    // parts[0] is the RHS name (ignored for now)
    let row_name = parts[1].to_string();
    let value: Bias = parts[2].parse().map_err(|_| {
        LunaModelError::Translation(format!("Invalid RHS value: {}", parts[2]).into())
    })?;

    rhs_data.insert(row_name, value);

    // Parse optional second RHS pair
    if parts.len() >= 5 {
        let row_name2 = parts[3].to_string();
        let value2: Bias = parts[4].parse().map_err(|_| {
            LunaModelError::Translation(format!("Invalid RHS value: {}", parts[4]).into())
        })?;
        rhs_data.insert(row_name2, value2);
    }

    Ok(())
}

fn parse_bounds_line(
    line: &str,
    bounds: &mut HashMap<String, Option<LazyBounds>>,
    binaries: &mut HashSet<String>,
    integers: &mut HashSet<String>,
) -> LunaModelResult<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 3 {
        return Err(LunaModelError::Translation(
            format!("Invalid BOUNDS line: {}", line).into(),
        ));
    }

    let bound_type = BoundType::try_parse(parts[0]).ok_or_else(|| {
        LunaModelError::Translation(format!("Invalid bound type: {}", parts[0]).into())
    })?;

    // parts[1] is the bound name (ignored)
    let var_name = parts[2].to_string();

    let base = bounds
        .entry(var_name.clone())
        .or_insert(Some(LazyBounds::new(
            Some(Bound::Bounded(0.0)),
            Some(Bound::Unbounded),
        )))
        .as_mut()
        .unwrap();

    // let mut base = LazyBounds::new(Some(Bound::Bounded(0.0)), Some(Bound::Unbounded));

    match bound_type {
        BoundType::LowerBound => {
            if parts.len() < 4 {
                return Err(LunaModelError::Translation(
                    format!("Missing value for LO bound: {}", line).into(),
                ));
            }
            let value: Bias = parts[3].parse().map_err(|_| {
                LunaModelError::Translation(format!("Invalid bound value: {}", parts[3]).into())
            })?;
            base.lower = Some(Bound::Bounded(value));
        }
        BoundType::UpperBound => {
            if parts.len() < 4 {
                return Err(LunaModelError::Translation(
                    format!("Missing value for UP bound: {}", line).into(),
                ));
            }
            let value: Bias = parts[3].parse().map_err(|_| {
                LunaModelError::Translation(format!("Invalid bound value: {}", parts[3]).into())
            })?;
            base.upper = Some(Bound::Bounded(value));
        }
        BoundType::Fixed => {
            if parts.len() < 4 {
                return Err(LunaModelError::Translation(
                    format!("Missing value for FX bound: {}", line).into(),
                ));
            }
            let value: Bias = parts[3].parse().map_err(|_| {
                LunaModelError::Translation(format!("Invalid bound value: {}", parts[3]).into())
            })?;
            base.lower = Some(Bound::Bounded(value));
            base.upper = Some(Bound::Bounded(value));
        }
        BoundType::Free => {
            base.lower = Some(Bound::Unbounded);
            base.upper = Some(Bound::Unbounded);
        }
        BoundType::Binary => {
            binaries.insert(var_name.clone());
            base.lower = Some(Bound::Bounded(0.0));
            base.upper = Some(Bound::Bounded(1.0));
        }
        BoundType::LowerInteger => {
            if parts.len() < 4 {
                return Err(LunaModelError::Translation(
                    format!("Missing value for LI bound: {}", line).into(),
                ));
            }
            let value: Bias = parts[3].parse().map_err(|_| {
                LunaModelError::Translation(format!("Invalid bound value: {}", parts[3]).into())
            })?;
            integers.insert(var_name.clone());
            base.lower = Some(Bound::Bounded(value));
        }
        BoundType::UpperInteger => {
            if parts.len() < 4 {
                return Err(LunaModelError::Translation(
                    format!("Missing value for UI bound: {}", line).into(),
                ));
            }
            let value: Bias = parts[3].parse().map_err(|_| {
                LunaModelError::Translation(format!("Invalid bound value: {}", parts[3]).into())
            })?;
            integers.insert(var_name.clone());
            base.upper = Some(Bound::Bounded(value));
        }
        BoundType::MinusInfinity => {
            base.lower = Some(Bound::Unbounded);
        }
        BoundType::PlusInfinity => {
            base.upper = Some(Bound::Unbounded);
        }
    }

    Ok(())
}

fn build_problem(
    problem: &mut MpsProblem,
    rows_map: HashMap<String, MpsRow>,
    column_data: HashMap<String, HashMap<String, Bias>>,
    rhs_data: HashMap<String, Bias>,
) -> LunaModelResult<()> {
    // Find objective row (first N type row, default to MIN)
    let objective_row = rows_map
        .values()
        .find(|row| row.row_type == RowType::Objective)
        .ok_or_else(|| LunaModelError::Translation("No objective row found in MPS file".into()))?;

    problem.objective_name = Some(objective_row.name.clone());
    // Note: sense may have been set by OBJSENSE section, don't override if already set
    // Only set to Min if it's still the default (which is also Min, so this is a no-op)
    // The sense was already initialized to Min in MpsProblem::default()

    // Build objective coefficients
    for (var_name, row_coeffs) in &column_data {
        problem.vars.insert(var_name.clone());

        if let Some(coef) = row_coeffs.get(&objective_row.name) {
            problem.objective.insert(var_name.clone(), *coef);
        }
    }

    for var_name in &problem.integers {
        if !problem.bounds.contains_key(var_name) {
            problem.bounds.insert(
                var_name.clone(),
                Some(LazyBounds::new(
                    Some(Bound::Bounded(0.0)),
                    Some(Bound::Bounded(1.0)),
                )),
            );
        }
    }

    // Build constraints
    for (row_name, row) in rows_map.iter() {
        let rhs = rhs_data.get(row_name).copied().unwrap_or(0.0);

        if row.row_type == RowType::Objective {
            problem.objective_constant = rhs;
            continue;
        }

        let mut coefficients = HashMap::new();
        for (var_name, row_coeffs) in &column_data {
            if let Some(coef) = row_coeffs.get(row_name) {
                coefficients.insert(var_name.clone(), *coef);
            }
        }

        problem.constraints.push(MpsConstraint {
            name: row_name.clone(),
            row_type: row.row_type,
            coefficients,
            rhs,
        });
    }

    problem.rows = rows_map;

    Ok(())
}

fn parse_quadratic_line(
    line: &str,
    quad_terms: &mut Vec<(String, String, Bias)>,
    is_obj: bool,
) -> LunaModelResult<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 3 {
        return Err(LunaModelError::Translation(
            format!("Invalid quadratic line (expected 3 parts): {}", line).into(),
        ));
    }

    let var1 = parts[0].to_string();
    let var2 = parts[1].to_string();
    let mut coef: Bias = parts[2].parse().map_err(|_| {
        LunaModelError::Translation(format!("Invalid quadratic coefficient: {}", parts[2]).into())
    })?;

    if is_obj && var1 == var2 {
        coef *= 0.5;
    }

    quad_terms.push((var1, var2, coef));

    Ok(())
}

fn parse_sense(sense_str: &str) -> Option<Sense> {
    // Parse OBJSENSE: MAX, MAXIMIZE, MIN, MINIMIZE
    if sense_str == "MAX" || sense_str == "MAXIMIZE" {
        Some(Sense::Max)
    } else if sense_str == "MIN" || sense_str == "MINIMIZE" {
        Some(Sense::Min)
    } else {
        None
    }
}
