use super::keywords::{
    BoundsKeywords, ConstraintsKeywords, ObjectiveKeywords, VariableTypeKeywords,
};
use super::{
    exprtree::{EvalContext, ExprTree},
    keywords::VariableType,
    util::starts_with_any,
};
use crate::core::environment::get_vref_by_name;
use crate::core::{Bound, LazyBounds, Sense, Vtype, DEFAULT_MODEL_NAME};
use crate::{
    core::{
        environment::add_variable,
        expression::{BiasConstraints, ExpressionBaseCreation, IndexConstraints},
        operations::AddAssignToExpression,
        Bounds, Comparator, Constraint, Expression, Model, VarRef,
    },
    errors::TranslationErr,
};
use hashbrown::{hash_map::Iter, HashMap};
use regex::Regex;
use strum_macros::Display;

use std::{cell::RefCell, hash::Hash, marker::PhantomData, ops::AddAssign, rc::Rc};

#[derive(Debug, Clone, PartialEq, Display, Eq, Hash)]
pub enum Section {
    /// Placeholder
    Placeholder,
    /// Single-Objective Case
    ///
    /// Let us consider single-objective models first, where this header is followed by
    /// a single linear or quadratic expression that captures the objective function.
    ///
    /// The objective optionally begins with a label. A label consists of a name,
    /// followed by a colon character, following by a space. A space is allowed between
    /// the name and the colon, but not required.
    ///
    /// The objective then continues with a list of linear terms, separated by the + or
    /// - operators. A term can contain a coefficient and a variable (e.g., 4.5 x), or
    /// just a variable (e.g., x). The objective can be spread over many lines, or it
    /// may be listed on a single line. Line breaks can come between tokens, but never
    /// within tokens.
    ///
    /// The objective may optionally continue with a list of quadratic terms. The
    /// quadratic portion of the objective expression begins with a [ symbol and ends
    /// with a ] symbol, followed by / 2. These brackets should enclose one or more
    /// quadratic terms. Either squared terms (e.g., 2 x ^ 2) or product terms
    /// (e.g., 3 x * y) are accepted. Coefficients on the quadratic terms are optional.
    ///
    /// For variables with piecewise-linear objective functions, the objective section
    /// will include a __pwl(x) term, where x is the name of the variable. The actual
    /// piecewise-linear expressions are pulled from the later PWLObj section.
    ///
    /// The objective expression must always end with a line break.
    ///
    /// An objective section might look like the following:
    ///
    /// Minimize
    ///   obj: 3.1 x + 4.5 y + 10 z + [ x ^ 2 + 2 x * y + 3 y ^ 2 ] / 2
    ///
    Objective(Sense),
    /// The next section is the constraints section. It begins with one of the following
    /// headers, on its own line: subject to, such that, st, or s.t..
    /// Capitalization is ignored.
    ///
    /// The constraint section can have an arbitrary number of constraints. Each
    /// constraint starts with an optional label (constraint name, followed by a colon,
    /// followed by a space), continues with a linear expression, followed by an optional
    /// quadratic expression (enclosed in square brackets), and ends with a comparison
    /// operator, followed by a numerical value, followed by a line break. Valid comparison
    /// operators are =, <=, <, >=, or >. Note that LP format does not distinguish between
    /// strict and non-strict inequalities, so for example < and <= are equivalent.
    ///
    /// Note that the left-hand side of a constraint may not contain a constant term;
    /// the constant must appear on the right-hand side.
    ///
    /// The following is a simple example of a valid linear constraint:
    ///
    /// c0: 2.5 x + 2.3 y + 5.3 z <= 8.1
    ///
    /// The following is a valid quadratic constraint:
    ///
    /// qc0: 3.1 x + 4.5 y + 10 z + [ x ^ 2 + 2 x * y + 3 y ^ 2 ] <= 10
    ///
    /// The constraint section may also contain another constraint type: the so-called
    /// indicator constraint. Indicator constraints start with an optional label
    /// (constraint name, followed by a colon, followed by a space), followed by a
    /// binary variable, a space, a =, again a space and a value, either 0 or 1.
    /// They continue with a space, followed by ->, and again a space and finally a
    /// linear constraint (without a label).
    ///
    /// For example:
    ///
    /// c0: b1 = 1 -> 2.5 x + 2.3 y + 5.3 z <= 8.1
    ///
    /// This example constraint requires the given linear constraint to be satisfied if
    /// the variable b1 takes a value of 1.
    ///
    /// Every LP format file must have a constraints section.
    Constraints,
    /// Bounds Section
    /// The next section is the bounds section. It begins with the word Bounds, on its
    /// own line, and is followed by a list of variable bounds. Each line specifies the
    /// lower bound, the upper bound, or both for a single variable. The keywords inf or
    /// infinity can be used in the bounds section to specify infinite bounds. A bound
    /// line can also indicate that a variable is free, meaning that it is unbounded in
    /// either direction.
    ///
    /// Here are examples of valid bound lines:
    ///
    /// Bounds
    ///   0 <= x0 <= 1
    ///   x1 <= 1.2
    ///   x2 >= 3
    ///   x3 free
    ///   x2 >= -Inf
    ///
    /// It is not necessary to specify bounds for all variables; by default, each
    /// variable has a lower bound of 0 and an infinite upper bound. In fact, the entire
    /// bounds section is optional.
    Bounds,
    /// Variable Type Section
    ///
    /// The next section is the variable types section. Variables can be designated as
    /// being either binary, general integer, or semi-continuous. In all cases, the
    /// designation is applied by first providing the appropriate header (on its own line),
    /// and then listing the variables that have the associated type. For example:
    ///
    /// Binary
    ///   x y z
    ///
    /// Variable type designations don’t need to appear in any particular order
    /// (e.g., general integers can either precede or follow binaries). If a variable is
    /// included in multiple sections, the last one determines the variable type.
    ///
    /// Valid keywords for variable type headers are: binary, binaries, bin, general,
    /// generals, gen, semi-continuous, semis, or semi.
    ///
    /// The variable types section is optional. By default, variables are assumed to be
    /// continuous.
    VariableType(VariableType),
}

impl Section {
    pub fn detect(line: &str) -> (Option<Section>, Option<&str>) {
        if starts_with_any(line, &ObjectiveKeywords::all_min()) {
            (Some(Section::Objective(Sense::Min)), None)
        } else if starts_with_any(line, &ObjectiveKeywords::all_max()) {
            (Some(Section::Objective(Sense::Max)), None)
        } else if starts_with_any(line, &ConstraintsKeywords::all()) {
            (Some(Section::Constraints), None)
        } else if starts_with_any(line, &BoundsKeywords::all()) {
            (Some(Section::Bounds), None)
        } else if starts_with_any(line, &VariableTypeKeywords::all_bin()) {
            (Some(Section::VariableType(VariableType::Binary)), None)
        } else if starts_with_any(line, &VariableTypeKeywords::all_gen()) {
            (Some(Section::VariableType(VariableType::General)), None)
        } else if starts_with_any(line, &VariableTypeKeywords::all_semi()) {
            (Some(Section::VariableType(VariableType::Semi)), None)
        } else {
            (None, Some(line.trim()))
        }
    }
}

#[derive(Debug)]
pub struct SectionsHolder<Index, Bias> {
    variable_sections: HashMap<VariableType, Vec<String>>,
    sections: HashMap<Section, Vec<String>>,
    pub model_name: Option<String>,
    _pi: PhantomData<Index>,
    _pb: PhantomData<Bias>,
}

impl<Index, Bias> SectionsHolder<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            variable_sections: HashMap::new(),
            model_name: None,
            _pb: PhantomData,
            _pi: PhantomData,
        }
    }

    pub fn get_objective_str(&self) -> Result<(String, &Vec<String>), TranslationErr> {
        let min_obj = self.get(Section::Objective(Sense::Min));
        let max_obj = self.get(Section::Objective(Sense::Max));
        match (min_obj, max_obj) {
            (None, Some(max_obj)) => Ok((ObjectiveKeywords::Maximize.to_string(), max_obj)),
            (Some(min_obj), None) => Ok((ObjectiveKeywords::Minimize.to_string(), min_obj)),
            _ => Err(TranslationErr::new(format!("unexpected objectives stored"))),
        }
    }

    pub fn from_model(model: &Model<Index, Bias>) -> Result<Self, TranslationErr> {
        let mut sections = Self::new();
        if model.name != DEFAULT_MODEL_NAME {
            sections.model_name = Some(model.name.clone());
        }
        // variables & bounds
        for v in model.environment.borrow().iter() {
            match v.vtype {
                Vtype::Binary => {
                    sections.push(&Section::VariableType(VariableType::Binary), v.name.clone())
                }
                Vtype::Spin => {
                    return Err(TranslationErr::new(
                        "LP files cannot contain variables of type SPIN".to_string(),
                    ))
                }
                Vtype::Integer => sections.push(
                    &Section::VariableType(VariableType::General),
                    v.name.clone(),
                ),
                Vtype::Real => {
                    sections.push(&Section::VariableType(VariableType::Semi), v.name.clone())
                }
            }
            if v.vtype != Vtype::Binary {
                // Binary bounds are fixed...does not make sense to change them.
                sections.push(&Section::Bounds, parse_bounds(&v.name, &v.bounds));
            }
        }
        // objective
        sections.push(
            &Section::Objective(model.sense),
            ExprTree::from_expression(&model.objective.borrow(), false)?
                .optimize()
                .to_string(true),
        );
        // constraints
        for (i, constraint) in model.constraints.borrow().iter().enumerate() {
            let lhs_str = ExprTree::from_expression(&constraint.lhs.borrow(), true)?
                .optimize()
                .to_string(false);
            let comparator = match constraint.comparator {
                Comparator::Eq => "=",
                Comparator::Le => "<=",
                Comparator::Ge => ">=",
            };
            sections.push(
                &Section::Constraints,
                format!(
                    "{}: {} {} {}",
                    constraint.name.clone().unwrap_or(format!("c{i}")),
                    lhs_str,
                    comparator,
                    constraint.rhs
                ),
            );
        }
        // end
        Ok(sections)
    }

    pub fn put(&mut self, section: &Section) {
        match section {
            Section::VariableType(vt) => self.put_variable_section(*vt),
            _ => self.put_section(section.clone()),
        }
    }

    fn put_section(&mut self, section: Section) {
        self.sections.insert(section, Vec::new());
    }

    fn put_variable_section(&mut self, vtype: VariableType) {
        self.variable_sections.insert(vtype, Vec::new());
    }

    pub fn push(&mut self, section: &Section, value: String) {
        match section {
            Section::VariableType(vt) => self.push_variable_section(*vt, value),
            _ => self.push_section(section, value),
        }
    }

    fn push_section(&mut self, section: &Section, value: String) {
        match &section {
            Section::Objective(Sense::Min) | Section::Objective(Sense::Max) => {
                // something
                match self.sections.get_mut(section) {
                    Some(item) => match item.first_mut() {
                        Some(v) => v.push_str(&value),
                        None => item.push(value),
                    },
                    None => {
                        let _ = self.sections.insert(section.clone(), vec![value.clone()]);
                    }
                }
            }
            Section::Constraints => {
                if value.contains(":") {
                    // new constraint overwrites old constraints
                    match self.sections.get_mut(section) {
                        Some(item) => item.push(value),
                        None => {
                            let _ = self.sections.insert(section.clone(), vec![value]);
                        }
                    }
                } else {
                    // new line of last constraint
                    self.sections
                        .get_mut(section)
                        .unwrap()
                        .last_mut()
                        .unwrap()
                        .push_str(&value);
                }
            }
            _ => match self.sections.get_mut(section) {
                Some(item) => item.push(value),
                None => {
                    let _ = self.sections.insert(section.clone(), vec![value]);
                }
            },
        }
    }

    fn push_variable_section(&mut self, vtype: VariableType, value: String) {
        let vals = value.split_whitespace();
        for val in vals {
            match self.variable_sections.get_mut(&vtype) {
                Some(item) => item.push(val.to_string()),
                None => {
                    let _ = self
                        .variable_sections
                        .insert(vtype.clone(), vec![val.to_string()]);
                }
            }
        }
    }

    pub fn get(&self, section: Section) -> Option<&Vec<String>> {
        self.sections.get(&section)
    }

    pub fn iter_variables(&self) -> Iter<VariableType, Vec<String>> {
        self.variable_sections.iter()
    }

    fn extract_bounds(&self) -> Option<HashMap<String, (Bound, Bound)>> {
        if let Some(bounds) = self.get(Section::Bounds) {
            let mut boundsmap: HashMap<String, (Bound, Bound)> = HashMap::new();
            for entry in bounds.iter() {
                if entry.contains("free") {
                    let var = entry.replace("free", "").trim().to_string();
                    boundsmap.insert(var, (Bound::Unbounded(), Bound::Unbounded()));
                    continue;
                }
                let parts: Vec<&str> = entry.split_whitespace().collect();
                match parts.as_slice() {
                    // Format: _ <= var <= _
                    [lower, "<=", var, "<=", upper] => {
                        boundsmap.insert(
                            var.to_string(),
                            (parse_bound_value(lower), parse_bound_value(upper)),
                        );
                    }
                    // Format: var <= upper
                    [var, "<=", upper] => {
                        boundsmap.insert(
                            var.to_string(),
                            (Bound::Unbounded(), parse_bound_value(upper)),
                        );
                    }
                    // Format: var >= lower
                    [var, ">=", lower] => {
                        boundsmap.insert(
                            var.to_string(),
                            (parse_bound_value(lower), Bound::Unbounded()),
                        );
                    }
                    _ => (),
                }
            }
            Some(boundsmap)
        } else {
            None
        }
    }

    pub fn make_variables(
        &self,
        model: &mut Model<Index, Bias>,
    ) -> Result<HashMap<String, VarRef<Index>>, TranslationErr> {
        let mut varlookup = HashMap::new();
        let mut boundsmap = self.extract_bounds();
        for (vtype, vars) in self.iter_variables() {
            for var in vars {
                let bounds: Option<Bounds> = match boundsmap {
                    Some(ref mut bm) => match bm.remove(var) {
                        Some((l, u)) => match vtype {
                            // We ignore the set bounds for the case of binary variables
                            // as we do not allow setting the bounds in case of Binary
                            // or string
                            VariableType::Binary => None,
                            _ => Some(Bounds::new(l, u)),
                        },
                        None => None,
                    },
                    None => None,
                };
                let vref = add_variable(
                    Rc::clone(&model.environment),
                    var,
                    Some(&(*vtype).into()),
                    bounds.map(|b| b.into()),
                )
                .map_err(|e| TranslationErr::new(e.to_string()))?;
                varlookup.insert(var.to_string(), vref);
            }
        }
        if let Some(ref mut bm) = boundsmap {
            if !bm.is_empty() {
                for (var, (lower, upper)) in bm.iter() {
                    let vref = add_variable(
                        Rc::clone(&model.environment),
                        var,
                        Some(&Vtype::Real),
                        Some(LazyBounds::new(Some(*lower), Some(*upper))),
                    )
                    .map_err(|e| TranslationErr::new(e.to_string()))?;
                    varlookup.insert(var.to_string(), vref);
                }
            }
        }
        Ok(varlookup)
    }

    pub fn make_objective(
        &self,
        model: &mut Model<Index, Bias>,
        vars: &HashMap<String, VarRef<Index>>,
    ) -> Result<(), TranslationErr> {
        let min_obj = self.get(Section::Objective(Sense::Min));
        let max_obj = self.get(Section::Objective(Sense::Max));
        let (sense, obj): (Sense, &Vec<String>) = match (min_obj, max_obj) {
            (Some(o), None) => (Sense::Min, o),
            (None, Some(o)) => (Sense::Max, o),
            (Some(_), Some(_)) => {
                return Err(TranslationErr::new(String::from(
                    "cannot have multiple objectives in model",
                )))
            }
            (None, None) => {
                return Err(TranslationErr::new(String::from(
                    "must have an objective in model",
                )))
            }
        };
        model.set_sense(sense);
        let all = obj.concat();
        Self::add_to_expression(&mut model.objective.borrow_mut(), &all, &vars)?;
        Ok(())
    }

    pub fn make_constraints(
        &self,
        model: &mut Model<Index, Bias>,
        vars: &HashMap<String, VarRef<Index>>,
    ) -> Result<(), TranslationErr> {
        if let Some(constrs) = self.get(Section::Constraints) {
            for entry in constrs {
                let (name, constr) = entry.split_once(":").unwrap();
                if let Some((lhs_str, comp, rhs_str)) = Self::split_constraint_expression(&constr) {
                    let mut lhs: Expression<Index, Bias> = Expression::new(
                        Rc::clone(&model.environment),
                        vec![false; model.objective.borrow().active.len()],
                        model.objective.borrow().num_variables,
                    );
                    Self::add_to_expression(&mut lhs, &lhs_str, &vars)?;
                    let rhs = rhs_str.parse::<Bias>().map_err(|_| {
                        TranslationErr::new(format!("cannot convert rhs to f64: {}", rhs_str))
                    })?;
                    let c = match comp {
                        "=" => Constraint::new(
                            Rc::new(RefCell::new(lhs)),
                            rhs,
                            Comparator::Eq,
                            Some(name.to_string()),
                        )?,
                        "<=" => Constraint::new(
                            Rc::new(RefCell::new(lhs)),
                            rhs,
                            Comparator::Le,
                            Some(name.to_string()),
                        )?,
                        ">=" => Constraint::new(
                            Rc::new(RefCell::new(lhs)),
                            rhs,
                            Comparator::Ge,
                            Some(name.to_string()),
                        )?,
                        _ => {
                            return Err(TranslationErr::new(format!(
                                "unknown comparator '{}' for constraint '{}'",
                                comp, name
                            )))
                        }
                    };
                    model.constraints.borrow_mut().add_assign(c);
                } else {
                    return Err(TranslationErr::new(format!(
                        "malformed constraint: {}",
                        name
                    )));
                }
            }
        }
        Ok(())
    }

    fn add_to_expression(
        expr: &mut Expression<Index, Bias>,
        expr_str: &str,
        vars: &HashMap<String, VarRef<Index>>,
    ) -> Result<(), TranslationErr> {
        let mut expression = ExprTree::build(&expr_str);
        expression = expression.optimize();
        let expression = expression.evaluate(&EvalContext::new(
            |n| {
                let mut var: Option<VarRef<_>> = vars.get(n).cloned(); // .unwrap().clone()
                if var.is_none() {
                    let res = get_vref_by_name(&n.to_string(), Rc::clone(&expr.env));
                    var = if let Ok(v) = res {
                        Some(v)
                    } else {
                        add_variable(
                            Rc::clone(&expr.env),
                            &n.to_string(),
                            Some(&Vtype::Real),
                            None,
                        )
                        .ok()
                    };
                }
                var.unwrap()
            },
            Rc::clone(&expr.env),
        ))?;
        expr.add_assign(&expression)?;
        Ok(())
    }

    fn split_constraint_expression(expr: &str) -> Option<(&str, &str, &str)> {
        // Matches <=, >=, or = with optional surrounding spaces
        let re = Regex::new(r"^(.*?)\s*(<=|>=|=)\s*(.*)$").unwrap();
        re.captures(expr).map(|caps| {
            (
                caps.get(1).unwrap().as_str().trim(),
                caps.get(2).unwrap().as_str().trim(),
                caps.get(3).unwrap().as_str().trim(),
            )
        })
    }
}

fn parse_bound_value(s: &str) -> Bound {
    match s {
        "inf" | "infinity" => Bound::Unbounded(),
        "-inf" | "-infinity" => Bound::Unbounded(),
        _ => s
            .parse::<f64>()
            .ok()
            .map_or(Bound::Unbounded(), |v| Bound::Some(v)),
    }
}

fn parse_bounds(v: &str, bounds: &Bounds) -> String {
    match (bounds.lower, bounds.upper) {
        (Bound::Unbounded(), Bound::Unbounded()) => format!("{} free", v),
        (Bound::Some(lower), Bound::Unbounded()) => format!("{} >= {}", v, lower),
        (Bound::Unbounded(), Bound::Some(upper)) => format!("{} <= {}", v, upper),
        (Bound::Some(lower), Bound::Some(upper)) => format!("{} <= {} <= {}", lower, v, upper),
    }
}
