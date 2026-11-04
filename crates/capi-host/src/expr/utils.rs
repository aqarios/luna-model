use lunamodel_capi_handle::{ExpressionHandle, drop_handle, handle_from_storage};
use lunamodel_core::Expression;

use crate::{LmExpr, error::CapiError, utils::storage_from_c};

pub(crate) fn expression_to_c(expression: Expression) -> *mut LmExpr {
    handle_from_storage::<LmExpr>(ExpressionHandle::new(expression))
}

pub(crate) unsafe fn expression_from_c<'a>(
    expression: *const LmExpr,
    what: &str,
) -> Result<&'a ExpressionHandle, CapiError> {
    unsafe { storage_from_c(expression, what) }
}

pub(crate) unsafe fn expression_out_from_c(
    out: *mut *mut LmExpr,
    expression: Expression,
) -> Result<(), CapiError> {
    let out = unsafe { crate::utils::out_from_c(out) }?;
    *out = expression_to_c(expression);
    Ok(())
}

pub(crate) unsafe fn expression_drop_from_c(expression: *mut LmExpr) {
    unsafe { drop_handle(expression) }
}
