use std::{collections::HashMap, sync::RwLock};

use pyo3::{
    Bound, Py, PyErr, PyResult, Python,
    exceptions::PyTypeError,
    sync::PyOnceLock,
    types::{PyAnyMethods, PyType, PyTypeMethods},
};

struct ClassPair {
    public: Py<PyType>,
    inner: Py<PyType>,
}

static CLASSES: PyOnceLock<RwLock<HashMap<&'static str, ClassPair>>> = PyOnceLock::new();

fn class_pair<'py>(
    py: Python<'py>,
    base: &'static str,
) -> PyResult<(Bound<'py, PyType>, Bound<'py, PyType>)> {
    let map = CLASSES.get_or_init(py, || RwLock::new(HashMap::new()));
    if let Some(p) = map.read().unwrap().get(base) {
        return Ok((p.public.bind(py).clone(), p.inner.bind(py).clone()));
    }
    let lm = crate::luna_model(py)?;
    let public = lm.getattr(base)?.cast_into::<PyType>()?;
    let inner = lm
        .getattr("_lm")?
        .getattr(format!("Py{base}"))?
        .cast_into::<PyType>()?;
    map.write().unwrap().insert(
        base,
        ClassPair {
            public: public.clone().unbind(),
            inner: inner.clone().unbind(),
        },
    );
    Ok((public, inner))
}

pub trait TypeCheck {
    fn check_type(&self, base: &'static str) -> Result<(), PyErr>;
    fn check_type_literal(&self, base: &'static str) -> Result<(), PyErr>;
}

impl<'a, 'py> TypeCheck for pyo3::Borrowed<'a, 'py, pyo3::PyAny> {
    fn check_type(&self, base: &'static str) -> Result<(), PyErr> {
        let (public, inner) = class_pair(self.py(), base)?;
        if self.is_instance(&public)? || self.is_instance(&inner)? {
            return Ok(());
        }
        Err(PyTypeError::new_err(format!(
            "Argument must be an instance of '{base}' or 'Py{base}'. Found '{}'.",
            self.get_type().name()?,
        )))
    }

    fn check_type_literal(&self, base: &'static str) -> Result<(), PyErr> {
        let (public, inner) = class_pair(self.py(), base)?;
        // identity comparison: caller passed the class object itself
        if self.is(&public) || self.is(&inner) {
            return Ok(());
        }
        let what = match self.cast::<PyType>() {
            Ok(t) => format!("type '{}'", t.name()?),
            Err(_) => format!("instance of '{}'", self.get_type().name()?),
        };
        Err(PyTypeError::new_err(format!(
            "Argument must be the class '{base}' or 'Py{base}'. Found {what}."
        )))
    }
}
