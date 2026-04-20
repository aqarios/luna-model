mod artifact;
mod pass;
mod ser_artifact;

pub use pass::IntegerToBinaryPass;
pub use artifact::IntegerToBinaryArtifact;

#[cfg(test)]
mod tests {
    use lunamodel_core::{
        Model, Solution,
        ops::LmAddAssign,
        prelude::{ContentEquality, LazyBounds},
    };
    use lunamodel_error::LunaModelResult;
    use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
    use lunamodel_transpiler::{PassManager, TransformationRecord, register_backward};
    use lunamodel_types::{Bound, Vtype};

    use crate::transformation::IntegerToBinaryPass;

    #[test]
    fn roundtrip_integer_to_binary_pass() -> LunaModelResult<()> {
        register_backward::<IntegerToBinaryPass>();

        let mut model = Model::default();
        let x = model.add_var(
            "i0",
            Vtype::Integer,
            Some(LazyBounds::new(
                Some(Bound::Bounded(0.0)),
                Some(Bound::Bounded(2.0)),
            )),
        )?;
        model.objective.add_assign((&x * 1.0)?)?;

        let pm = PassManager::default().add_transform(IntegerToBinaryPass::default());
        let output = pm.run(model)?;

        assert_eq!(2, output.model.num_variables());

        let blob = output.record.encode(Some(true), Some(3))?;
        let recovered: TransformationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_binary("i0_b0".into(), vec![1.0, 0.0], None)?;
        solution.add_binary("i0_b1".into(), vec![0.0, 1.0], None)?;
        solution.counts = vec![1, 1];

        let back_direct = output.record.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));

        Ok(())
    }
}
