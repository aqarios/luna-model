use lunamodel_core::Model;

use crate::TransformationRecord;

/// Result of the forward execution containing the input model, the output model after applying the
/// pass manager and the compilation record for backward execution.
pub struct TransformationOutput {
    /// A record of the forward compilation, structured for backwards execution.
    pub record: TransformationRecord,

    /// The output model after all transformations are applied.
    pub model: Model,
    // /// The output model after all transformations are applied.
    // input: Model,
}
