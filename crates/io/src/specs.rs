//! Formatting helpers for structural model specs.

use lunamodel_types::Specs;

use super::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Specs {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        _ = format_type;
        write!(
            fmt,
            "ModelSpecs(sense={}, vtype={}, constraints={}, max_degree={}, max_constraint_degree={}, max_num_variables={})",
            self.sense
                .map_or_else(|| "None".to_string(), |s| s.to_string()),
            self.vtypes
                .map_or_else(|| String::from("None"), |v| v.to_string()),
            self.constraints
                .map_or_else(|| String::from("None"), |v| v.to_string()),
            self.max_degree
                .map_or_else(|| String::from("None"), |v| v.to_string()),
            self.max_constraint_degree
                .map_or_else(|| String::from("None"), |v| v.to_string()),
            self.max_num_variables
                .map_or_else(|| String::from("None"), |v| v.to_string()),
        )
    }

    // fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
    //     _ = format_type;
    //     let (l, u) = fmt_maybe_bounds(self.lower(), self.upper());
    //     write!(fmt, "LazyBounds(lower={}, upper={})", l, u)
    // }
}
