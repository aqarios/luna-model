//! MPS back-translator from core models to text.

use lunamodel_core::{Model, prelude::Bounds};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Bound, Comparator, Sense, Vtype};
use std::{collections::BTreeMap, fs::File, io::Write, path::PathBuf};

use super::MpsTranslator;

impl MpsTranslator {
    pub fn back_translate(
        model: &Model,
        filepath: Option<PathBuf>,
    ) -> LunaModelResult<Option<String>> {
        let mps_str = Self::build_string(model)?;
        if let Some(pb) = filepath {
            Self::write_file(mps_str, &pb)?;
            Ok(None)
        } else {
            Ok(Some(mps_str))
        }
    }

    pub fn write_file(data: String, filepath: &PathBuf) -> LunaModelResult<()> {
        let mut file = File::create(filepath)
            .map_err(|why| LunaModelError::Translation(why.to_string().into()))?;
        file.write_all(data.as_bytes())
            .map_err(|why| LunaModelError::Translation(why.to_string().into()))?;
        Ok(())
    }

    fn build_string(model: &Model) -> LunaModelResult<String> {
        let mut out = String::new();

        let name = &model.name;
        out.push_str(&format!("NAME          {}\n", name));

        match model.sense {
            Sense::Min => {
                out.push_str("OBJSENSE\n");
                out.push_str(" MIN\n");
            }
            Sense::Max => {
                out.push_str("OBJSENSE\n");
                out.push_str(" MAX\n");
            }
        }

        out.push_str("ROWS\n");
        out.push_str(" N  OBJ\n");

        for (idx, (_key, constraint)) in model.constraints.iter().enumerate() {
            let row_type = match constraint.comparator {
                Comparator::Le => "L",
                Comparator::Ge => "G",
                Comparator::Eq => "E",
            };
            let cname = constraint.name();
            let cname_str = if cname.is_empty() {
                format!("C{}", idx)
            } else {
                cname.to_string()
            };
            out.push_str(&format!(" {}  {}\n", row_type, cname_str));
        }

        // Step 3: COLUMNS section
        out.push_str("COLUMNS\n");

        // Collect variable coefficients
        let mut var_coeffs: BTreeMap<String, Vec<(String, Bias)>> = BTreeMap::new();
        let mut integer_vars = Vec::new();

        // Collect variable types
        for v in model.environment.vars() {
            let vname = v.name()?;
            match v.vtype()? {
                Vtype::Integer => integer_vars.push(vname.clone()),
                Vtype::Binary | Vtype::Real => (),
                Vtype::Spin | Vtype::InvertedBinary => {
                    return Err(LunaModelError::Translation(
                        format!(
                            "MPS files cannot contain variables of type {:?}",
                            v.vtype()?
                        )
                        .into(),
                    ));
                }
            }
        }

        // Collect objective coefficients (linear and quadratic separately)
        let mut quadratic_obj_terms = Vec::new();
        for (vars, coef) in model.objective.items() {
            if vars.len() > 2 {
                return Err(LunaModelError::Translation(
                    "MPS files cannot contain higher-order objective terms".into(),
                ));
            }
            if vars.len() == 1 {
                let vname = vars[0].name()?;
                var_coeffs
                    .entry(vname.clone())
                    .or_default()
                    .push(("OBJ".to_string(), coef));
            } else if vars.len() == 2 {
                // Quadratic term
                let v1 = vars[0].name()?;
                let v2 = vars[1].name()?;
                quadratic_obj_terms.push((v1, v2, coef));
            }
        }

        // Collect constraint coefficients (linear and quadratic separately)
        let mut quadratic_constraint_terms: BTreeMap<String, Vec<(String, String, Bias)>> =
            BTreeMap::new();
        for (idx, (_key, constraint)) in model.constraints.iter().enumerate() {
            let cname_raw = constraint.name();
            let cname = if cname_raw.is_empty() {
                format!("C{}", idx)
            } else {
                cname_raw.to_string()
            };

            for (vars, coef) in constraint.lhs.items() {
                if vars.len() > 2 {
                    return Err(LunaModelError::Translation(
                        "MPS files cannot contain higher-order constraint terms".into(),
                    ));
                }
                if vars.len() == 1 {
                    let vname = vars[0].name()?;
                    var_coeffs
                        .entry(vname.clone())
                        .or_default()
                        .push((cname.clone(), coef));
                } else if vars.len() == 2 {
                    // Quadratic term
                    let v1 = vars[0].name()?;
                    let v2 = vars[1].name()?;
                    quadratic_constraint_terms
                        .entry(cname.clone())
                        .or_default()
                        .push((v1, v2, coef));
                }
            }
        }

        // Write columns with integer markers
        let mut sorted_vars: Vec<_> = var_coeffs.keys().cloned().collect();
        sorted_vars.sort();

        let mut in_integer_section = false;

        for vname in sorted_vars {
            let is_integer = integer_vars.contains(&vname);

            // Start integer section if needed
            if is_integer && !in_integer_section {
                out.push_str("    MARK0000  'MARKER'                 'INTORG'\n");
                in_integer_section = true;
            }

            // End integer section if needed
            if !is_integer && in_integer_section {
                out.push_str("    MARK0000  'MARKER'                 'INTEND'\n");
                in_integer_section = false;
            }

            // Write coefficients for this variable (1 per line)
            if let Some(coeffs) = var_coeffs.get(&vname) {
                for (row, coef) in coeffs {
                    let line = format!("    {}  {}  {}\n", vname, row, format_number(*coef));
                    out.push_str(&line);
                }
            }
        }

        // Close integer section if still open
        if in_integer_section {
            out.push_str("    MARK0000  'MARKER'                 'INTEND'\n");
        }

        // Step 4: RHS section
        out.push_str("RHS\n");
        for (idx, (_key, constraint)) in model.constraints.iter().enumerate() {
            let cname_raw = constraint.name();
            let cname = if cname_raw.is_empty() {
                format!("C{}", idx)
            } else {
                cname_raw.to_string()
            };
            out.push_str(&format!(
                "    RHS1      {}  {}\n",
                cname,
                format_number(constraint.rhs)
            ));
        }
        if model.objective.offset != 0.0 {
            out.push_str(&format!(
                "    RHS1      OBJ  {}\n",
                format_number(-model.objective.offset)
            ));
        }

        // Step 7: BOUNDS section
        out.push_str("BOUNDS\n");
        for v in model.vars() {
            let vname = v.name()?;
            match v.vtype()? {
                Vtype::Binary => {
                    out.push_str(&format!(" BV BND1      {}\n", vname));
                }
                Vtype::Integer => {
                    let Bounds { lower, upper } = v.bounds()?;
                    match (lower, upper) {
                        (Bound::Unbounded, Bound::Unbounded) => {
                            out.push_str(&format!(" FR BND1      {}\n", vname));
                        }
                        (Bound::Bounded(l), Bound::Unbounded) => {
                            out.push_str(&format!(
                                " LI BND1      {}  {}\n",
                                vname,
                                format_number(l)
                            ));
                        }
                        (Bound::Unbounded, Bound::Bounded(u)) => {
                            out.push_str(&format!(" MI BND1      {}\n", vname));
                            out.push_str(&format!(
                                " UI BND1      {}  {}\n",
                                vname,
                                format_number(u)
                            ));
                        }
                        (Bound::Bounded(l), Bound::Bounded(u)) => {
                            if (l - u).abs() < f64::EPSILON {
                                out.push_str(&format!(
                                    " FX BND1      {}  {}\n",
                                    vname,
                                    format_number(l)
                                ));
                            } else {
                                out.push_str(&format!(
                                    " LI BND1      {}  {}\n",
                                    vname,
                                    format_number(l)
                                ));
                                out.push_str(&format!(
                                    " UI BND1      {}  {}\n",
                                    vname,
                                    format_number(u)
                                ));
                            }
                        }
                    }
                }
                Vtype::Real => {
                    let Bounds { lower, upper } = v.bounds()?;
                    let default_lower = Bound::Bounded(0.0);
                    let default_upper = Bound::Unbounded;

                    // Only write bounds if they're non-default
                    match (lower, upper) {
                        (Bound::Unbounded, Bound::Unbounded) => {
                            out.push_str(&format!(" FR BND1      {}\n", vname));
                        }
                        (l, u) if l != default_lower || u != default_upper => {
                            match l {
                                Bound::Bounded(val) if val != 0.0 => {
                                    out.push_str(&format!(
                                        " LO BND1      {}  {}\n",
                                        vname,
                                        format_number(val)
                                    ));
                                }
                                Bound::Unbounded => {
                                    out.push_str(&format!(" MI BND1      {}\n", vname));
                                }
                                _ => {}
                            }
                            if let Bound::Bounded(val) = u {
                                out.push_str(&format!(
                                    " UP BND1      {}  {}\n",
                                    vname,
                                    format_number(val)
                                ));
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Step 5: QUADOBJ section (if quadratic objective terms exist)
        if !quadratic_obj_terms.is_empty() {
            out.push_str("QUADOBJ\n");
            for (v1, v2, coef) in quadratic_obj_terms {
                // For diagonal terms (v1 == v2), MPS expects 2*coef
                let mps_coef = if v1 == v2 { coef * 2.0 } else { coef };
                out.push_str(&format!(
                    "    {}  {}  {}\n",
                    v1,
                    v2,
                    format_number(mps_coef)
                ));
            }
        }

        // Step 6: QCMATRIX sections (if quadratic constraint terms exist)
        for (cname, qterms) in quadratic_constraint_terms {
            out.push_str(&format!("QCMATRIX   {}\n", cname));
            for (v1, v2, coef) in qterms {
                if v1 == v2 {
                    out.push_str(&format!("    {}  {}  {}\n", v1, v2, format_number(coef)));
                } else {
                    out.push_str(&format!(
                        "    {}  {}  {}\n",
                        v1,
                        v2,
                        format_number(coef * 0.5)
                    ));
                    out.push_str(&format!(
                        "    {}  {}  {}\n",
                        v2,
                        v1,
                        format_number(coef * 0.5)
                    ));
                }
            }
        }

        out.push_str("ENDATA");
        Ok(out)
    }
}

fn format_number(n: Bias) -> String {
    if n.fract().abs() < f64::EPSILON {
        format!("{:.1}", n)
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::MpsTranslator;
    use lunamodel_core::{
        ArcEnv, Expression, Model,
        ops::{LmAddAssign, LmMulAssign},
        prelude::{Constraint, ContentEquality, LazyBounds},
    };
    use lunamodel_types::{Bound, Sense, Vtype};

    #[test]
    fn test_created_model_simple() {
        // Create a simple model: minimize x + 2*y subject to x + y <= 10
        let env = ArcEnv::default();
        env.write_arc().insert("x", Vtype::Real, None).unwrap();
        env.write_arc().insert("y", Vtype::Real, None).unwrap();

        let x = env.lookup("x").unwrap();
        let y = env.lookup("y").unwrap();

        // Objective: x + 2*y
        let mut objective = Expression::empty(env.clone());
        let mut term1 = Expression::constant(env.clone(), 1.0);
        term1.mul_assign(&x).unwrap();
        objective.add_assign(&term1).unwrap();

        let mut term2 = Expression::constant(env.clone(), 2.0);
        term2.mul_assign(&y).unwrap();
        objective.add_assign(&term2).unwrap();

        // Constraint: x + y <= 10
        let mut lhs = Expression::empty(env.clone());
        let mut t1 = Expression::constant(env.clone(), 1.0);
        t1.mul_assign(x).unwrap();
        lhs.add_assign(&t1).unwrap();

        let mut t2 = Expression::constant(env.clone(), 1.0);
        t2.mul_assign(y).unwrap();
        lhs.add_assign(&t2).unwrap();

        let constraint =
            Constraint::new(lhs, 10.0, Comparator::Le, Some("C1".to_string())).unwrap();

        let mut model1 = Model::with_env(Some("CREATED".to_string()), Some(Sense::Min), env);
        model1.objective = objective;
        model1.constraints.add_constraint(constraint, None).unwrap();

        // Back-translate to MPS
        let mps_output = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();

        println!("Created model back-translated:\n{}", mps_output);

        // Translate back to model
        let model2 = MpsTranslator::translate(mps_output).unwrap();

        // Check equality
        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_created_model_with_binary() {
        // Create model with binary variables
        let env = ArcEnv::default();
        env.write_arc().insert("x1", Vtype::Binary, None).unwrap();
        env.write_arc().insert("x2", Vtype::Binary, None).unwrap();

        let x1 = env.lookup("x1").unwrap();
        let x2 = env.lookup("x2").unwrap();

        // Objective: 3*x1 + 5*x2
        let mut objective = Expression::empty(env.clone());
        let mut term1 = Expression::constant(env.clone(), 3.0);
        term1.mul_assign(&x1).unwrap();
        objective.add_assign(&term1).unwrap();

        let mut term2 = Expression::constant(env.clone(), 5.0);
        term2.mul_assign(&x2).unwrap();
        objective.add_assign(&term2).unwrap();

        // Constraint: x1 + x2 = 1
        let mut lhs = Expression::empty(env.clone());
        let mut t1 = Expression::constant(env.clone(), 1.0);
        t1.mul_assign(x1).unwrap();
        lhs.add_assign(&t1).unwrap();

        let mut t2 = Expression::constant(env.clone(), 1.0);
        t2.mul_assign(x2).unwrap();
        lhs.add_assign(&t2).unwrap();

        let constraint =
            Constraint::new(lhs, 1.0, Comparator::Eq, Some("SELECT".to_string())).unwrap();

        let mut model1 = Model::with_env(Some("BINARY".to_string()), Some(Sense::Max), env);
        model1.objective = objective;
        model1.constraints.add_constraint(constraint, None).unwrap();

        // Back-translate to MPS
        let mps_output = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();

        println!("Binary model back-translated:\n{}", mps_output);

        // Translate back to model
        let model2 = MpsTranslator::translate(mps_output).unwrap();

        // Check equality
        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_created_model_with_integers() {
        // Create model with integer variables
        let env = ArcEnv::default();
        let bounds = LazyBounds::new(Some(Bound::Bounded(0.0)), Some(Bound::Bounded(10.0)));
        env.write_arc()
            .insert("i1", Vtype::Integer, Some(bounds))
            .unwrap();
        env.write_arc()
            .insert("i2", Vtype::Integer, Some(bounds))
            .unwrap();

        let i1 = env.lookup("i1").unwrap();
        let i2 = env.lookup("i2").unwrap();

        // Objective: 2*i1 + 3*i2
        let mut objective = Expression::empty(env.clone());
        let mut term1 = Expression::constant(env.clone(), 2.0);
        term1.mul_assign(&i1).unwrap();
        objective.add_assign(&term1).unwrap();

        let mut term2 = Expression::constant(env.clone(), 3.0);
        term2.mul_assign(&i2).unwrap();
        objective.add_assign(&term2).unwrap();

        // Constraint: i1 + i2 >= 5
        let mut lhs = Expression::empty(env.clone());
        let mut t1 = Expression::constant(env.clone(), 1.0);
        t1.mul_assign(i1).unwrap();
        lhs.add_assign(&t1).unwrap();

        let mut t2 = Expression::constant(env.clone(), 1.0);
        t2.mul_assign(i2).unwrap();
        lhs.add_assign(&t2).unwrap();

        let constraint =
            Constraint::new(lhs, 5.0, Comparator::Ge, Some("MIN_SUM".to_string())).unwrap();

        let mut model1 = Model::with_env(Some("INTEGER".to_string()), Some(Sense::Min), env);
        model1.objective = objective;
        model1.constraints.add_constraint(constraint, None).unwrap();

        // Back-translate to MPS
        let mps_output = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();

        println!("Integer model back-translated:\n{}", mps_output);

        // Translate back to model
        let model2 = MpsTranslator::translate(mps_output).unwrap();

        // Check equality
        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_created_model_mixed_types() {
        // Create model with mixed variable types
        let env = ArcEnv::default();
        env.write_arc().insert("b", Vtype::Binary, None).unwrap();

        let int_bounds = LazyBounds::new(Some(Bound::Bounded(0.0)), Some(Bound::Bounded(5.0)));
        env.write_arc()
            .insert("i", Vtype::Integer, Some(int_bounds))
            .unwrap();

        let real_bounds = LazyBounds::new(Some(Bound::Bounded(0.0)), Some(Bound::Bounded(10.0)));
        env.write_arc()
            .insert("r", Vtype::Real, Some(real_bounds))
            .unwrap();

        let b = env.lookup("b").unwrap();
        let i = env.lookup("i").unwrap();
        let r = env.lookup("r").unwrap();

        // Objective: b + 2*i + 3*r
        let mut objective = Expression::empty(env.clone());
        let mut term1 = Expression::constant(env.clone(), 1.0);
        term1.mul_assign(&b).unwrap();
        objective.add_assign(&term1).unwrap();

        let mut term2 = Expression::constant(env.clone(), 2.0);
        term2.mul_assign(&i).unwrap();
        objective.add_assign(&term2).unwrap();

        let mut term3 = Expression::constant(env.clone(), 3.0);
        term3.mul_assign(&r).unwrap();
        objective.add_assign(&term3).unwrap();

        // Constraint 1: b + i <= 3
        let mut lhs1 = Expression::empty(env.clone());
        let mut t1 = Expression::constant(env.clone(), 1.0);
        t1.mul_assign(b).unwrap();
        lhs1.add_assign(&t1).unwrap();

        let mut t2 = Expression::constant(env.clone(), 1.0);
        t2.mul_assign(&i).unwrap();
        lhs1.add_assign(&t2).unwrap();

        let constraint1 =
            Constraint::new(lhs1, 3.0, Comparator::Le, Some("C1".to_string())).unwrap();

        // Constraint 2: i + r >= 2
        let mut lhs2 = Expression::empty(env.clone());
        let mut t3 = Expression::constant(env.clone(), 1.0);
        t3.mul_assign(i).unwrap();
        lhs2.add_assign(&t3).unwrap();

        let mut t4 = Expression::constant(env.clone(), 1.0);
        t4.mul_assign(r).unwrap();
        lhs2.add_assign(&t4).unwrap();

        let constraint2 =
            Constraint::new(lhs2, 2.0, Comparator::Ge, Some("C2".to_string())).unwrap();

        let mut model1 = Model::with_env(Some("MIXED".to_string()), Some(Sense::Min), env);
        model1.objective = objective;
        model1
            .constraints
            .add_constraint(constraint1, None)
            .unwrap();
        model1
            .constraints
            .add_constraint(constraint2, None)
            .unwrap();

        // Back-translate to MPS
        let mps_output = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();

        println!("Mixed model back-translated:\n{}", mps_output);

        // Translate back to model
        let model2 = MpsTranslator::translate(mps_output).unwrap();

        // Check equality
        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_created_model_multiple_constraints() {
        // Create model with multiple constraints
        let env = ArcEnv::default();
        env.write_arc().insert("x", Vtype::Real, None).unwrap();
        env.write_arc().insert("y", Vtype::Real, None).unwrap();
        env.write_arc().insert("z", Vtype::Real, None).unwrap();

        let x = env.lookup("x").unwrap();
        let y = env.lookup("y").unwrap();
        let z = env.lookup("z").unwrap();

        // Objective: x + 2*y + 3*z
        let mut objective = Expression::empty(env.clone());
        let mut term1 = Expression::constant(env.clone(), 1.0);
        term1.mul_assign(&x).unwrap();
        objective.add_assign(&term1).unwrap();

        let mut term2 = Expression::constant(env.clone(), 2.0);
        term2.mul_assign(&y).unwrap();
        objective.add_assign(&term2).unwrap();

        let mut term3 = Expression::constant(env.clone(), 3.0);
        term3.mul_assign(&z).unwrap();
        objective.add_assign(&term3).unwrap();

        // Constraint 1: x + y <= 10 (Le)
        let mut lhs1 = Expression::empty(env.clone());
        let mut t1 = Expression::constant(env.clone(), 1.0);
        t1.mul_assign(&x).unwrap();
        lhs1.add_assign(&t1).unwrap();
        let mut t2 = Expression::constant(env.clone(), 1.0);
        t2.mul_assign(&y).unwrap();
        lhs1.add_assign(&t2).unwrap();
        let c1 =
            Constraint::new(lhs1, 10.0, Comparator::Le, Some("LE_CONSTR".to_string())).unwrap();

        // Constraint 2: y + z >= 5 (Ge)
        let mut lhs2 = Expression::empty(env.clone());
        let mut t3 = Expression::constant(env.clone(), 1.0);
        t3.mul_assign(&y).unwrap();
        lhs2.add_assign(&t3).unwrap();
        let mut t4 = Expression::constant(env.clone(), 1.0);
        t4.mul_assign(&z).unwrap();
        lhs2.add_assign(&t4).unwrap();
        let c2 = Constraint::new(lhs2, 5.0, Comparator::Ge, Some("GE_CONSTR".to_string())).unwrap();

        // Constraint 3: x + z = 7 (Eq)
        let mut lhs3 = Expression::empty(env.clone());
        let mut t5 = Expression::constant(env.clone(), 1.0);
        t5.mul_assign(x).unwrap();
        lhs3.add_assign(&t5).unwrap();
        let mut t6 = Expression::constant(env.clone(), 1.0);
        t6.mul_assign(z).unwrap();
        lhs3.add_assign(&t6).unwrap();
        let c3 = Constraint::new(lhs3, 7.0, Comparator::Eq, Some("EQ_CONSTR".to_string())).unwrap();

        let mut model1 = Model::with_env(Some("MULTI".to_string()), Some(Sense::Min), env);
        model1.objective = objective;
        model1.constraints.add_constraint(c1, None).unwrap();
        model1.constraints.add_constraint(c2, None).unwrap();
        model1.constraints.add_constraint(c3, None).unwrap();

        // Back-translate to MPS
        let mps_output = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();

        println!("Multi-constraint model back-translated:\n{}", mps_output);

        // Translate back to model
        let model2 = MpsTranslator::translate(mps_output).unwrap();

        // Check equality
        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_quadratic_objective() {
        // Create model with quadratic objective: min x^2 + 2*x*y + y
        let env = ArcEnv::default();
        env.write_arc().insert("x", Vtype::Real, None).unwrap();
        env.write_arc().insert("y", Vtype::Real, None).unwrap();

        let x = env.lookup("x").unwrap();
        let y = env.lookup("y").unwrap();

        // Objective: x^2 + 2*x*y + y
        let mut objective = Expression::empty(env.clone());

        // x^2 term
        let mut x_squared = Expression::constant(env.clone(), 1.0);
        x_squared.mul_assign(&x).unwrap();
        x_squared.mul_assign(&x).unwrap();
        objective.add_assign(&x_squared).unwrap();

        // 2*x*y term
        let mut xy_term = Expression::constant(env.clone(), 2.0);
        xy_term.mul_assign(&x).unwrap();
        xy_term.mul_assign(&y).unwrap();
        objective.add_assign(&xy_term).unwrap();

        // y linear term
        let mut y_term = Expression::constant(env.clone(), 1.0);
        y_term.mul_assign(&y).unwrap();
        objective.add_assign(&y_term).unwrap();

        // Constraint: x + y <= 10
        let mut lhs = Expression::empty(env.clone());
        let mut t1 = Expression::constant(env.clone(), 1.0);
        t1.mul_assign(&x).unwrap();
        lhs.add_assign(&t1).unwrap();
        let mut t2 = Expression::constant(env.clone(), 1.0);
        t2.mul_assign(&y).unwrap();
        lhs.add_assign(&t2).unwrap();

        let constraint =
            Constraint::new(lhs, 10.0, Comparator::Le, Some("C1".to_string())).unwrap();

        let mut model1 = Model::with_env(Some("QUADOBJ".to_string()), Some(Sense::Min), env);
        model1.objective = objective;
        model1.constraints.add_constraint(constraint, None).unwrap();

        // Back-translate to MPS
        let mps_output = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();

        println!("Quadratic objective model back-translated:\n{}", mps_output);

        // Check that QUADOBJ section exists
        assert!(mps_output.contains("QUADOBJ"));

        // Translate back to model
        let model2 = MpsTranslator::translate(mps_output).unwrap();

        // Check equality
        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_quadratic_constraint() {
        // Create model with quadratic constraint: min x + y subject to x^2 + y^2 <= 25
        let env = ArcEnv::default();
        env.write_arc().insert("x", Vtype::Real, None).unwrap();
        env.write_arc().insert("y", Vtype::Real, None).unwrap();

        let x = env.lookup("x").unwrap();
        let y = env.lookup("y").unwrap();

        // Objective: x + y
        let mut objective = Expression::empty(env.clone());
        let mut term1 = Expression::constant(env.clone(), 1.0);
        term1.mul_assign(&x).unwrap();
        objective.add_assign(&term1).unwrap();
        let mut term2 = Expression::constant(env.clone(), 1.0);
        term2.mul_assign(&y).unwrap();
        objective.add_assign(&term2).unwrap();

        // Constraint: x^2 + y^2 <= 25
        let mut lhs = Expression::empty(env.clone());

        // x^2 term
        let mut x_squared = Expression::constant(env.clone(), 1.0);
        x_squared.mul_assign(&x).unwrap();
        x_squared.mul_assign(&x).unwrap();
        lhs.add_assign(&x_squared).unwrap();

        // y^2 term
        let mut y_squared = Expression::constant(env.clone(), 1.0);
        y_squared.mul_assign(&y).unwrap();
        y_squared.mul_assign(&y).unwrap();
        lhs.add_assign(&y_squared).unwrap();

        let constraint =
            Constraint::new(lhs, 25.0, Comparator::Le, Some("CIRCLE".to_string())).unwrap();

        let mut model1 = Model::with_env(Some("QUADCON".to_string()), Some(Sense::Min), env);
        model1.objective = objective;
        model1.constraints.add_constraint(constraint, None).unwrap();

        // Back-translate to MPS
        let mps_output = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();

        println!(
            "Quadratic constraint model back-translated:\n{}",
            mps_output
        );

        // Check that QCMATRIX section exists
        assert!(mps_output.contains("QCMATRIX"));

        // Translate back to model
        let model2 = MpsTranslator::translate(mps_output).unwrap();

        // Check equality
        assert!(model1.equal_contents(&model2));
    }
}
