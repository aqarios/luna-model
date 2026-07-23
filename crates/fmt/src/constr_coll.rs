//! Formatting helpers for constraint collections.

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
                                .map(|(cname, c)| format!("{cname}: {}", c.format(FormatOpt::Rs)))
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
                if self.is_empty() {
                    "{}".to_string()
                } else {
                    let items = self
                        .iter()
                        .map(|(cname, c)| format!("{cname}: {}", c.format(FormatOpt::Py)))
                        .collect::<Vec<String>>();

                    if items.len() == 1 {
                        format!("{{{}}}", items[0])
                    } else {
                        let body = items
                            .into_iter()
                            .map(|item| format!("  {item},"))
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!("{{\n{}\n}}", body)
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
