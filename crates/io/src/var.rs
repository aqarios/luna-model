use super::{CustomFormat, FormatOpt};
use lunamodel_core::prelude::VarRef;

impl CustomFormat<FormatOpt> for VarRef {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match format_type {
            FormatOpt::Rs => write!(fmt, "{}", self),
            #[cfg(feature = "py")]
            FormatOpt::Py => {
                use lunamodel_core::prelude::Variable;
                use lunamodel_types::{Bound, Vtype};

                let var: Option<Variable> = self.try_into().ok();
                if let Some(var) = var {
                    let mut base = format!("{}: {}", var.name(), var.vtype());
                    if !(var.vtype() == Vtype::Binary
                        || var.vtype() == Vtype::Spin
                        || var.vtype() == Vtype::InvertedBinary)
                    {
                        match (var.bounds().lower(), var.bounds().upper()) {
                            (Bound::Bounded(l), Bound::Bounded(u)) => base.push_str(&format!(
                                "(lower={}, upper={})",
                                match l {
                                    0.0 => "0",
                                    l => &l.to_string(),
                                },
                                match u {
                                    0.0 => "0",
                                    l => &l.to_string(),
                                }
                            )),
                            (Bound::Bounded(l), Bound::Unbounded) => base.push_str(&format!(
                                "(lower={})",
                                match l {
                                    0.0 => "0".to_string(),
                                    l => l.to_string(),
                                }
                            )),
                            (Bound::Unbounded, Bound::Bounded(u)) => base.push_str(&format!(
                                "(upper={})",
                                match u {
                                    0.0 => "0".to_string(),
                                    u => u.to_string(),
                                }
                            )),
                            (Bound::Unbounded, Bound::Unbounded) => (),
                        };
                    }
                    write!(fmt, "{}", base)
                } else {
                    write!(fmt, "<deleted>")
                }
            }
            #[cfg(feature = "py")]
            FormatOpt::PySol(_) => unreachable!("cannot format VarRef as Solution."),
        }
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
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
            #[cfg(feature = "py")]
            FormatOpt::PySol(_) => unreachable!("cannot format VarRef as Solution."),
        }
    }
}
