use std::{collections::HashMap, sync::Once};

use indexmap::{IndexMap, indexmap};
use rs_lunamodel::{
    Assignment, Column, Comparator, Constraint, ConstraintCollection, Environment, ErrorKind,
    HigherOrder, LmAddAssign, Result, Sense, Solution, Timing, TryAddAssign, TryClone, TryIndex,
    Variable, Vtype,
};

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| unsafe {
        rs_lunamodel::init(&lunamodel_capi_host::capi_table::LUNA_MODEL_C_API).unwrap();
    });
}

#[test]
fn environment_expression_and_constraint_facade() -> Result<()> {
    init();

    let mut env = rs_lunamodel::ArcEnv::default();
    let x = env.insert("x", Vtype::Binary, None)?;
    let y = env.insert_with_fallback("x", Vtype::Spin, None, Some(&[7, 11]))?;
    assert_ne!(x.name()?, y.name()?);
    assert_eq!(
        env.sort(vec![y.name()?, x.name()?])?,
        vec![x.name()?, y.name()?]
    );

    let mut expr = (&x + &y)?;
    LmAddAssign::add_assign(&mut expr, 2.0)?;
    expr.add_assign((x.id()?, 3.0))?;
    expr.set_offset(4.0)?;
    assert_eq!(expr.linear(x.id()?)?, 4.0);
    assert_eq!(expr.offset()?, 4.0);
    assert_eq!(expr.quadratic_term()?, None);
    assert_eq!(expr.higher_order_term()?, None);

    let constraint = Constraint::new(expr, 3.0, Comparator::Le, Some("limit".into()))?;
    let mut constraints = ConstraintCollection::new();
    assert_eq!(constraints.add_constraint(&constraint, None)?, "limit");
    assert!(constraints.contains("limit")?);
    assert_eq!(constraints.get("limit")?.name()?, "limit");
    {
        let mut editor = constraints.iter_mut()?;
        for (_, constraint) in &mut editor {
            constraint.set_name("renamed".into())?;
        }
        editor.commit()?;
    }
    assert_eq!(constraints.get("limit")?.name()?, "renamed");
    Ok(())
}

#[test]
fn owned_environment_and_mutation_proxy() -> Result<()> {
    init();

    let variable = Variable::new("x", Vtype::Binary, None)?;
    let env = Environment::new(
        IndexMap::from([(4, variable)]),
        HashMap::from([("x".to_owned(), 4)]),
        9,
    )?;
    assert_eq!(env.lookup("x")?, 4);
    assert_eq!(env.next_idx()?, 9);
    assert_eq!(env.get(4)?.name()?, "x");

    let mut env = Environment::default();
    let x = env.insert("x", Vtype::Binary, None)?;
    let arc = env.as_arc().try_clone()?;
    let x_ref = arc.get(x)?;
    let inverted = env.insert_inverted(&x_ref)?;
    {
        let mut variable = env.get_mut(x)?;
        variable.inverted = Some(inverted);
        variable.commit()?;
    }
    assert_eq!(env.get(x)?.inverted()?, Some(inverted));
    Ok(())
}

#[test]
fn term_editors_commit_to_host() -> Result<()> {
    init();

    let mut linear = rs_lunamodel::Linear::for_var(1, 2.0)?;
    {
        let mut editor = linear.iter_mut()?;
        for (_, bias) in &mut editor {
            *bias *= 3.0;
        }
        editor.commit()?;
    }
    assert_eq!(linear.try_index(1)?, 6.0);

    let mut quadratic = rs_lunamodel::Quadratic::default();
    quadratic.try_add_assign((1, 2, 4.0))?;
    {
        let mut editor = quadratic.iter_mut()?;
        for (_, neighborhood) in &mut editor {
            for (_, bias) in neighborhood.iter_mut() {
                *bias += 1.0;
            }
        }
        editor.commit()?;
    }
    assert_eq!(quadratic.try_index((1, 2))?, 5.0);

    let mut higher = HigherOrder::default();
    higher.try_add_assign((vec![3, 2, 1], 7.0))?;
    {
        let mut editor = higher.iter_mut()?;
        for (_, bias) in &mut editor {
            *bias = 8.0;
        }
        editor.commit()?;
    }
    assert_eq!(higher.try_index(&[1, 2, 3][..])?, 8.0);
    Ok(())
}

#[test]
fn solution_optional_values_buffers_and_owned_views() -> Result<()> {
    init();

    let mut solution = Solution::with_sense(Sense::Min);
    solution.push(
        indexmap! { "x".into() => rs_lunamodel::Assignment::Binary(1) },
        2,
        None,
    )?;
    assert_eq!(solution.raw_energies()?, None);
    assert_eq!(solution.obj_values()?, None);
    assert_eq!(solution.feasible()?, None);
    assert_eq!(solution.timing()?, None);
    assert_eq!(solution.try_index((0, "x"))?, 1.0);

    let result = solution.result(0)?.unwrap();
    let sample = result.sample()?;
    solution.add_real("y", vec![3.0])?;
    assert_eq!(sample.variable_names()?, vec!["x", "y"]);
    assert_eq!(result.sample()?.try_index("y")?, 3.0);
    drop(solution);
    assert_eq!(result.counts()?, 2);
    assert_eq!(sample.try_index("x")?, 1.0);

    let mut timing = Timing::new(20.0)?;
    assert_eq!(timing.total_for("qpu")?, None);
    assert_eq!(timing.total()?, 20.0);
    timing.set("qpu", 5.0)?;
    assert_eq!(timing.total_for("qpu")?, Some(5.0));
    let mut metadata = Solution::with_sense(Sense::Min);
    metadata.set_counts(vec![7])?;
    metadata.set_timing(Some(&timing))?;
    assert_eq!(metadata.counts()?, vec![7]);
    assert_eq!(metadata.timing()?.unwrap(), timing);
    metadata.set_timing(None)?;
    assert_eq!(metadata.timing()?, None);
    Ok(())
}

#[test]
fn model_children_are_live_parent_retaining_views() -> Result<()> {
    init();

    let mut model = rs_lunamodel::Model::default();
    assert_eq!(model.environment()?.id()?, model.env()?.id()?);
    let mut objective = model.objective()?;
    objective.try_add_assign(2.0)?;
    assert_eq!(model.objective()?.offset()?, 2.0);

    model.set_objective(
        rs_lunamodel::Expression::constant(model.env()?, 7.0)?,
        Some(Sense::Max),
    )?;
    assert_eq!(objective.offset()?, 7.0);
    assert_eq!(model.sense()?, Sense::Max);

    let mut constraints = model.constraints()?;
    let constraint = Constraint::new(
        rs_lunamodel::Expression::constant(model.env()?, 0.0)?,
        1.0,
        Comparator::Le,
        Some("first".into()),
    )?;
    constraints.add_constraint(&constraint, None)?;
    assert_eq!(model.constraints()?.len()?, 1);

    model.set_constraints(ConstraintCollection::new())?;
    assert!(constraints.is_empty()?);

    drop(model);
    assert_eq!(objective.offset()?, 7.0);
    assert!(constraints.is_empty()?);

    let clone = rs_lunamodel::Model::default().deep_clone()?;
    assert_eq!(clone.name()?, rs_lunamodel::DEFAULT_MODEL_NAME);
    Ok(())
}

#[test]
fn model_views_enforce_model_environment() -> Result<()> {
    init();

    let mut model = rs_lunamodel::Model::default();
    let x = model.add_var("x", Vtype::Binary, None)?;
    let mut other = rs_lunamodel::Model::default();
    let a = other.add_var("a", Vtype::Binary, None)?;

    // Objective view: raw indices must name living model variables, and
    // arithmetic with another environment is rejected.
    let mut objective = model.objective()?;
    objective.try_add_assign((x.id()?, 1.0))?;
    let err = objective.try_add_assign((99, 1.0)).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::VariableNotExisting);
    let err = objective.try_add_assign(&(&a * 2.0)?).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DifferentEnvironments);
    assert_eq!(model.objective()?.raw_linear_items()?, vec![(x.id()?, 1.0)]);

    // Constraints view: constraints from another environment never reach the model.
    let mut constraints = model.constraints()?;
    let foreign = Constraint::new((&a * 1.0)?, 1.0, Comparator::Le, Some("foreign".into()))?;
    let err = constraints.add_constraint(&foreign, None).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DifferentEnvironments);
    let mut foreign_col = ConstraintCollection::new();
    foreign_col.add_constraint(&foreign, None)?;
    let err = constraints.add_collection(&foreign_col, None).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DifferentEnvironments);
    assert!(model.constraints()?.is_empty()?);

    let own = Constraint::new((&x * 1.0)?, 1.0, Comparator::Le, Some("own".into()))?;
    constraints.add_constraint(&own, None)?;
    let err = constraints.set_constraint("own", &foreign).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DifferentEnvironments);
    assert_eq!(model.constraints()?.len()?, 1);
    Ok(())
}

#[test]
fn column_facade_dispatches_to_host() -> Result<()> {
    init();

    let mut column = Column::binary(vec![0.0, 1.0])?;
    column.try_push(1.0, None)?;
    column.push(Assignment::Binary(0))?;
    assert_eq!(column.values()?, vec![0.0, 1.0, 1.0, 0.0]);
    assert_eq!(column.as_assignments()?.len(), 4);
    assert_eq!(column.vtype()?, Vtype::Binary);

    let filtered = column.filter_by_mask(&[false, true, true, false])?;
    assert_eq!(filtered.values()?, vec![1.0, 1.0]);
    let cloned = column.try_clone()?;
    assert_eq!(cloned, column);
    assert_eq!(column.extract(2)?.assignment(0)?, Assignment::Binary(1));
    Ok(())
}

#[test]
fn expression_and_model_evaluation_via_capi() -> Result<()> {
    init();

    let mut env = rs_lunamodel::ArcEnv::default();
    let x = env.insert("x", Vtype::Binary, None)?;
    let y = env.insert("y", Vtype::Binary, None)?;

    // Objective x + y. `evaluate_sample` dispatches to the C ABI (no host-side
    // summation); the sample is a name->value map.
    let obj = (&x + &y)?;
    let sample = HashMap::from([("x".to_owned(), 1.0), ("y".to_owned(), 1.0)]);
    assert_eq!(obj.evaluate_sample(&sample)?, 2.0);
    assert!(obj.contains(&x)?);

    // A one-row solution; `evaluate` runs the expression over every row.
    let mut sol = Solution::with_sense(Sense::Min);
    sol.push(
        indexmap! {
            "x".into() => rs_lunamodel::Assignment::Binary(1),
            "y".into() => rs_lunamodel::Assignment::Binary(1),
        },
        1,
        None,
    )?;
    assert_eq!(obj.evaluate(&sol)?, vec![2.0]);

    // Model with the objective and a constraint x + y <= 1.
    let mut model = rs_lunamodel::Model::with_env(Some("m".into()), None, &env)?;
    model.set_objective(obj.try_clone()?, None)?;
    let cap = Constraint::new((&x + &y)?, 1.0, Comparator::Le, Some("cap".into()))?;
    let mut cc = ConstraintCollection::new();
    cc.add_constraint(&cap, None)?;
    assert!(!cap.evaluate_sample(&sample, None)?);
    assert!(!cc.evaluate_sample(&sample, None)?["cap"]);
    model.set_constraints(cc)?;

    // evaluate_sample -> ResultView; violated_constraints -> collection.
    let sample_view = sol.sample(0)?.unwrap();
    let result = model.evaluate_sample(&sample_view, None)?;
    assert_eq!(result.obj_value()?, Some(2.0));
    let violated = model.violated_constraints(&sample_view, None)?;
    assert!(violated.contains("cap")?); // x + y = 2 > 1
    Ok(())
}
