pub mod analysis;
pub mod control_flow;
pub mod pipelines;
pub mod transformation;

pub fn register_backward() {
    lunamodel_transpiler::register_backward::<transformation::BinarySpinPass>();
    lunamodel_transpiler::register_backward::<transformation::ChangeSensePass>();
    lunamodel_transpiler::register_backward::<
        transformation::EqualityConstraintsToQuadraticPenaltyPass,
    >();
    lunamodel_transpiler::register_backward::<transformation::GeToLeConstraintsPass>();
    lunamodel_transpiler::register_backward::<transformation::IntegerToBinaryPass>();
    lunamodel_transpiler::register_backward::<transformation::LeToEqConstraintsPass>();
}
