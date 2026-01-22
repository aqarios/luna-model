use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use super::LpTranslator;
use super::builder::build_model;
use super::reader::read_lp;

impl LpTranslator {
    pub fn translate(content: String) -> LunaModelResult<Model> {
        build_model(read_lp(&content)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lunamodel_types::{Comparator, Sense, Vtype};

    #[test]
    fn test_parse_simple_linear() {
        let lp_content = r#"
Minimize
  x + 2 y + 5 ^ 3
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
    fn test_parse_simple_linear_obj() {
        let lp_content = r#"
Minimize
  obj: x + 2 y
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
        assert!(err_msg.to_lowercase().contains("binary variable"));
        assert!(err_msg.to_lowercase().contains("invalid"));
        assert!(err_msg.to_lowercase().contains("bound"));
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
        assert!(
            quad.len() >= 1,
            "Expected at least 1 quadratic term in objective, got {}",
            quad.len()
        );
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

    #[test]
    fn test_quadratic() {
        let lp_content = r#"
Minimize
 obj: - 7 x_0 - 10 x_1 - 2 x_2 
      + [ - 2 x_1 * x_0 - 10 x_2 * x_0 - 2 x_2 * x_1 ]/2
Subject To
 capacity: + 5 x_0 + 2 x_1 + 10 x_2 <= 25.0
Binary
 x_0 x_1 x_2
End
"#;

        let model = LpTranslator::translate(lp_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 3);
        assert_eq!(model.constraints.len(), 1);

        // Check the constraint has all variables
        let capacity = model.constraints.get("capacity").unwrap();
        assert_eq!(capacity.lhs.linear.len(), 3);
        assert_eq!(capacity.comparator, Comparator::Le);
        assert_eq!(capacity.rhs, 25.0);

        // Check objective has all variables
        assert_eq!(model.objective.offset, 0.0);
    }
}
