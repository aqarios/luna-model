//! High-level MPS translation entry points.

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

use super::MpsTranslator;
use super::builder::build_model;
use super::reader::read_mps;

impl MpsTranslator {
    pub fn translate(content: String) -> LunaModelResult<Model> {
        build_model(read_mps(&content)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lunamodel_core::prelude::ContentEquality;
    use lunamodel_types::Sense;

    #[test]
    fn test_simple_mps() {
        let mps_content = r#"
NAME          TESTPROB
ROWS
 N  COST
 L  LIM1
 G  LIM2
 E  MYEQN
COLUMNS
    XONE      COST      1.0       LIM1      1.0
    XONE      LIM2      1.0
    YTWO      COST      4.0       LIM1      1.0
    YTWO      MYEQN    -1.0
    ZTHREE    COST      9.0       LIM2      1.0
    ZTHREE    MYEQN     1.0
RHS
    RHS1      LIM1      5.0
    RHS1      LIM2      10.0
    RHS1      MYEQN     7.0
BOUNDS
 UP BND1      XONE      4.0
 LO BND1      YTWO      -1.0
 UP BND1      YTWO      1.0
ENDATA
"#;

        let model = MpsTranslator::translate(mps_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Min);
        assert_eq!(model.environment.vars().len(), 3);
        assert_eq!(model.constraints.len(), 3);
        assert_eq!(model.objective.linear.len(), 3);
    }

    #[test]
    fn test_mps_with_integers() {
        let mps_content = r#"
NAME          INTTEST
ROWS
 N  OBJ
 L  C1
COLUMNS
    MARK0000  'MARKER'            'INTORG'
    X1        OBJ       2.0       C1        1.0
    X2        OBJ       3.0       C1        1.0
    MARK0001  'MARKER'            'INTEND'
RHS
    RHS1      C1        10.0
BOUNDS
 LO BND1      X1        0
 UP BND1      X1        5
ENDATA
"#;

        let model = MpsTranslator::translate(mps_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Min);
        assert_eq!(model.environment.vars().len(), 2);
        assert_eq!(model.constraints.len(), 1);
    }

    #[test]
    fn test_mps_with_binary() {
        let mps_content = r#"
NAME          BINTEST
ROWS
 N  OBJ
 E  C1
COLUMNS
    X1        OBJ       1.0       C1        1.0
    X2        OBJ       2.0       C1        1.0
RHS
    RHS1      C1        1.0
BOUNDS
 BV BND1      X1
 BV BND1      X2
ENDATA
"#;

        let model = MpsTranslator::translate(mps_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Min);
        assert_eq!(model.environment.vars().len(), 2);
        assert_eq!(model.constraints.len(), 1);
    }

    #[test]
    fn test_back_translate_basic() {
        let mps_content = r#"
NAME          TESTPROB
ROWS
 N  OBJ
 L  C1
COLUMNS
    X1        OBJ       1.0       C1        1.0
    X2        OBJ       2.0       C1        1.0
RHS
    RHS1      C1        5.0
ENDATA
"#;

        let model = MpsTranslator::translate(mps_content.to_string()).unwrap();
        let mps_output = MpsTranslator::back_translate(&model, None)
            .unwrap()
            .unwrap();

        println!("Back-translated MPS:\n{}", mps_output);

        // Check that basic sections are present
        assert!(mps_output.contains("NAME"));
        assert!(mps_output.contains("ROWS"));
        assert!(mps_output.contains("N  OBJ"));
        assert!(mps_output.contains("ENDATA"));
    }

    #[test]
    fn test_roundtrip_simple() {
        let mps_content = r#"
NAME          SIMPLE
ROWS
 N  OBJ
 L  C1
COLUMNS
    X1        OBJ       2.0       C1        1.0
    X2        OBJ       3.0       C1        1.0
RHS
    RHS1      C1        10.0
ENDATA
"#;

        let model1 = MpsTranslator::translate(mps_content.to_string()).unwrap();
        let mps_back = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();
        let model2 = MpsTranslator::translate(mps_back).unwrap();

        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_roundtrip_with_integers() {
        let mps_content = r#"
NAME          INTTEST
ROWS
 N  OBJ
 L  C1
 G  C2
COLUMNS
    MARK0000  'MARKER'            'INTORG'
    X1        OBJ       2.0       C1        1.0
    X2        OBJ       3.0       C1        1.0
    X2        C2        1.0
    MARK0001  'MARKER'            'INTEND'
RHS
    RHS1      C1        10.0
    RHS1      C2        2.0
BOUNDS
 LI BND1      X1        0.0
 UI BND1      X1        5.0
ENDATA
"#;

        let model1 = MpsTranslator::translate(mps_content.to_string()).unwrap();
        let mps_back = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();

        println!("Round-trip integer test:\n{}", mps_back);

        let model2 = MpsTranslator::translate(mps_back).unwrap();

        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_roundtrip_with_binary() {
        let mps_content = r#"
NAME          BINTEST
ROWS
 N  OBJ
 E  C1
COLUMNS
    X1        OBJ       1.0       C1        1.0
    X2        OBJ       2.0       C1        1.0
RHS
    RHS1      C1        1.0
BOUNDS
 BV BND1      X1
 BV BND1      X2
ENDATA
"#;

        let model1 = MpsTranslator::translate(mps_content.to_string()).unwrap();
        let mps_back = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();
        let model2 = MpsTranslator::translate(mps_back).unwrap();

        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_roundtrip_mixed_constraints() {
        let mps_content = r#"
NAME          MIXED
ROWS
 N  OBJ
 L  LIM1
 G  LIM2
 E  EQ1
COLUMNS
    X1        OBJ       1.0       LIM1      1.0
    X1        LIM2      2.0
    X2        OBJ       4.0       LIM1      1.0
    X2        EQ1      -1.0
    X3        OBJ       9.0       LIM2      1.0
    X3        EQ1       1.0
RHS
    RHS1      LIM1      5.0
    RHS1      LIM2      10.0
    RHS1      EQ1       7.0
BOUNDS
 UP BND1      X1        4.0
 LO BND1      X2       -1.0
 UP BND1      X2        1.0
ENDATA
"#;

        let model1 = MpsTranslator::translate(mps_content.to_string()).unwrap();
        let mps_back = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();
        let model2 = MpsTranslator::translate(mps_back).unwrap();

        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_roundtrip_with_free_bounds() {
        let mps_content = r#"
NAME          FREETEST
ROWS
 N  OBJ
 E  C1
COLUMNS
    X1        OBJ       1.0       C1        1.0
    X2        OBJ       2.0       C1        1.0
RHS
    RHS1      C1        5.0
BOUNDS
 FR BND1      X1
 LO BND1      X2       -5.0
 UP BND1      X2        5.0
ENDATA
"#;

        let model1 = MpsTranslator::translate(mps_content.to_string()).unwrap();
        let mps_back = MpsTranslator::back_translate(&model1, None)
            .unwrap()
            .unwrap();
        let model2 = MpsTranslator::translate(mps_back).unwrap();

        assert!(model1.equal_contents(&model2));
    }

    #[test]
    fn test_objsense_max() {
        let mps_content = r#"
NAME          MAXTEST
OBJSENSE
 MAX
ROWS
 N  OBJ
 E  C1
COLUMNS
    X1        OBJ       1.0       C1        1.0
    X2        OBJ       2.0       C1        1.0
RHS
    RHS1      C1        5.0
ENDATA
"#;

        let model = MpsTranslator::translate(mps_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Max);
    }

    #[test]
    fn test_quadobj() {
        let mps_content = r#"
NAME          QUADTEST
ROWS
 N  OBJ
 L  C1
COLUMNS
    X1        OBJ       1.0       C1        1.0
    X2        OBJ       2.0       C1        1.0
RHS
    RHS1      C1        10.0
QUADOBJ
    X1  X1  2.0
    X1  X2  1.0
    X2  X2  4.0
ENDATA
"#;

        let model = MpsTranslator::translate(mps_content.to_string()).unwrap();
        assert_eq!(model.sense, Sense::Min);
        assert_eq!(model.environment.vars().len(), 2);
        assert_eq!(model.constraints.len(), 1);
        // Check that objective has quadratic terms
        assert!(model.objective.quadratic.is_some());
        assert!(!model.objective.quadratic.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_qcmatrix() {
        let mps_content = r#"
NAME          QCTEST
ROWS
 N  OBJ
 L  QC1
COLUMNS
    X1        OBJ       1.0       QC1       1.0
    X2        OBJ       2.0       QC1       1.0
RHS
    RHS1      QC1       10.0
QCMATRIX QC1
    X1  X1  1.0
    X1  X2  0.5
ENDATA
"#;

        let model = MpsTranslator::translate(mps_content.to_string()).unwrap();
        assert_eq!(model.environment.vars().len(), 2);
        assert_eq!(model.constraints.len(), 1);
        // Check that constraint has quadratic terms
        let (_, constraint) = model.constraints.iter().next().unwrap();
        assert!(constraint.lhs.quadratic.is_some());
        assert!(!constraint.lhs.quadratic.as_ref().unwrap().is_empty());
    }
}
