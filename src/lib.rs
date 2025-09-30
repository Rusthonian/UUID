use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod builder;
pub mod constants;
pub mod errors;
pub mod generators;
pub mod parsers;
pub mod timestamp;
pub mod types;
pub mod utils;

use crate::builder::PyBuilder;
use crate::constants::UuidConstants;
use crate::generators::{
    nil, max, new_v1, new_v3, new_v4, new_v5, new_v6, new_v8, now_v7,
    new_v1_from_timestamp, new_v6_from_timestamp, new_v7_from_timestamp,
    py_uuid1, py_uuid3, py_uuid4, py_uuid5, py_uuid6, py_uuid7, py_uuid8,
};
use crate::parsers::{is_valid, parse_str};
use crate::timestamp::{PyContext, PyTimestamp};
use crate::types::PyUuid;

#[pymodule]
fn rusthonian_uuid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUuid>()?;
    m.add_class::<PyBuilder>()?;
    m.add_class::<PyTimestamp>()?;
    m.add_class::<PyContext>()?;

    m.add_function(wrap_pyfunction!(py_uuid1, m)?)?;
    m.add_function(wrap_pyfunction!(py_uuid3, m)?)?;
    m.add_function(wrap_pyfunction!(py_uuid4, m)?)?;
    m.add_function(wrap_pyfunction!(py_uuid5, m)?)?;
    m.add_function(wrap_pyfunction!(py_uuid6, m)?)?;
    m.add_function(wrap_pyfunction!(py_uuid7, m)?)?;
    m.add_function(wrap_pyfunction!(py_uuid8, m)?)?;

    m.add_function(wrap_pyfunction!(new_v1, m)?)?;
    m.add_function(wrap_pyfunction!(new_v3, m)?)?;
    m.add_function(wrap_pyfunction!(new_v4, m)?)?;
    m.add_function(wrap_pyfunction!(new_v5, m)?)?;
    m.add_function(wrap_pyfunction!(new_v6, m)?)?;
    m.add_function(wrap_pyfunction!(new_v8, m)?)?;
    m.add_function(wrap_pyfunction!(now_v7, m)?)?;

    m.add_function(wrap_pyfunction!(new_v1_from_timestamp, m)?)?;
    m.add_function(wrap_pyfunction!(new_v6_from_timestamp, m)?)?;
    m.add_function(wrap_pyfunction!(new_v7_from_timestamp, m)?)?;

    m.add_function(wrap_pyfunction!(nil, m)?)?;
    m.add_function(wrap_pyfunction!(max, m)?)?;

    m.add_function(wrap_pyfunction!(parse_str, m)?)?;
    m.add_function(wrap_pyfunction!(is_valid, m)?)?;

    let py = m.py();
    UuidConstants::register_constants(py, m)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "Complete Python bindings for the Rust uuid crate")?;

    Ok(())
}

pub fn setup_uuid_module(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    rusthonian_uuid(m)
}