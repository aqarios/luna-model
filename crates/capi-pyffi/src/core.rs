use std::ffi::CStr;

use lunamodel_capi::{
    LmCC, LmConstraint, LmEnv, LmExpr, LmLazyBounds, LmModel, LmSol, LmSpecs, LmVref,
};

use super::CapsuleKind;

impl CapsuleKind for LmModel {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Model.v1";
}

impl CapsuleKind for LmSol {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Solution.v1";
}

impl CapsuleKind for LmEnv {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Environment.v1";
}

impl CapsuleKind for LmExpr {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Expression.v1";
}

impl CapsuleKind for LmCC {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.ConstraintCollection.v1";
}

impl CapsuleKind for LmConstraint {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Constraint.v1";
}

impl CapsuleKind for LmVref {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Variable.v1";
}

impl CapsuleKind for LmLazyBounds {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.Bounds.v1";
}

impl CapsuleKind for LmSpecs {
    const CAPSULE_NAME: &'static CStr = c"luna_model.capi.ModelSpecs.v1";
}
