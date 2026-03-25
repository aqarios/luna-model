use std::fmt::Display;
use std::sync::Arc;

use std::collections::HashMap;
use pyo3::{IntoPyObjectExt, prelude::*, types::PyDict};

use parking_lot::RwLock;

#[pyclass(subclass, name = "PyModelMetadata")]
#[derive(Clone, Debug)]
pub struct PyModelMetadata {
    pub data: Arc<RwLock<HashMap<String, Py<PyAny>>>>,
}

impl PyModelMetadata {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[pymethods]
impl PyModelMetadata {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    fn __len__(&self) -> usize {
        self.data.read_arc().len()
    }

    fn __contains__(&self, key: String) -> bool {
        self.data.read_arc().contains_key(&key)
    }

    fn __getitem__(&self, py: Python, key: String) -> Py<PyAny> {
        self.get_item(py, key)
    }

    fn __setitem__(&mut self, key: String, value: Py<PyAny>) {
        self.set_item(key, value)
    }

    fn __delitem__(&mut self, key: String) {
        self.del_item(key)
    }

    fn __str__(&self) -> String {
        format!("{self}")
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn get_item(&self, py: Python, key: String) -> Py<PyAny> {
        #[allow(deprecated)]
        self.data
            .read_arc()
            .get(&key)
            .map(|v| v.into_py_any(py).unwrap_or_else(|_| py.None()))
            .unwrap_or_else(|| py.None())
    }

    fn set_item(&mut self, key: String, value: Py<PyAny>) {
        self.data.write_arc().insert(key, value);
    }

    fn del_item(&mut self, key: String) {
        self.data.write_arc().remove(&key);
    }

    fn to_dict<'py>(&'py self, py: Python<'py>) -> Bound<'py, PyDict> {
        let dict = PyDict::new(py);
        for (k, v) in self.data.read_arc().iter() {
            dict.set_item(k, v).unwrap()
        }
        dict
    }
}

impl Display for PyModelMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_line_length = 80; // You can make this dynamic via fmt::Formatter if you want
        let mut current_line_length = 1; // account for initial '{'

        write!(f, "{{")?;

        let mut first = true;
        for (key, value) in self.data.read_arc().iter() {
            let item = format!("'{}': '{}'", key, value);

            if !first {
                current_line_length += 2; // account for ", "
            }

            // If adding the next item would exceed the line length, insert a line break
            if current_line_length + item.len() > max_line_length {
                write!(f, ",\n  {}", item)?;
                current_line_length = 2 + item.len(); // reset line length with indentation
            } else {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
                current_line_length += item.len();
            }

            first = false;
        }

        write!(f, "}}")
    }
}
