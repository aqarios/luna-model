use std::fmt::{self, Debug, Display};
// Based on Source - https://stackoverflow.com/a
// Posted by Kevin Reid
// Retrieved 2025-12-09, License - CC BY-SA 4.0
pub struct CustomFormatWrapper<'a, F: Copy, T: CustomFormat<F> + ?Sized>(F, &'a T);

impl<'a, F: Copy, T: CustomFormat<F>> Display for CustomFormatWrapper<'a, F, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as CustomFormat<F>>::fmt(self.1, fmt, self.0)
    }
}

impl<'a, F: Copy, T: CustomFormat<F>> Debug for CustomFormatWrapper<'a, F, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as CustomFormat<F>>::dbg(self.1, fmt, self.0)
    }
}

pub trait CustomFormat<F: Copy> {
    fn format(&self, format_type: F) -> CustomFormatWrapper<'_, F, Self> {
        CustomFormatWrapper(format_type, self)
    }

    fn dbg(&self, fmt: &mut fmt::Formatter<'_>, format_type: F) -> fmt::Result {
        self.fmt(fmt, format_type)
    }

    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, format_type: F) -> fmt::Result;
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
//     fn fmt(&self, fmt: &mut fmt::Formatter<'_>, format_type: FormatOpt) -> fmt::Result {
//         match format_type {
//             FormatOpt::Rs => write!(fmt, "{}", self),
//             #[cfg(feature = "py")]
//             FormatOpt::Py => self.py_fmt(fmt),
//         }
//     }
//
//     fn dbg(&self, fmt: &mut fmt::Formatter<'_>, format_type: FormatOpt) -> fmt::Result {
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
