use lunamodel_core::ConstraintCollection;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for ConstraintCollection {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_opt: &FormatOpt) -> std::fmt::Result {
        match format_opt {
            FormatOpt::Rs => write!(
                fmt,
                "{}",
                match self.is_empty() {
                    true => "{}".to_string(),
                    false => {
                        format!(
                            "{{{}}}",
                            self.iter()
                                .map(|(cname, c)| format!("{cname}: {}", c.format(FormatOpt::Py)))
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    }
                }
            ),
            #[cfg(feature = "py")]
            FormatOpt::Py => write!(
                fmt,
                "{}",
                match self.is_empty() {
                    true => "{}".to_string(),
                    false => {
                        format!(
                            "{{{}}}",
                            self.iter()
                                .map(|(cname, c)| format!("{cname}: {}", c.format(FormatOpt::Py)))
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    }
                }
            ),
            #[cfg(feature = "py")]
            FormatOpt::PySol(_) => {
                unreachable!("cannot format ConstraintCollection for PySol opts")
            }
        }
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: &FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
