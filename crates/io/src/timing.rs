//! Formatting helpers for timing values.

use lunamodel_core::Timing;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use super::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Timing {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        _ = format_type;
        let qpu = match self.qpu {
            Some(val) => &val.to_string(),
            None => "None",
        };
        write!(
            fmt,
            "Timing(start={}, stop={}, qpu={})",
            OffsetDateTime::from(self.start()).format(&Rfc3339).unwrap(),
            OffsetDateTime::from(self.end()).format(&Rfc3339).unwrap(),
            qpu
        )
    }

    // fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
    //     _ = format_type;
    //     let (l, u) = fmt_maybe_bounds(self.lower(), self.upper());
    //     write!(fmt, "LazyBounds(lower={}, upper={})", l, u)
    // }
}
