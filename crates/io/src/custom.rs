use std::fmt::{self, Debug, Display};
// Based on Source - https://stackoverflow.com/a
// Posted by Kevin Reid
// Retrieved 2025-12-09, License - CC BY-SA 4.0
/// Display/debug wrapper returned by [`CustomFormat::format`].
pub struct CustomFormatWrapper<'a, F: Clone, T: CustomFormat<F> + ?Sized>(F, &'a T);

impl<'a, F: Clone, T: CustomFormat<F>> Display for CustomFormatWrapper<'a, F, T> {
    /// Delegates display formatting to the wrapped [`CustomFormat`] impl.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as CustomFormat<F>>::fmt(self.1, f, &self.0)
    }
}

impl<'a, F: Clone, T: CustomFormat<F>> Debug for CustomFormatWrapper<'a, F, T> {
    /// Delegates debug formatting to the wrapped [`CustomFormat`] impl.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as CustomFormat<F>>::dbg(self.1, f, &self.0)
    }
}

/// Formatting trait parameterized by a caller-supplied format option type.
pub trait CustomFormat<F: Clone> {
    /// Returns a wrapper that can participate in normal `Display`/`Debug` formatting.
    fn format(&self, format_type: F) -> CustomFormatWrapper<'_, F, Self> {
        CustomFormatWrapper(format_type, self)
    }

    /// Debug-style formatting hook. By default this falls back to [`CustomFormat::fmt`].
    fn dbg(&self, f: &mut fmt::Formatter<'_>, format_type: &F) -> fmt::Result {
        self.fmt(f, format_type)
    }

    /// Display-style formatting hook.
    fn fmt(&self, f: &mut fmt::Formatter<'_>, format_type: &F) -> fmt::Result;
}

// pub trait PyFormat {
//     fn py_fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result;
//     fn py_dbg(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result;
// }
//
// impl<T> PyFormat<FormatOpt> for T
// where
//     T: Display + Debug,
// {
//     fn fmt(&self, fmt: &mut fmt::Formatter<'_>, format_type: &FormatOpt) -> fmt::Result {
//         match format_type {
//             FormatOpt::Rs => write!(fmt, "{}", self),
//             #[cfg(feature = "py")]
//             FormatOpt::Py => self.py_fmt(fmt),
//         }
//     }
//
//     fn dbg(&self, fmt: &mut fmt::Formatter<'_>, format_type: &FormatOpt) -> fmt::Result {
//         match format_type {
//             FormatOpt::Rs => write!(fmt, "{:?}", self),
//             #[cfg(feature = "py")]
//             FormatOpt::Py => self.py_dbg(fmt),
//         }
//     }
//
//     fn py_fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
//         _ = fmt;
//         unreachable!()
//     }
//     fn py_dbg(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
//         _ = fmt;
//         unreachable!()
//     }
// }
