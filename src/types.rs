use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyBytesMethods;
use std::convert::TryInto;
use std::str::FromStr;
use uuid::Uuid;

/// Python wrapper for Rust UUID
#[pyclass(name = "UUID")]
#[derive(Clone, Debug)]
pub struct PyUuid {
    pub(crate) inner: Uuid,
}

impl PyUuid {
    pub fn new(uuid: Uuid) -> Self {
        PyUuid { inner: uuid }
    }
}

#[pymethods]
impl PyUuid {
    #[new]
    #[pyo3(signature = (hex = None, bytes = None, bytes_le = None, fields = None, int = None, version = None))]
    fn py_new(
        hex: Option<&str>,
        bytes: Option<&Bound<'_, PyBytes>>,
        bytes_le: Option<&Bound<'_, PyBytes>>,
        fields: Option<(u32, u16, u16, u8, u8, u64)>,
        int: Option<u128>,
        version: Option<u8>,
    ) -> PyResult<Self> {
        let mut provided = 0;
        if hex.is_some() { provided += 1; }
        if bytes.is_some() { provided += 1; }
        if bytes_le.is_some() { provided += 1; }
        if fields.is_some() { provided += 1; }
        if int.is_some() { provided += 1; }
        if provided != 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("UUID() expects exactly one of hex, bytes, bytes_le, fields, or int"));
        }
        if let Some(s) = hex {
            let mut s = s.trim();
            if let Some(stripped) = s.strip_prefix("urn:uuid:") { s = stripped; }
            s = s.trim_matches(|c| c == '{' || c == '}');
            let uuid = Uuid::from_str(s).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
            return Ok(PyUuid::new(uuid));
        }
        if let Some(b) = bytes {
            let slice = b.as_bytes();
            if slice.len() != 16 { return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("UUID bytes must be exactly 16 bytes")); }
            let uuid = Uuid::from_bytes(slice.try_into().unwrap());
            return Ok(PyUuid::new(uuid));
        }
        if let Some(b) = bytes_le {
            let slice = b.as_bytes();
            if slice.len() != 16 { return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("UUID bytes_le must be exactly 16 bytes")); }
            let mut arr: [u8; 16] = slice.try_into().unwrap();
            arr[0..4].reverse(); arr[4..6].reverse(); arr[6..8].reverse();
            if let Some(v) = version {
                let mut thv = u16::from_be_bytes([arr[6], arr[7]]);
                thv = (thv & 0x0FFF) | ((v as u16) << 12);
                let be = thv.to_be_bytes();
                arr[6] = be[0]; arr[7] = be[1];
            }
            let uuid = Uuid::from_bytes(arr);
            return Ok(PyUuid::new(uuid));
        }
        if let Some((time_low, time_mid, mut time_hi_version, clock_seq_hi, clock_seq_low, node)) = fields {
            if let Some(v) = version { time_hi_version = (time_hi_version & 0x0FFF) | ((v as u16) << 12); }
            let mut arr = [0u8; 16];
            arr[..4].copy_from_slice(&time_low.to_be_bytes());
            arr[4..6].copy_from_slice(&time_mid.to_be_bytes());
            arr[6..8].copy_from_slice(&time_hi_version.to_be_bytes());
            arr[8] = clock_seq_hi; arr[9] = clock_seq_low;
            for i in 0..6 { arr[10 + i] = ((node >> (8 * (5 - i))) & 0xFF) as u8; }
            let uuid = Uuid::from_bytes(arr);
            return Ok(PyUuid::new(uuid));
        }
        if let Some(value) = int {
            let mut uuid = Uuid::from_u128(value);
            if let Some(v) = version {
                let mut b = *uuid.as_bytes();
                let mut thv = u16::from_be_bytes([b[6], b[7]]);
                thv = (thv & 0x0FFF) | ((v as u16) << 12);
                let be = thv.to_be_bytes();
                b[6] = be[0]; b[7] = be[1];
                uuid = Uuid::from_bytes(b);
            }
            return Ok(PyUuid::new(uuid));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid UUID() arguments"))
    }

    #[staticmethod]
    fn from_bytes(bytes: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let bytes_slice = bytes.as_bytes();
        if bytes_slice.len() != 16 { return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("UUID must be exactly 16 bytes")); }
        let uuid = Uuid::from_bytes(bytes_slice.try_into().unwrap());
        Ok(PyUuid::new(uuid))
    }

    #[staticmethod]
    fn from_bytes_le(bytes: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let slice = bytes.as_bytes();
        if slice.len() != 16 { return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("UUID must be exactly 16 bytes")); }
        let mut b: [u8; 16] = slice.try_into().unwrap();
        b[0..4].reverse(); b[4..6].reverse(); b[6..8].reverse();
        let uuid = Uuid::from_bytes(b);
        Ok(PyUuid::new(uuid))
    }

    #[staticmethod]
    fn from_u128(value: u128) -> Self { PyUuid::new(Uuid::from_u128(value)) }

    #[staticmethod]
    fn from_u64_pair(high: u64, low: u64) -> Self { PyUuid::new(Uuid::from_u64_pair(high, low)) }

    #[staticmethod]
    fn from_int(value: u128) -> Self { PyUuid::new(Uuid::from_u128(value)) }

    fn __str__(&self) -> PyResult<String> { Ok(self.inner.to_string()) }
    fn __repr__(&self) -> PyResult<String> { Ok(format!("UUID('{}')", self.inner)) }

    #[getter]
    fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new_bound(py, self.inner.as_bytes()))
    }

    #[getter]
    fn bytes_le<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let mut b = *self.inner.as_bytes();
        b[0..4].reverse(); b[4..6].reverse(); b[6..8].reverse();
        Ok(PyBytes::new_bound(py, &b))
    }

    fn as_u128(&self) -> PyResult<u128> { Ok(self.inner.as_u128()) }

    #[getter]
    fn int(&self) -> PyResult<u128> { Ok(self.inner.as_u128()) }

    fn as_u64_pair(&self) -> PyResult<(u64, u64)> { Ok(self.inner.as_u64_pair()) }

    #[getter]
    fn fields(&self) -> PyResult<(u32, u16, u16, u8, u8, u64)> {
        let (time_low, time_mid, time_hi_and_version, rest) = self.inner.as_fields();
        let clock_seq_hi_variant = rest[0];
        let clock_seq_low = rest[1];
        let mut node_val: u64 = 0;
        for &byte in &rest[2..8] { node_val = (node_val << 8) | byte as u64; }
        Ok((time_low, time_mid, time_hi_and_version, clock_seq_hi_variant, clock_seq_low, node_val))
    }

    fn time_low(&self) -> PyResult<u32> { Ok(self.inner.as_fields().0) }
    fn time_mid(&self) -> PyResult<u16> { Ok(self.inner.as_fields().1) }
    fn time_hi_version(&self) -> PyResult<u16> { Ok(self.inner.as_fields().2) }

    fn clock_seq(&self) -> PyResult<u16> {
        let (_, _, _, rest) = self.inner.as_fields();
        let hi = (rest[0] & 0b0011_1111) as u16;
        let lo = rest[1] as u16;
        Ok((hi << 8) | lo)
    }

    fn node(&self) -> PyResult<u64> {
        let (_, _, _, rest) = self.inner.as_fields();
        let mut node_val: u64 = 0;
        for &byte in &rest[2..8] { node_val = (node_val << 8) | byte as u64; }
        Ok(node_val)
    }

    #[getter]
    fn version(&self) -> PyResult<Option<u8>> {
        use uuid::Version;
        Ok(match self.inner.get_version() {
            Some(Version::Mac) => Some(1),
            Some(Version::Md5) => Some(3),
            Some(Version::Random) => Some(4),
            Some(Version::Sha1) => Some(5),
            Some(Version::SortMac) => Some(6),
            Some(Version::SortRand) => Some(7),
            Some(Version::Custom) => Some(8),
            Some(_) => None,
            None => None,
        })
    }

    #[getter]
    fn variant(&self) -> PyResult<&'static str> {
        Ok(match self.inner.get_variant() {
            uuid::Variant::NCS => "NCS",
            uuid::Variant::RFC4122 => "RFC4122",
            uuid::Variant::Microsoft => "Microsoft",
            uuid::Variant::Future => "Future",
            _ => "Unknown",
        })
    }

    #[getter]
    fn hex(&self) -> PyResult<String> { Ok(self.inner.simple().to_string()) }

    #[getter]
    fn urn(&self) -> PyResult<String> { Ok(self.inner.urn().to_string()) }

    fn __int__(&self) -> PyResult<u128> { Ok(self.inner.as_u128()) }

    fn __bytes__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new_bound(py, self.inner.as_bytes()))
    }

    fn is_nil(&self) -> PyResult<bool> { Ok(self.inner.is_nil()) }
    fn is_max(&self) -> PyResult<bool> { Ok(self.inner.is_max()) }

    fn __eq__(&self, other: &PyUuid) -> PyResult<bool> { Ok(self.inner == other.inner) }
    fn __lt__(&self, other: &PyUuid) -> PyResult<bool> { Ok(self.inner < other.inner) }
    fn __le__(&self, other: &PyUuid) -> PyResult<bool> { Ok(self.inner <= other.inner) }
    fn __gt__(&self, other: &PyUuid) -> PyResult<bool> { Ok(self.inner > other.inner) }
    fn __ge__(&self, other: &PyUuid) -> PyResult<bool> { Ok(self.inner >= other.inner) }

    fn __hash__(&self) -> PyResult<u64> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish())
    }

    fn as_hyphenated(&self) -> PyResult<String> { Ok(self.inner.hyphenated().to_string()) }
    fn as_simple(&self) -> PyResult<String> { Ok(self.inner.simple().to_string()) }
    fn as_braced(&self) -> PyResult<String> { Ok(self.inner.braced().to_string()) }
    fn as_urn(&self) -> PyResult<String> { Ok(self.inner.urn().to_string()) }

    fn encode_hyphenated(&self) -> PyResult<String> {
        let mut buf = [0u8; 36];
        let s = self.inner.hyphenated().encode_lower(&mut buf);
        Ok(s.to_string())
    }

    fn encode_simple(&self) -> PyResult<String> {
        let mut buf = [0u8; 32];
        let s = self.inner.simple().encode_lower(&mut buf);
        Ok(s.to_string())
    }

    fn encode_braced(&self) -> PyResult<String> {
        let mut buf = [0u8; 38];
        let s = self.inner.braced().encode_lower(&mut buf);
        Ok(s.to_string())
    }

    fn encode_urn(&self) -> PyResult<String> {
        let mut buf = [0u8; 45];
        let s = self.inner.urn().encode_lower(&mut buf);
        Ok(s.to_string())
    }

    fn encode_hyphenated_upper(&self) -> PyResult<String> {
        let mut buf = [0u8; 36];
        let s = self.inner.hyphenated().encode_upper(&mut buf);
        Ok(s.to_string())
    }

    fn encode_simple_upper(&self) -> PyResult<String> {
        let mut buf = [0u8; 32];
        let s = self.inner.simple().encode_upper(&mut buf);
        Ok(s.to_string())
    }

    fn encode_braced_upper(&self) -> PyResult<String> {
        let mut buf = [0u8; 38];
        let s = self.inner.braced().encode_upper(&mut buf);
        Ok(s.to_string())
    }

    fn encode_urn_upper(&self) -> PyResult<String> {
        let mut buf = [0u8; 45];
        let s = self.inner.urn().encode_upper(&mut buf);
        Ok(s.to_string())
    }

    fn to_bytes_le<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let mut b = *self.inner.as_bytes();
        b[0..4].reverse(); b[4..6].reverse(); b[6..8].reverse();
        Ok(PyBytes::new_bound(py, &b))
    }

    #[staticmethod]
    fn from_slice(bytes: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let slice = bytes.as_bytes();
        let uuid = Uuid::from_slice(slice).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyUuid::new(uuid))
    }

    #[staticmethod]
    fn from_slice_le(bytes: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let slice = bytes.as_bytes();
        let uuid = Uuid::from_slice_le(slice).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyUuid::new(uuid))
    }

    #[staticmethod]
    fn from_fields(time_low: u32, time_mid: u16, time_hi_and_version: u16, clock_seq_hi_and_reserved: u8, clock_seq_low: u8, node: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let node_slice = node.as_bytes();
        if node_slice.len() != 6 { return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Node must be exactly 6 bytes")); }
        let node_arr: [u8; 6] = node_slice.try_into().unwrap();
        let uuid = Uuid::from_fields(time_low, time_mid, time_hi_and_version, &[clock_seq_hi_and_reserved, clock_seq_low, node_arr[0], node_arr[1], node_arr[2], node_arr[3], node_arr[4], node_arr[5]]);
        Ok(PyUuid::new(uuid))
    }

    #[staticmethod]
    fn from_fields_le(time_low: u32, time_mid: u16, time_hi_and_version: u16, clock_seq_hi_and_reserved: u8, clock_seq_low: u8, node: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let node_slice = node.as_bytes();
        if node_slice.len() != 6 { return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Node must be exactly 6 bytes")); }
        let node_arr: [u8; 6] = node_slice.try_into().unwrap();
        let uuid = Uuid::from_fields_le(time_low, time_mid, time_hi_and_version, &[clock_seq_hi_and_reserved, clock_seq_low, node_arr[0], node_arr[1], node_arr[2], node_arr[3], node_arr[4], node_arr[5]]);
        Ok(PyUuid::new(uuid))
    }

    fn to_fields_le(&self) -> PyResult<(u32, u16, u16, Vec<u8>)> {
        let (d1, d2, d3, d4) = self.inner.to_fields_le();
        Ok((d1, d2, d3, d4.to_vec()))
    }

    fn to_u128_le(&self) -> PyResult<u128> { Ok(self.inner.to_u128_le()) }

    #[staticmethod]
    fn from_u128_le(value: u128) -> Self { PyUuid::new(Uuid::from_u128_le(value)) }

    fn get_timestamp(&self) -> PyResult<Option<(u64, u32)>> {
        if let Some(ts) = self.inner.get_timestamp() {
            let (secs, nanos) = ts.to_unix();
            Ok(Some((secs, nanos)))
        } else {
            Ok(None)
        }
    }

    fn get_timestamp_dict(&self, py: Python) -> PyResult<Option<PyObject>> {
        if let Some(ts) = self.inner.get_timestamp() {
            let (secs, nanos) = ts.to_unix();
            let (_, counter) = ts.to_gregorian();
            
            let dict = pyo3::types::PyDict::new_bound(py);
            dict.set_item("seconds", secs)?;
            dict.set_item("nanos", nanos)?;
            dict.set_item("counter", counter)?;
            Ok(Some(dict.to_object(py)))
        } else {
            Ok(None)
        }
    }

    fn has_timestamp(&self) -> PyResult<bool> { Ok(self.inner.get_timestamp().is_some()) }
}
