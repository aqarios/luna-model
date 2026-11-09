//! MPS reader behaviour, checked only through the public `MpsTranslator` API.

mod common;

use common::{bounds, constraints, expected, objective, terms, var_names, vtype};
use lunamodel_core::Model;
use lunamodel_translate::model::MpsTranslator;
use lunamodel_types::{Bound, Comparator, Sense, Vtype};

fn read(mps: &str) -> Model {
    MpsTranslator::translate(mps.to_string()).unwrap()
}

fn read_err(mps: &str) -> String {
    MpsTranslator::translate(mps.to_string())
        .unwrap_err()
        .to_string()
}

/// A fixed-format data line: the fields start at columns 2, 5, 15, 25, 40 and 50.
fn fixed(fields: &[&str]) -> String {
    let mut line = String::new();
    for (field, start) in fields.iter().zip([1, 4, 14, 24, 39, 49]) {
        while line.len() < start {
            line.push(' ');
        }
        line.push_str(field);
    }
    line + "\n"
}

/// A model with one objective row `obj` and one row `c1: x + y <= 4`, plus `rest`.
fn small(rest: &str) -> String {
    format!(
        "NAME test\nROWS\n N obj\n L c1\nCOLUMNS\n x obj 1 c1 1\n y obj 2 c1 1\nRHS\n rhs c1 4\n{rest}ENDATA\n"
    )
}

#[test]
fn rows_columns_and_rhs() {
    let m = read(
        "NAME test\nROWS\n N obj\n L le\n G ge\n E eq\nCOLUMNS\n x obj 1 le 2\n x ge 3\n y obj -4 eq 5\nRHS\n rhs le 6 ge 7\n rhs eq 8\nENDATA\n",
    );
    assert_eq!(m.name, "test");
    assert_eq!(m.sense, Sense::Min);
    assert_eq!(objective(&m), expected(&[("x", 1.0), ("y", -4.0)]));
    assert_eq!(
        constraints(&m),
        [
            ("le".to_string(), Comparator::Le, 6.0),
            ("ge".to_string(), Comparator::Ge, 7.0),
            ("eq".to_string(), Comparator::Eq, 8.0),
        ]
    );
    assert_eq!(
        terms(&m.constraints().get("le").unwrap().lhs),
        expected(&[("x", 2.0)])
    );
}

#[test]
fn rows_and_columns_keep_file_order() {
    let m = read(
        "NAME t\nROWS\n N obj\n L z_row\n L a_row\nCOLUMNS\n z obj 1 z_row 1\n x10 a_row 1\n x2 a_row 1\nENDATA\n",
    );
    assert_eq!(var_names(&m), ["z", "x10", "x2"]);
    let names: Vec<_> = constraints(&m).into_iter().map(|c| c.0).collect();
    assert_eq!(names, ["z_row", "a_row"]);
}

#[test]
fn fixed_format_row_name_with_spaces() {
    let mps = [
        "NAME          SPACES\nROWS\n".to_string(),
        fixed(&["N", "COST"]),
        fixed(&["L", "MY ROW"]),
        "COLUMNS\n".to_string(),
        fixed(&["", "x", "COST", "1", "MY ROW", "2"]),
        "RHS\n".to_string(),
        fixed(&["", "RHS", "MY ROW", "4"]),
        "ENDATA\n".to_string(),
    ]
    .concat();
    let m = read(&mps);
    assert_eq!(
        constraints(&m),
        [("MY ROW".to_string(), Comparator::Le, 4.0)]
    );
    assert_eq!(
        terms(&m.constraints().get("MY ROW").unwrap().lhs),
        expected(&[("x", 2.0)])
    );
}

#[test]
fn fixed_format_error_is_reported_where_it_is() {
    // Free format already fails on line 4 ("MY ROW"); the actual mistake is the typo on line 8.
    let mps = [
        "NAME          SPACES\nROWS\n".to_string(),
        fixed(&["N", "COST"]),
        fixed(&["L", "MY ROW"]),
        "COLUMNS\n".to_string(),
        fixed(&["", "x", "COST", "1", "MY ROW", "2"]),
        "RHS\n".to_string(),
        fixed(&["", "RHS", "MY TYPO", "4"]),
        "ENDATA\n".to_string(),
    ]
    .concat();
    let err = read_err(&mps);
    assert!(err.contains("line 8") && err.contains("MY TYPO"), "{err}");
}

#[test]
fn missing_rhs_and_bound_set_names() {
    let m = read(&small("BOUNDS\n UP x 3\n FR y\n").replace(" rhs c1 4", " c1 4"));
    assert_eq!(constraints(&m), [("c1".to_string(), Comparator::Le, 4.0)]);
    assert_eq!(bounds(&m, "x"), (Bound::Bounded(0.0), Bound::Bounded(3.0)));
    assert_eq!(bounds(&m, "y"), (Bound::Unbounded, Bound::Unbounded));
}

#[test]
fn column_named_like_a_section() {
    let m = read(&small("").replace(" y obj 2 c1 1", " NAMES obj 2 c1 1"));
    assert_eq!(var_names(&m), ["x", "NAMES"]);
}

#[test]
fn objsense_on_the_same_or_next_line() {
    let same_line = read(&small("").replace("NAME test\n", "NAME test\nOBJSENSE max\n"));
    let next_line = read(&small("").replace("NAME test\n", "NAME test\nOBJSENSE\n    MAXIMIZE\n"));
    assert_eq!(same_line.sense, Sense::Max);
    assert_eq!(next_line.sense, Sense::Max);
}

#[test]
fn first_n_row_is_the_objective_and_other_n_rows_are_dropped() {
    let m = read(
        "NAME t\nROWS\n N obj\n N free\n L a\n L b\nCOLUMNS\n x obj 1 free 7\n x a 1 b 1\nRHS\n rhs obj -3 free -9\nENDATA\n",
    );
    assert_eq!(objective(&m), expected(&[("", 3.0), ("x", 1.0)]));
    let names: Vec<_> = constraints(&m).into_iter().map(|c| c.0).collect();
    assert_eq!(names, ["a", "b"]);
}

#[test]
fn objective_constant_is_the_negated_rhs() {
    let m = read(&small(" rhs obj -2.5\n"));
    assert_eq!(m.objective().offset, 2.5);
}

#[test]
fn unknown_row_is_an_error() {
    read_err(&small("").replace(" y obj 2 c1 1", " y obj 2 typo 1"));
    read_err(&small(" rhs typo 1\n"));
}

#[test]
fn bound_types() {
    let m = read(
        "NAME t\nROWS\n N obj\nCOLUMNS\n lo obj 1\n up obj 1\n fx obj 1\n fr obj 1\n mi obj 1\n pl obj 1\n bv obj 1\n li obj 1\n ui obj 1\n\
         BOUNDS\n LO b lo -1\n UP b up 5\n FX b fx 2\n FR b fr\n MI b mi\n PL b pl\n BV b bv\n LI b li 3\n UI b ui 7\nENDATA\n",
    );
    use Bound::{Bounded, Unbounded};
    assert_eq!(bounds(&m, "lo"), (Bounded(-1.0), Unbounded));
    assert_eq!(bounds(&m, "up"), (Bounded(0.0), Bounded(5.0)));
    assert_eq!(bounds(&m, "fx"), (Bounded(2.0), Bounded(2.0)));
    assert_eq!(bounds(&m, "fr"), (Unbounded, Unbounded));
    assert_eq!(bounds(&m, "mi"), (Unbounded, Unbounded));
    assert_eq!(bounds(&m, "pl"), (Bounded(0.0), Unbounded));
    assert_eq!(vtype(&m, "bv"), Vtype::Binary);
    assert_eq!(
        (vtype(&m, "li"), bounds(&m, "li")),
        (Vtype::Integer, (Bounded(3.0), Unbounded))
    );
    assert_eq!(
        (vtype(&m, "ui"), bounds(&m, "ui")),
        (Vtype::Integer, (Bounded(0.0), Bounded(7.0)))
    );
}

#[test]
fn negative_upper_bound_keeps_the_lower_bound() {
    let m = read(&small("BOUNDS\n UP b x -2\n"));
    assert_eq!(bounds(&m, "x"), (Bound::Bounded(0.0), Bound::Bounded(-2.0)));
}

#[test]
fn integer_markers() {
    let mps = "NAME t\nROWS\n N obj\nCOLUMNS\n m 'MARKER' 'INTORG'\n plain obj 1\n bounded obj 1\n m 'MARKER' 'INTEND'\n real obj 1\nBOUNDS\n LO b bounded 2\nENDATA\n";
    let m = read(mps);
    // Without bounds an integer column defaults to 0 <= x <= 1; any bound entry removes that default.
    assert_eq!(vtype(&m, "plain"), Vtype::Binary);
    assert_eq!(vtype(&m, "bounded"), Vtype::Integer);
    assert_eq!(
        bounds(&m, "bounded"),
        (Bound::Bounded(2.0), Bound::Unbounded)
    );
    assert_eq!(vtype(&m, "real"), Vtype::Real);
}

#[test]
fn ranges_become_two_constraints() {
    let m = read(
        "NAME t\nROWS\n N obj\n E e_pos\n E e_neg\n L l\n G g\nCOLUMNS\n x obj 1 e_pos 1\n x e_neg 1 l 1\n x g 1\n\
         RHS\n rhs e_pos 10 e_neg 10\n rhs l 10 g 10\nRANGES\n rng e_pos 4 e_neg -4\n rng l -4 g 4\nENDATA\n",
    );
    let c = |name: &str, cmp, rhs| (name.to_string(), cmp, rhs);
    use Comparator::{Ge, Le};
    assert_eq!(
        constraints(&m),
        [
            c("e_pos_lb", Ge, 10.0),
            c("e_pos_ub", Le, 14.0),
            c("e_neg_lb", Ge, 6.0),
            c("e_neg_ub", Le, 10.0),
            c("l_lb", Ge, 6.0),
            c("l_ub", Le, 10.0),
            c("g_lb", Ge, 10.0),
            c("g_ub", Le, 14.0),
        ]
    );
}

#[test]
fn quadobj_lists_one_triangle_of_half_xqx() {
    let m = read(&small("QUADOBJ\n x x 4\n x y 3\n"));
    let quadratic: Vec<_> = objective(&m)
        .into_iter()
        .filter(|t| t.0.contains('*'))
        .collect();
    assert_eq!(quadratic, expected(&[("x*x", 2.0), ("x*y", 3.0)]));
}

#[test]
fn qmatrix_lists_the_full_matrix_of_half_xqx() {
    let m = read(&small("QMATRIX\n x x 4\n x y 3\n y x 3\n"));
    let quadratic: Vec<_> = objective(&m)
        .into_iter()
        .filter(|t| t.0.contains('*'))
        .collect();
    assert_eq!(quadratic, expected(&[("x*x", 2.0), ("x*y", 3.0)]));
}

#[test]
fn qsection_for_the_objective_is_like_quadobj() {
    let m = read(&small("QSECTION obj\n x x 4\n y x 3\n"));
    let quadratic: Vec<_> = objective(&m)
        .into_iter()
        .filter(|t| t.0.contains('*'))
        .collect();
    assert_eq!(quadratic, expected(&[("x*x", 2.0), ("x*y", 3.0)]));
}

#[test]
fn qcmatrix_lists_the_full_matrix_of_xqx() {
    let m = read(&small("QCMATRIX c1\n x x 4\n x y 3\n y x 3\n"));
    let lhs = terms(&m.constraints().get("c1").unwrap().lhs);
    assert_eq!(
        lhs,
        expected(&[("x", 1.0), ("x*x", 4.0), ("x*y", 6.0), ("y", 1.0)])
    );
}

#[test]
fn qsection_for_a_row_lists_one_triangle_of_half_xqx() {
    let m = read(&small("QSECTION c1\n x x 4\n y x 3\n"));
    let lhs = terms(&m.constraints().get("c1").unwrap().lhs);
    assert_eq!(
        lhs,
        expected(&[("x", 1.0), ("x*x", 2.0), ("x*y", 3.0), ("y", 1.0)])
    );
}

#[test]
fn unsupported_sections_are_rejected_by_name() {
    for rest in [
        "SOS\n S1 SOS\n x 1\n",
        "INDICATORS\n IF c1 x 1\n",
        "BOUNDS\n SC b x 5\n",
    ] {
        let err = read_err(&small(rest));
        assert!(err.contains("Unsupported"), "{err}");
    }
}

#[test]
fn malformed_number_is_an_error_naming_the_line() {
    let err = read_err(&small("").replace(" y obj 2 c1 1", " y obj 2 c1 1.2.3"));
    assert!(err.contains("line 7"), "{err}");
}
