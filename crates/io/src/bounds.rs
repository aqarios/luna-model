use super::{CustomFormat, FormatOpt};
use lunamodel_core::prelude::{Bounds, LazyBounds};
use lunamodel_types::Bound;

impl CustomFormat<FormatOpt> for Bounds {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        _ = format_type;
        write!(
            fmt,
            "Bounds(lower={}, upper={})",
            match self.lower() {
                Bound::Bounded(0.0) => "0",
                r => &r.to_string(),
            },
            match self.upper() {
                Bound::Bounded(0.0) => "0",
                r => &r.to_string(),
            },
        )
    }
}

impl CustomFormat<FormatOpt> for LazyBounds {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        _ = format_type;
        let (l, u) = fmt_maybe_bounds(self.lower(), self.upper());
        write!(fmt, "Bounds(lower={}, upper={})", l, u)
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        _ = format_type;
        let (l, u) = fmt_maybe_bounds(self.lower(), self.upper());
        write!(fmt, "LazyBounds(lower={}, upper={})", l, u)
    }
}

fn fmt_maybe_bounds(lower: Option<Bound>, upper: Option<Bound>) -> (String, String) {
    match (lower, upper) {
        (Some(l), Some(u)) => (l.to_string(), u.to_string()),
        (Some(l), None) => (l.to_string(), "None".into()),
        (None, Some(u)) => ("None".into(), u.to_string()),
        (None, None) => ("None".into(), "None".into()),
    }
}
