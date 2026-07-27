use derive_more::Deref;

/// Translation targets are Python-facing dispatch values rather than core
/// model objects. Keeping this small enum local avoids linking the translator
/// implementation into an extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationTarget {
    Qubo,
    Lp,
    Mps,
    Bqm,
    Cqm,
    OptMapper,
}

#[repr(transparent)]
#[derive(Deref)]
pub struct PyTranslationTarget(pub TranslationTarget);

python_enum_wrapper! {
    wrapper: PyTranslationTarget,
    public: TranslationTarget,
    inner: TranslationTarget,
    variants: {
        TranslationTarget::Qubo => ("QUBO", "Qubo"),
        TranslationTarget::Lp => ("LP", "Lp"),
        TranslationTarget::Mps => ("MPS", "Mps"),
        TranslationTarget::Bqm => ("BQM", "Bqm"),
        TranslationTarget::Cqm => ("CQM", "Cqm"),
        TranslationTarget::OptMapper => ("OPT_MAPPER", "OptMapper"),
    },
}
