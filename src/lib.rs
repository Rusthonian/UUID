use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;
use std::str::FromStr;
use uuid::Uuid;

/// UUID submodule for Rusthonian
#[pymodule]
fn uuid_module(_py: Python, m: &PyModule) -> PyResult<()> {
    // Register the UUID class
    m.add_class::<PyUuid>()?;
    
    // Register utility functions
    m.add_function(wrap_pyfunction!(new_v4, m)?)?;
    m.add_function(wrap_pyfunction!(parse_str, m)?)?;
    m.add_function(wrap_pyfunction!(nil, m)?)?;
    m.add_function(wrap_pyfunction!(max, m)?)?;
    m.add_function(wrap_pyfunction!(is_valid, m)?)?;
    
    // Register constants
    m.add("NAMESPACE_DNS", PyUuid::new(Uuid::NAMESPACE_DNS))?;
    m.add("NAMESPACE_URL", PyUuid::new(Uuid::NAMESPACE_URL))?;
    m.add("NAMESPACE_OID", PyUuid::new(Uuid::NAMESPACE_OID))?;
    m.add("NAMESPACE_X500", PyUuid::new(Uuid::NAMESPACE_X500))?;
    
    Ok(())
}

/// Public function to setup UUID module (for use by main crate)
pub fn setup_uuid_module(_py: Python, m: &PyModule) -> PyResult<()> {
    uuid_module(_py, m)
}

/// Python wrapper for Rust UUID
#[pyclass(name = "UUID")]
#[derive(Clone, Debug)]
pub struct PyUuid {
    inner: Uuid,
}

impl PyUuid {
    fn new(uuid: Uuid) -> Self {
        PyUuid { inner: uuid }
    }
}

#[pymethods]
impl PyUuid {
    /// Create a new UUID from a string
    #[new]
    fn new_from_str(s: &str) -> PyResult<Self> {
        let uuid = Uuid::from_str(s)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyUuid::new(uuid))
    }
    
    /// Create a new UUID from bytes
    #[staticmethod]
    fn from_bytes(bytes: &PyBytes) -> PyResult<Self> {
        let bytes_slice = bytes.as_bytes();
        if bytes_slice.len() != 16 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "UUID must be exactly 16 bytes"
            ));
        }
        let uuid = Uuid::from_bytes(bytes_slice.try_into().unwrap());
        Ok(PyUuid::new(uuid))
    }
    
    /// Create a new UUID from u128
    #[staticmethod]
    fn from_u128(value: u128) -> Self {
        PyUuid::new(Uuid::from_u128(value))
    }
    
    /// Create a new UUID from u64 pair (high, low)
    #[staticmethod]
    fn from_u64_pair(high: u64, low: u64) -> Self {
        PyUuid::new(Uuid::from_u64_pair(high, low))
    }
    
    /// Get UUID as string
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
    
    /// Get UUID as string (repr)
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("UUID('{}')", self.inner))
    }
    
    /// Get UUID as bytes
    fn bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.as_bytes().to_vec())
    }
    
    /// Get UUID as u128
    fn as_u128(&self) -> PyResult<u128> {
        Ok(self.inner.as_u128())
    }
    
    /// Get UUID as u64 pair (high, low)
    fn as_u64_pair(&self) -> PyResult<(u64, u64)> {
        Ok(self.inner.as_u64_pair())
    }
    
    /// Get UUID version
    fn version(&self) -> PyResult<Option<u8>> {
        // TODO: Implement proper version extraction
        Ok(None)
    }
    
    /// Get UUID variant
    fn variant(&self) -> PyResult<&'static str> {
        match self.inner.get_variant() {
            uuid::Variant::NCS => Ok("NCS"),
            uuid::Variant::RFC4122 => Ok("RFC4122"),
            uuid::Variant::Microsoft => Ok("Microsoft"),
            uuid::Variant::Future => Ok("Future"),
            _ => Ok("Unknown"),
        }
    }
    
    /// Check if UUID is nil
    fn is_nil(&self) -> PyResult<bool> {
        Ok(self.inner.is_nil())
    }
    
    /// Check if UUID is max
    fn is_max(&self) -> PyResult<bool> {
        Ok(self.inner.is_max())
    }
    
    /// Compare UUIDs
    fn __eq__(&self, other: &PyUuid) -> PyResult<bool> {
        Ok(self.inner == other.inner)
    }
    
    fn __lt__(&self, other: &PyUuid) -> PyResult<bool> {
        Ok(self.inner < other.inner)
    }
    
    fn __le__(&self, other: &PyUuid) -> PyResult<bool> {
        Ok(self.inner <= other.inner)
    }
    
    fn __gt__(&self, other: &PyUuid) -> PyResult<bool> {
        Ok(self.inner > other.inner)
    }
    
    fn __ge__(&self, other: &PyUuid) -> PyResult<bool> {
        Ok(self.inner >= other.inner)
    }
    
    /// Hash implementation
    fn __hash__(&self) -> PyResult<u64> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish())
    }
}

/// Generate a new v4 UUID
#[pyfunction]
fn new_v4() -> PyResult<PyUuid> {
    let uuid = Uuid::new_v4();
    Ok(PyUuid::new(uuid))
}

/// Parse UUID from string
#[pyfunction]
fn parse_str(s: &str) -> PyResult<PyUuid> {
    let uuid = Uuid::from_str(s)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(PyUuid::new(uuid))
}

/// Get nil UUID
#[pyfunction]
fn nil() -> PyResult<PyUuid> {
    Ok(PyUuid::new(Uuid::nil()))
}

/// Get max UUID
#[pyfunction]
fn max() -> PyResult<PyUuid> {
    Ok(PyUuid::new(Uuid::max()))
}

/// Check if string is valid UUID
#[pyfunction]
fn is_valid(s: &str) -> PyResult<bool> {
    Ok(Uuid::from_str(s).is_ok())
}
