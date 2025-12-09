#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum FormatOpt {
    Rs,
    #[cfg(feature = "py")]
    Py,
}
