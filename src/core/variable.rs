#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyclass(eq, eq_int))]
#[derive(PartialEq, Clone)]
pub enum Vtype {
    Real,
    Integer,
    Binary,
    Spin,
}

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone)]
pub struct Bounds {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

impl Bounds {
    fn new(lower: Option<f64>, upper: Option<f64>) -> Self {
        Self { lower, upper }
    }
}

#[cfg(feature = "py")]
#[pymethods]
impl Bounds {
    #[new]
    #[pyo3(signature=(lower=None, upper=None))]
    fn py_new(lower: Option<f64>, upper: Option<f64>) -> PyResult<Self> {
        Ok(Bounds::new(lower, upper))
    }
}

pub struct Variable {
    // #[cfg_attr(feature = "py", pyo3(get))]
    pub name: String,
    // pub vtype: Vtype,
    // pub bounds: Bounds,
}

// #[cfg_attr(feature = "py", pyclass)]
// pub struct VariableRef {
//     pub id: i32,
// }
//
// impl VariableRef {
//     pub fn new(id: i32) -> Self {
//         Self { id }
//     }
// }

impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

// impl Add<f64> for Variable {
//     type Output = Expression;
//     fn add(self, rhs: f64) -> Self::Output {
//         let mut out = Expression::empty();
//         out += self;
//         out += rhs;
//         out
//     }
// }
//
// impl Add<f64> for &Variable {
//     type Output = Expression;
//     fn add(self, rhs: f64) -> Self::Output {
//         let mut out = Expression::empty();
//         out += self;
//         out += rhs;
//         out
//     }
// }
//
// impl Mul<f64> for Variable {
//     type Output = Expression;
//
//     fn mul(self, rhs: f64) -> Self::Output {
//         let mut out = Expression::empty();
//         out.linear.mul_var(&self, rhs);
//         out
//     }
// }
//
// impl Mul<f64> for &Variable {
//     type Output = Expression;
//
//     fn mul(self, rhs: f64) -> Self::Output {
//         let mut out = Expression::empty();
//         out.linear.mul_var(self, rhs);
//         out
//     }
// }
//
// #[cfg(feature = "py")]
// #[pymethods]
// impl Variable {
//     #[new]
//     fn new_ref() -> PyResult<VarRef> {
//         unimplemented!()
//     }
// }
// #[cfg(feature = "py")]
// #[pymethods]
// impl Variable {
//     #[new]
//     #[pyo3(signature=(name))]
//     fn py_new(name: String) -> PyResult<VarRef> {
//         Ok(Self::new(name, None))
//     }
//
//     // #[getter(name)]
//     // fn get_name(&self) -> PyResult<&String> {
//     //     Ok(&self.name)
//     // }
//
//     // fn __add__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
//     //     if let Ok(value) = other.extract::<f64>(py) {
//     //         let expr = self + value;
//     //         Ok(expr)
//     //     } else {
//     //         Err(PyRuntimeError::new_err("other type not recognized"))
//     //     }
//     // }
//
//     // fn __mul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
//     //     if let Ok(value) = other.extract::<f64>(py) {
//     //         let expr = self * value;
//     //         Ok(expr)
//     //     } else {
//     //         Err(PyRuntimeError::new_err("other type not recognized"))
//     //     }
//     // }
//
//     // fn __str__(&self) -> String {
//     //     format!("{}", self.name)
//     // }
// }
