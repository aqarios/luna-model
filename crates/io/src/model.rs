use lunamodel_core::Model;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Model {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: &FormatOpt) -> std::fmt::Result {
        write!(fmt, "{}", self)
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: &FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}


