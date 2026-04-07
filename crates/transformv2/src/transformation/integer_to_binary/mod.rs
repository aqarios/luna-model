mod artifact;
mod pass;
mod ser_artifact;

pub use pass::IntegerToBinaryPass;

#[cfg(test)]
mod tests {
    use lunamodel_core::{
        Model, Solution,
        ops::LmAddAssign,
        prelude::{ContentEquality, LazyBounds},
    };
    use lunamodel_error::LunaModelResult;
    use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
    use lunamodel_transpiler::{CompilationRecord, PassManager, register_backward};
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

        let mut pm = PassManager::default().add_transform(IntegerToBinaryPass::default());
        let cr = pm.run(&mut model)?;

        assert_eq!(2, model.num_variables());

        let blob = cr.encode(Some(true), Some(3))?;
        let recovered: CompilationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        dbg!(&model.environment);

        let mut solution = Solution::default();
        solution.add_binary("i0_b0".into(), vec![1.0, 0.0])?;
        solution.add_binary("i0_b1".into(), vec![0.0, 1.0])?;
        solution.counts = vec![1, 1];

        eprintln!("a");
        let back_direct = cr.backward(solution.clone())?;
        eprintln!("b");
        let back_recovered = recovered.backward(solution.clone())?;
        eprintln!("c");

        assert!(back_direct.equal_contents(&back_recovered));

        Ok(())
    }
}
