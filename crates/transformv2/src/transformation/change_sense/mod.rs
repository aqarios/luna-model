mod artifact;
mod pass;
mod ser_artifact;

pub use pass::ChangeSensePass;

#[cfg(test)]
mod tests {
    use lunamodel_core::{Model, Solution, ops::LmAddAssign, prelude::ContentEquality};
    use lunamodel_error::LunaModelResult;
    use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
    use lunamodel_transpiler::{CompilationRecord, PassManager};
    use lunamodel_types::{Sense, Vtype};

    use crate::transformation::ChangeSensePass;

    #[test]
    fn roundtrip_change_sense_pass_to_max() -> LunaModelResult<()> {
        let mut model = Model::default();
        let x = model.add_var("i0", Vtype::Integer, None)?;
        let y = model.add_var("i1", Vtype::Integer, None)?;
        model.objective.add_assign((&x * &y)?)?;
        model.sense = Sense::Min;

        let mut pm = PassManager::default().add_transform(ChangeSensePass::new(Sense::Max));
        let cr = pm.run(&mut model)?;

        let blob = cr.encode(Some(true), Some(3))?;
        let recovered: CompilationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_integer("i0".into(), vec![2.0, 7.0])?;
        solution.add_integer("i1".into(), vec![2.0, 2.0])?;
        solution = model.evaluate_solution(&solution)?;

        let back_direct = cr.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));

        Ok(())
    }

    #[test]
    fn roundtrip_change_sense_pass_to_min() -> LunaModelResult<()> {
        let mut model = Model::default();
        let x = model.add_var("i0", Vtype::Integer, None)?;
        let y = model.add_var("i1", Vtype::Integer, None)?;
        model.objective.add_assign((&x * &y)?)?;
        model.sense = Sense::Max;

        let mut pm = PassManager::default().add_transform(ChangeSensePass::new(Sense::Min));
        let cr = pm.run(&mut model)?;

        let blob = cr.encode(Some(true), Some(3))?;
        let recovered: CompilationRecord =
            blob.as_slice().unversionize().decompress()?.decode(())?;

        let mut solution = Solution::default();
        solution.add_integer("i0".into(), vec![2.0, 7.0])?;
        solution.add_integer("i1".into(), vec![2.0, 2.0])?;
        solution = model.evaluate_solution(&solution)?;

        let back_direct = cr.backward(solution.clone())?;
        let back_recovered = recovered.backward(solution.clone())?;

        assert!(back_direct.equal_contents(&back_recovered));

        Ok(())
    }
}
