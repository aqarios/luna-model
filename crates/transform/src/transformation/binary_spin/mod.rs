mod artifact;
mod pass;
mod ser_artifact;

pub use artifact::BinarySpinPassArtifact;
pub use pass::BinarySpinPass;

#[cfg(test)]
mod tests {
    use lunamodel_core::{Model, Solution, ops::LmAddAssign, prelude::ContentEquality};
    use lunamodel_error::LunaModelResult;
    use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
    use lunamodel_transpiler::{PassManager, TransformationRecord, register_backward};
    use lunamodel_types::Vtype;

    use crate::transformation::BinarySpinPass;

    #[test]
    fn roundtrip_binary_spin_pass_bin() -> LunaModelResult<()> {
        register_backward::<BinarySpinPass>();

        let mut model = Model::default();
        let x = model.add_var("b0", Vtype::Binary, None)?;
        let y = model.add_var("b1", Vtype::Binary, None)?;
        model.objective.add_assign((&x * &y)?)?;

        let pm = PassManager::default().add_transform(BinarySpinPass::new(Vtype::Spin, None));
        let output = pm.run(model)?;

        let blob = output.record.encode(Some(true), Some(3))?;
        let recovered: TransformationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_spin("s_b0".into(), vec![-1.0, 1.0], None)?;
        solution.add_spin("s_b1".into(), vec![1.0, -1.0], None)?;

        let back_direct = output.record.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));
        assert!(back_direct.obj_values.is_none());

        Ok(())
    }

    #[test]
    fn roundtrip_binary_spin_pass_spin() -> LunaModelResult<()> {
        let mut model = Model::default();
        let x = model.add_var("s0", Vtype::Spin, None)?;
        let y = model.add_var("s1", Vtype::Spin, None)?;
        model.objective.add_assign((&x * &y)?)?;

        let pm = PassManager::default().add_transform(BinarySpinPass::new(Vtype::Binary, None));
        let output = pm.run(model)?;

        let blob = output.record.encode(Some(true), Some(3))?;
        let recovered: TransformationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_binary("x_s0".into(), vec![1.0, 0.0], None)?;
        solution.add_binary("x_s1".into(), vec![0.0, 1.0], None)?;

        let back_direct = output.record.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));
        assert!(back_direct.obj_values.is_none());

        Ok(())
    }
}
