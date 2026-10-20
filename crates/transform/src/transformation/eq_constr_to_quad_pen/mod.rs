//! Equality-constraint penalty conversion pass and artifact types.
mod artifact;
mod pass;

pub use artifact::EqualityConstraintsToQuadraticPenaltyArtifact;
pub use pass::EqualityConstraintsToQuadraticPenaltyPass;

#[cfg(test)]
mod tests {
    use lunamodel_core::{
        Model, Solution,
        ops::LmAddAssign,
        prelude::{Constraint, ContentEquality},
    };
    use lunamodel_error::LunaModelResult;
    use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
    use lunamodel_transpiler::{PassManager, TransformationRecord, register_backward};
    use lunamodel_types::{Comparator, Sense, Vtype};

    use crate::{
        analysis::MaxBiasAnalysis, transformation::EqualityConstraintsToQuadraticPenaltyPass,
    };

    #[test]
    fn roundtrip_equality_constraints_to_quadratic_penalty_pass() -> LunaModelResult<()> {
        register_backward::<EqualityConstraintsToQuadraticPenaltyPass>();

        let mut model = Model::default();
        let x = model.add_var("i0", Vtype::Integer, None)?;
        let y = model.add_var("i1", Vtype::Integer, None)?;
        model.objective_mut().add_assign((&x * &y)?)?;
        model.add_constraint(
            Constraint::new((&x + &y)?, 2.0, Comparator::Eq, Some("c0".to_string()))?,
            None,
        )?;
        model.sense = Sense::Min;

        let pm = PassManager::default()
            .add_analysis(MaxBiasAnalysis)
            .add_transform(EqualityConstraintsToQuadraticPenaltyPass::new(10.0));
        let output = pm.run(model)?;

        let blob = output.record.encode(Some(true), Some(3))?;
        let recovered: TransformationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_integer("i0".into(), vec![2.0, 7.0], None)?;
        solution.add_integer("i1".into(), vec![2.0, 2.0], None)?;
        solution = output.model.evaluate_solution(&solution)?;

        let back_direct = output.record.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));
        assert!(back_direct.obj_values.is_none());

        Ok(())
    }

    /// Penalized objective values for (x, y) = (0, 1) [feasible] and (1, 1)
    /// [infeasible] of `x + 3y` subject to `x + y == 1`.
    fn penalized_values(sense: Sense) -> LunaModelResult<Vec<f64>> {
        let mut model = Model::default();
        let x = model.add_var("x", Vtype::Binary, None)?;
        let y = model.add_var("y", Vtype::Binary, None)?;
        model.objective_mut().add_assign((&x + (&y * 3.0)?)?)?;
        model.add_constraint(
            Constraint::new((&x + &y)?, 1.0, Comparator::Eq, Some("one".to_string()))?,
            None,
        )?;
        model.sense = sense;

        let pm = PassManager::default()
            .add_analysis(MaxBiasAnalysis)
            .add_transform(EqualityConstraintsToQuadraticPenaltyPass::new(10.0));
        let output = pm.run(model)?;

        let mut solution = Solution::default();
        solution.add_binary("x".into(), vec![0.0, 1.0], None)?;
        solution.add_binary("y".into(), vec![1.0, 1.0], None)?;
        Ok(output
            .model
            .evaluate_solution(&solution)?
            .obj_values
            .unwrap())
    }

    #[test]
    fn penalty_makes_violation_worse_when_maximizing() -> LunaModelResult<()> {
        let v = penalized_values(Sense::Max)?;
        assert!(
            v[0] > v[1],
            "feasible {} must beat infeasible {}",
            v[0],
            v[1]
        );
        Ok(())
    }

    #[test]
    fn penalty_makes_violation_worse_when_minimizing() -> LunaModelResult<()> {
        let v = penalized_values(Sense::Min)?;
        assert!(
            v[0] < v[1],
            "feasible {} must beat infeasible {}",
            v[0],
            v[1]
        );
        Ok(())
    }
}
