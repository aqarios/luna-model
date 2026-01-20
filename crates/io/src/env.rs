use lunamodel_core::Environment;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Environment {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_opt: &FormatOpt) -> std::fmt::Result {
        match format_opt {
            FormatOpt::Rs => write!(fmt, "{}", self),
            #[cfg(feature = "py")]
            FormatOpt::Py => write!(
                fmt,
                "Environment {}\n  {}",
                self.id(),
                self.vars()
                    .map(|v| self.get(v).unwrap().name().to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            #[cfg(feature = "py")]
            FormatOpt::PySol(_) => unreachable!("cannot format Constraint for PySol opts"),
        }
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: &FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
