// pub sol_merge_many: unsafe extern "C" fn(
//     *const *const LmSol,
//     usize,
//     *const LmModel,
//     *const f64,
//     *mut *mut LmSol,
// ) -> LmStatus,
//
// pub sol_cvar:
//     unsafe extern "C" fn(*const LmSol, f64, *const LmValueSource, *mut f64) -> LmStatus,
// pub sol_expectation_value:
//     unsafe extern "C" fn(*const LmSol, *const LmValueSource, *mut f64) -> LmStatus,
// pub sol_temperature_weighted:
//     unsafe extern "C" fn(*const LmSol, f64, *const LmValueSource, *mut f64) -> LmStatus,
// pub sol_feasibility_ratio: unsafe extern "C" fn(*const LmSol, *mut f64) -> LmStatus,
// pub sol_highest_constraint_violations:
//     unsafe extern "C" fn(*const LmSol, *mut LmBuffer) -> LmStatus,
// pub sol_serialize: unsafe extern "C" fn(*const LmSol, *mut LmBuffer) -> LmStatus,
// pub sol_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmSol) -> LmStatus,
