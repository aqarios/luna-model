mod artifact;
mod pass;

pub use pass::LeToEqConstraintsPass;
pub use artifact::LeToEqConstraintsArtifact;

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

    use crate::transformation::LeToEqConstraintsPass;

    #[test]
    fn roundtrip_le_to_eq_constraints_pass() -> LunaModelResult<()> {
        register_backward::<LeToEqConstraintsPass>();

        let mut model = Model::default();
        let x = model.add_var("i0", Vtype::Integer, None)?;
        let y = model.add_var("i1", Vtype::Integer, None)?;
        model.objective.add_assign((&x * &y)?)?;
        model.constraints.add_constraint(
            Constraint::new((&x + &y)?, 2.0, Comparator::Le, Some("c0".to_string()))?,
            None,
        )?;
        model.sense = Sense::Min;

        let pm = PassManager::default().add_transform(LeToEqConstraintsPass::default());
        let output = pm.run(model)?;

        let blob = output.record.encode(Some(true), Some(3))?;
        let recovered: TransformationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_integer("i0".into(), vec![2.0, 7.0])?;
        solution.add_integer("i1".into(), vec![2.0, 2.0])?;
        solution = output.model.evaluate_solution(&solution)?;

        let back_direct = output.record.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));

        Ok(())
    }
}
