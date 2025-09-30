use pyo3::prelude::*;
use pyo3::types::PyModule;
use uuid::Uuid;
use crate::types::PyUuid;

pub struct UuidConstants;

impl UuidConstants {
    pub fn register_constants(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("NAMESPACE_DNS", PyUuid::new(Uuid::NAMESPACE_DNS))?;
        m.add("NAMESPACE_URL", PyUuid::new(Uuid::NAMESPACE_URL))?;
        m.add("NAMESPACE_OID", PyUuid::new(Uuid::NAMESPACE_OID))?;
        m.add("NAMESPACE_X500", PyUuid::new(Uuid::NAMESPACE_X500))?;
        Ok(())
    }
}