use super::{CustomFormat, FormatOpt};
use lunamodel_core::prelude::VarRef;

impl CustomFormat<FormatOpt> for VarRef {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: FormatOpt) -> std::fmt::Result {
        match format_type {
            FormatOpt::Rs => write!(fmt, "{}", self),
            #[cfg(feature = "py")]
            FormatOpt::Py => {
                if let Some(name) = self.name().ok() {
                    write!(fmt, "{}", name)
                } else {
                    write!(fmt, "<deleted>")
                }
            }
        }
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: FormatOpt) -> std::fmt::Result {
        match format_type {
            FormatOpt::Rs => write!(fmt, "{:?}", self),
            #[cfg(feature = "py")]
            FormatOpt::Py => {
                if let Some(name) = self.name().ok() {
                    write!(
                        fmt,
                        "Variable(name=\"{}\", vtype={}, id={}, env={})",
                        name,
                        self.vtype().unwrap(),
                        self.id(),
                        self.env.id(),
                    )
                } else {
                    write!(
                        fmt,
                        "DeletedVariable(id={}, env={})",
                        self.id(),
                        self.env.id()
                    )
                }
            }
        }
    }
}
