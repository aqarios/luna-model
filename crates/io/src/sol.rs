use lunamodel_core::Solution;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Solution {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{}", self.to_string())
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
