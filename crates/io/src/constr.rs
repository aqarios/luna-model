use lunamodel_core::prelude::Constraint;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Constraint {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{}", self)
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

