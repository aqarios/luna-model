//! Formatting helpers for timing values.

use lunamodel_core::Timing;

use super::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Timing {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        _ = format_type;
        let timings = self
            .timings
            .iter()
            .map(|(key, values)| {
                let sum: f64 = values.iter().sum();
                format!("{key}={sum:?}")
            })
            .collect::<Vec<String>>()
            .join(", ");
        write!(fmt, "Timing(total={:?}, {})", self.total, timings)
    }
}
