use pyo3::prelude::*;

pub struct UuidErrors;

impl UuidErrors {
    pub fn value_error(message: &str) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(message.to_string())
    }

    pub fn type_error(message: &str) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyTypeError, _>(message.to_string())
    }
}

