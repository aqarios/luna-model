//! Objective-sense normalization pass and artifact types.
mod artifact;
mod pass;
mod ser_artifact;

pub use artifact::ChangeSensePassArtifact;
pub use pass::ChangeSensePass;

#[cfg(test)]
mod tests {
    use lunamodel_core::{Model, Solution, ops::LmAddAssign, prelude::ContentEquality};
    use lunamodel_error::LunaModelResult;
    use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
    use lunamodel_transpiler::{PassManager, TransformationRecord, register_backward};
    use lunamodel_types::{Sense, Vtype};

    use crate::transformation::ChangeSensePass;

    #[test]
    fn roundtrip_change_sense_pass_to_max() -> LunaModelResult<()> {
        register_backward::<ChangeSensePass>();

        let mut model = Model::default();
        let x = model.add_var("i0", Vtype::Integer, None)?;
        let y = model.add_var("i1", Vtype::Integer, None)?;
        model.objective.add_assign((&x * &y)?)?;
        model.sense = Sense::Min;

        let pm = PassManager::default().add_transform(ChangeSensePass::new(Sense::Max));
        let output = pm.run(model.clone())?;

        let blob = output.record.encode(Some(true), Some(3))?;
        let recovered: TransformationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_integer("i0".into(), vec![2.0, 7.0], None)?;
        solution.add_integer("i1".into(), vec![2.0, 2.0], None)?;
        solution = model.evaluate_solution(&solution)?;

        let back_direct = output.record.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));
        assert!(back_direct.obj_values.is_none());

        Ok(())
    }

    #[test]
    fn roundtrip_change_sense_pass_to_min() -> LunaModelResult<()> {
        register_backward::<ChangeSensePass>();

        let mut model = Model::default();
        let x = model.add_var("i0", Vtype::Integer, None)?;
        let y = model.add_var("i1", Vtype::Integer, None)?;
        model.objective.add_assign((&x * &y)?)?;
        model.sense = Sense::Max;

        let pm = PassManager::default().add_transform(ChangeSensePass::new(Sense::Min));
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
}
