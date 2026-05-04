//! Inverted-binary reduction pass and artifact types.
mod artifact;
mod pass;

pub use artifact::ReduceInvertedBinaryPassArtifact;
pub use pass::ReduceInvertedBinaryPass;

#[cfg(test)]
mod tests {
    use lunamodel_core::{Model, Solution, ops::LmAddAssign, prelude::ContentEquality};
    use lunamodel_error::LunaModelResult;
    use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
    use lunamodel_transpiler::{PassManager, TransformationRecord, register_backward};
    use lunamodel_types::{Sense, Vtype};

    use crate::transformation::ReduceInvertedBinaryPass;

    #[test]
    fn roundtrip_equality_constraints_to_quadratic_penalty_pass() -> LunaModelResult<()> {
        register_backward::<ReduceInvertedBinaryPass>();

        let mut model = Model::default();
        let x = model.add_var("b0", Vtype::Binary, None)?;
        let y = model.add_var("b1", Vtype::Binary, None)?;
        model.objective.add_assign(((&x * &y)? + (!&x)?)?)?;
        model.sense = Sense::Min;

        let pm = PassManager::default().add_transform(ReduceInvertedBinaryPass::default());
        let output = pm.run(model)?;

        let blob = output.record.encode(Some(true), Some(3))?;
        let recovered: TransformationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_binary("b0".into(), vec![1.0, 0.0], None)?;
        solution.add_binary("b1".into(), vec![1.0, 0.0], None)?;
        solution = output.model.evaluate_solution(&solution)?;

        let back_direct = output.record.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));
        assert!(back_direct.obj_values.is_none());

        Ok(())
    }
}
