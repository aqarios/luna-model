//! Both writers: a written model must read back unchanged, in either format.

mod common;

use common::contents;
use lunamodel_core::Model;
use lunamodel_translate::model::{LpTranslator, MpsTranslator};
use lunamodel_types::Vtype;

/// Builds the model to write from LP text, the shortest way to describe one.
fn model(lp: &str) -> Model {
    LpTranslator::translate(lp.to_string()).unwrap()
}

fn assert_roundtrips(original: &Model) {
    let lp = LpTranslator::back_translate(original, None)
        .unwrap()
        .unwrap();
    let from_lp = LpTranslator::translate(lp.clone()).unwrap();
    assert_eq!(contents(&from_lp), contents(original), "LP:\n{lp}");

    let mps = MpsTranslator::back_translate(original, None)
        .unwrap()
        .unwrap();
    let from_mps = MpsTranslator::translate(mps.clone()).unwrap();
    assert_eq!(contents(&from_mps), contents(original), "MPS:\n{mps}");
}

#[test]
fn every_variable_kind_and_bound() {
    assert_roundtrips(&model(
        "Maximize\n obj: 2 x + 3 y - z + [ x * y + 2 z ^ 2 ] / 2 + 5\n\
         Subject To\n c1: x + y + z <= 10\n c2: x - y >= -2\n q: [ x * y ] + z <= 4\n eq: y + z = 3\n\
         Bounds\n x free\n -5 <= y <= 5\n z <= -1\n 0 <= n <= 1\n\
         Generals\n n i\nBinaries\n b\nEnd\n",
    ));
}

#[test]
fn variables_without_coefficients_keep_their_place() {
    assert_roundtrips(&model(
        "Minimize\n z\nSubject To\n c: a >= 1\nBounds\n unused <= 3\nGenerals\n gen\nBinaries\n bin\nEnd\n",
    ));
}

#[test]
fn constraint_named_like_the_objective_row() {
    assert_roundtrips(&model("Minimize\n x\nSubject To\n OBJ: x + y >= 1\nEnd\n"));
}

#[test]
fn names_with_special_characters() {
    assert_roundtrips(&model(
        "Minimize\n x.1 + 2 y(0) + 3 x[0,0]\nSubject To\n c: x.1 + y(0) >= 1\nEnd\n",
    ));
}

#[test]
fn long_lp_lines_are_wrapped() {
    let sum: Vec<_> = (0..60).map(|i| format!("x{i}")).collect();
    let original = model(&format!(
        "Minimize\n {}\nSubject To\n c: {} <= 1\nEnd\n",
        sum.join(" + "),
        sum.join(" + ")
    ));
    let lp = LpTranslator::back_translate(&original, None)
        .unwrap()
        .unwrap();
    let longest = lp.lines().map(str::len).max().unwrap();
    assert!(
        longest <= 100,
        "longest line has {longest} characters:\n{lp}"
    );
    assert_roundtrips(&original);
}

/// A model with one constraint `x >= 1` named `name`.
fn with_constraint_name(name: &str) -> Model {
    let mut m = model("Minimize\n x\nSubject To\n c: x >= 1\nEnd\n");
    let c = m.constraints().get("c").unwrap().clone();
    m.set_constraints(Default::default()).unwrap();
    m.add_constraint(c, Some(name.into())).unwrap();
    m
}

fn lp_error(m: &Model) -> String {
    LpTranslator::back_translate(m, None)
        .unwrap_err()
        .to_string()
}

#[test]
fn constraint_names_lp_cannot_read_are_rejected() {
    for name in ["a b", "c:1", "c>=", "st"] {
        let err = lp_error(&with_constraint_name(name));
        assert!(err.contains(&format!("constraint name '{name}'")), "{err}");
    }
}

#[test]
fn variable_names_lp_cannot_read_are_rejected() {
    for name in ["end", "st", "free", "bin", "inf", "s.t.", "x]"] {
        let mut m = model("Minimize\n x\nEnd\n");
        m.add_var(name, Vtype::Binary, None).unwrap();
        let err = lp_error(&m);
        assert!(err.contains(&format!("variable name '{name}'")), "{err}");
    }
}

#[test]
fn names_with_whitespace_are_rejected_by_mps() {
    let err = MpsTranslator::back_translate(&with_constraint_name("a b"), None)
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("constraint name 'a b': it contains whitespace"),
        "{err}"
    );
}
