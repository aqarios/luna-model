//! LP reader behaviour, checked only through the public `LpTranslator` API.

use lunamodel_core::Model;
use lunamodel_translate::model::LpTranslator;
use lunamodel_types::{Bound, Comparator};

fn read(lp: &str) -> Model {
    LpTranslator::translate(lp.to_string()).unwrap()
}

fn read_err(lp: &str) -> String {
    LpTranslator::translate(lp.to_string())
        .unwrap_err()
        .to_string()
}

fn bounds(model: &Model, name: &str) -> (Bound, Bound) {
    let b = model.environment.lookup(name).unwrap().bounds().unwrap();
    (b.lower, b.upper)
}

/// Objective terms as `("x", c)` or `("x*y", c)`, sorted by name.
fn objective(model: &Model) -> Vec<(String, f64)> {
    let mut terms: Vec<_> = model
        .objective()
        .items()
        .map(|(vars, c)| {
            let names: Vec<_> = vars.iter().map(|v| v.name().unwrap()).collect();
            (names.join("*"), c)
        })
        .collect();
    terms.sort_by(|a, b| a.0.cmp(&b.0));
    terms
}

fn var_names(model: &Model) -> Vec<String> {
    model
        .environment
        .vars()
        .iter()
        .map(|v| v.name().unwrap())
        .collect()
}

#[test]
fn free_bound_is_unbounded_on_both_sides() {
    let m = read("Minimize\n x\nSubject To\n c: x >= -5\nBounds\n x free\nEnd\n");
    assert_eq!(bounds(&m, "x"), (Bound::Unbounded, Bound::Unbounded));
}

#[test]
fn minus_infinity_lower_bound_is_unbounded() {
    let m = read(
        "Minimize\n x + y\nSubject To\n c: x + y >= 1\nBounds\n -inf <= x <= 4\n y >= -infinity\nEnd\n",
    );
    assert_eq!(bounds(&m, "x"), (Bound::Unbounded, Bound::Bounded(4.0)));
    assert_eq!(bounds(&m, "y"), (Bound::Unbounded, Bound::Unbounded));
}

#[test]
fn number_formats() {
    let m = read(
        "Minimize\n 2.5e-3 x + 1E+2 y + .5 z + 3. w\nSubject To\n c: x + y + z + w >= 1\nEnd\n",
    );
    let expected = [("w", 3.0), ("x", 2.5e-3), ("y", 100.0), ("z", 0.5)];
    assert_eq!(objective(&m), expected.map(|(n, c)| (n.to_string(), c)));
}

#[test]
fn malformed_number_is_an_error_naming_the_line() {
    let err = read_err("Minimize\n x\n + 1.2.3 y\nSubject To\n c: x >= 1\nEnd\n");
    assert!(err.contains("line 3"), "{err}");
}

#[test]
fn names_with_special_characters() {
    let m = read("Minimize\n x.1 + 2 y(0) + 3 x[0,0] + 4 x0,0\nSubject To\n c: x.1 >= 1\nEnd\n");
    assert_eq!(var_names(&m), ["x.1", "y(0)", "x[0,0]", "x0,0"]);
    assert_eq!(objective(&m).len(), 4);
}

#[test]
fn bracketed_name_inside_quadratic_term() {
    let m = read("Minimize\n [x[0] * y] / 2\nSubject To\n c: x[0] + y >= 1\nEnd\n");
    assert_eq!(objective(&m), [("x[0]*y".to_string(), 0.5)]);
}

#[test]
fn own_output_reads_back_unchanged() {
    let lp = "Minimize\n x.1 + 2 y(0) + 3 x0,0\nSubject To\n c: x.1 + y(0) >= 1\nEnd\n";
    let m = read(lp);
    let written = LpTranslator::back_translate(&m, None).unwrap().unwrap();
    let again = read(&written);
    assert_eq!(var_names(&again), var_names(&m));
    assert_eq!(objective(&again), objective(&m));
}

#[test]
fn variables_keep_file_order() {
    let m = read("Minimize\n z + x10 + x2 + a\nSubject To\n c: z >= 1\nEnd\n");
    assert_eq!(var_names(&m), ["z", "x10", "x2", "a"]);
}

#[test]
fn problem_name_comment_without_colon() {
    let m = read("\\ Problem name foo\nMinimize\n x\nSubject To\n c: x >= 1\nEnd\n");
    assert_eq!(var_names(&m), ["x"]);
}

#[test]
fn constraint_name_on_its_own_line() {
    let m = read("Minimize\n x\nSubject To\n c1:\n x >= 1\nEnd\n");
    assert_eq!(m.constraints().get("c1").unwrap().rhs, 1.0);
}

#[test]
fn alternative_comparison_operators() {
    let m = read("Minimize\n x\nSubject To\n c1: x =< 4\n c2: x => 1\nEnd\n");
    assert_eq!(
        m.constraints().get("c1").unwrap().comparator,
        Comparator::Le
    );
    assert_eq!(
        m.constraints().get("c2").unwrap().comparator,
        Comparator::Ge
    );
}

#[test]
fn number_first_bounds() {
    let m = read(
        "Minimize\n x + y + z\nSubject To\n c: x + y + z >= 1\nBounds\n 3 <= x\n 5 >= y\n 10 >= z >= 2\nEnd\n",
    );
    assert_eq!(bounds(&m, "x"), (Bound::Bounded(3.0), Bound::Unbounded));
    assert_eq!(bounds(&m, "y"), (Bound::Bounded(0.0), Bound::Bounded(5.0)));
    assert_eq!(bounds(&m, "z"), (Bound::Bounded(2.0), Bound::Bounded(10.0)));
}

#[test]
fn variable_name_containing_free_keeps_its_bound() {
    let m = read("Minimize\n x_free\nSubject To\n c: x_free >= 1\nBounds\n x_free <= 4\nEnd\n");
    assert_eq!(
        bounds(&m, "x_free"),
        (Bound::Bounded(0.0), Bound::Bounded(4.0))
    );
}

#[test]
fn invalid_bound_value_is_an_error() {
    read_err("Minimize\n x\nSubject To\n c: x >= 1\nBounds\n x <= 1e\nEnd\n");
}

#[test]
fn quadratic_divided_by_two_point_zero() {
    let m = read("Minimize\n [ x^2 ] / 2.0\nSubject To\n c: x >= 1\nEnd\n");
    assert_eq!(objective(&m), [("x*x".to_string(), 0.5)]);
}

#[test]
fn unsupported_section_is_rejected_by_name() {
    let err = read_err(
        "Minimize\n x + y\nSubject To\n c: x + y >= 1\nGenerals\n x\nSemi-continuous\n y\nEnd\n",
    );
    assert!(err.contains("Unsupported section"), "{err}");
}
