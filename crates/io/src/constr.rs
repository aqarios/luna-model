use lunamodel_core::prelude::Constraint;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Constraint {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match format_type {
            FormatOpt::Rs => write!(
                fmt,
                "{} {} {}",
                self.lhs.format(FormatOpt::Rs),
                self.comparator.to_string(),
                self.rhs.to_string()
            ),
            #[cfg(feature = "py")]
            FormatOpt::Py => write!(
                fmt,
                "{} {} {}",
                self.lhs.format(FormatOpt::Py),
                self.comparator.to_string(),
                self.rhs.to_string()
            ),
            #[cfg(feature = "py")]
            FormatOpt::PySol(_) => unreachable!("cannot format Constraint for PySol opts"),
        }
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match format_type {
            FormatOpt::Rs => write!(fmt, "{:?}", self),
            #[cfg(feature = "py")]
            FormatOpt::Py => self.fmt(fmt, &FormatOpt::Py),
            #[cfg(feature = "py")]
            FormatOpt::PySol(_) => unreachable!("cannot format Constraint for PySol opts"),
        }
    }
}
