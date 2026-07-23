use lunamodel_core::ConstraintCollection;
use lunamodel_cext_handle::{ConstraintCollectionHandle, drop_handle, handle_from_storage};

use crate::{LmCC, error::CapiError, utils::storage_from_c};

pub(crate) fn collection_to_c(collection: ConstraintCollection) -> *mut LmCC {
    handle_from_storage::<LmCC>(ConstraintCollectionHandle::new(collection))
}

pub(crate) unsafe fn collection_from_c<'a>(
    collection: *const LmCC,
    what: &str,
) -> Result<&'a ConstraintCollectionHandle, CapiError> {
    unsafe { storage_from_c(collection, what) }
}

pub(crate) unsafe fn collection_out_from_c(
    out: *mut *mut LmCC,
    collection: ConstraintCollection,
) -> Result<(), CapiError> {
    let out = unsafe { crate::utils::out_from_c(out) }?;
    *out = collection_to_c(collection);
    Ok(())
}

pub(crate) unsafe fn collection_drop_from_c(collection: *mut LmCC) {
    unsafe { drop_handle(collection) }
}
