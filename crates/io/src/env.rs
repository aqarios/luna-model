use lunamodel_core::Environment;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Environment {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{}", self)
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

