use std::{cell::RefCell, fmt::Display, ops::Deref, rc::Rc};

use hashbrown::HashMap;
use pyo3::{prelude::*, types::PyDict};

#[pyclass(unsendable, subclass, name = "ModelMetadata", module = "aqmodels")]
#[derive(Clone, Debug)]
pub struct PyModelMetadata {
    pub data: Rc<RefCell<HashMap<String, PyObject>>>,
}

impl PyModelMetadata {
    pub fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(HashMap::new())),
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
        self.data.borrow().len()
    }

    fn __contains__(&self, key: String) -> bool {
        self.data.borrow().contains_key(&key)
    }

    fn __getitem__(&self, py: Python, key: String) -> PyObject {
        self.get_item(py, key)
    }

    fn __setitem__(&mut self, key: String, value: PyObject) {
        self.set_item(key, value);
    }

    fn __delitem__(&mut self, key: String) {
        self.del_item(key);
    }

    fn __str__(&self) -> String {
        format!("{self}")
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn get_item(&self, py: Python, key: String) -> PyObject {
        #[allow(deprecated)]
        self.data
            .borrow()
            .get(&key)
            .map(|v| v.into_py(py))
            .unwrap_or_else(|| py.None())
    }

    fn set_item(&mut self, key: String, value: PyObject) {
        self.data.borrow_mut().insert(key, value);
    }

    fn del_item(&mut self, key: String) {
        self.data.borrow_mut().remove(&key);
    }

    fn to_dict<'py>(&'py self, py: Python<'py>) -> Bound<'py, PyDict> {
        let dict = PyDict::new(py);
        for (k, v) in self.data.borrow().iter() {
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
        for (key, value) in self.data.borrow().iter() {
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
