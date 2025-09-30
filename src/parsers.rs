use pyo3::prelude::*;
use std::str::FromStr;
use uuid::Uuid;
use crate::types::PyUuid;

#[pyfunction]
pub fn parse_str(s: &str) -> PyResult<PyUuid> {
    let uuid = Uuid::from_str(s.trim())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn is_valid(s: &str) -> PyResult<bool> {
    Ok(Uuid::from_str(s).is_ok())
}

