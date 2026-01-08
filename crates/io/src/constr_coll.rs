use lunamodel_core::ConstraintCollection;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for ConstraintCollection {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{}", self)
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
