use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyBytesMethods;
use uuid::{Builder, Uuid, Version, Variant};
use crate::types::PyUuid;
use std::convert::TryInto;

#[pyclass(name = "Builder")]
#[derive(Clone, Debug)]
pub struct PyBuilder {
    bytes: [u8; 16],
}

#[pymethods]
impl PyBuilder {
    #[new]
    fn py_new() -> Self {
        PyBuilder {
            bytes: [0u8; 16],
        }
    }

    #[staticmethod]
    fn from_bytes(bytes: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let slice = bytes.as_bytes();
        if slice.len() != 16 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Builder requires exactly 16 bytes",
            ));
        }
        Ok(PyBuilder {
            bytes: slice.try_into().unwrap(),
        })
    }

    #[staticmethod]
    fn from_fields(
        d1: u32,
        d2: u16,
        d3: u16,
        d4: &Bound<'_, PyBytes>,
    ) -> PyResult<Self> {
        let d4_slice = d4.as_bytes();
        if d4_slice.len() != 8 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "d4 field must be exactly 8 bytes",
            ));
        }
        let d4_arr: [u8; 8] = d4_slice.try_into().unwrap();
        let builder = Builder::from_fields(d1, d2, d3, &d4_arr);
        let uuid = builder.into_uuid();
        Ok(PyBuilder {
            bytes: *uuid.as_bytes(),
        })
    }

    #[staticmethod]
    fn from_fields_le(
        d1: u32,
        d2: u16,
        d3: u16,
        d4: &Bound<'_, PyBytes>,
    ) -> PyResult<Self> {
        let d4_slice = d4.as_bytes();
        if d4_slice.len() != 8 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "d4 field must be exactly 8 bytes",
            ));
        }
        let d4_arr: [u8; 8] = d4_slice.try_into().unwrap();
        let builder = Builder::from_fields_le(d1, d2, d3, &d4_arr);
        let uuid = builder.into_uuid();
        Ok(PyBuilder {
            bytes: *uuid.as_bytes(),
        })
    }

    #[staticmethod]
    fn from_u128(value: u128) -> Self {
        let builder = Builder::from_u128(value);
        let uuid = builder.into_uuid();
        PyBuilder {
            bytes: *uuid.as_bytes(),
        }
    }

    #[staticmethod]
    fn from_u128_le(value: u128) -> Self {
        let builder = Builder::from_u128_le(value);
        let uuid = builder.into_uuid();
        PyBuilder {
            bytes: *uuid.as_bytes(),
        }
    }

    #[staticmethod]
    fn from_random_bytes(bytes: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let slice = bytes.as_bytes();
        if slice.len() != 16 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Random bytes must be exactly 16 bytes",
            ));
        }
        let arr: [u8; 16] = slice.try_into().unwrap();
        let builder = Builder::from_random_bytes(arr);
        let uuid = builder.into_uuid();
        Ok(PyBuilder {
            bytes: *uuid.as_bytes(),
        })
    }

    #[staticmethod]
    fn from_unix_timestamp(seconds: u64, nanos: u32, random_bytes: &Bound<'_, PyBytes>) -> PyResult<Self> {
        let slice = random_bytes.as_bytes();
        if slice.len() != 10 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Random bytes must be exactly 10 bytes for v7",
            ));
        }
        let arr: [u8; 10] = slice.try_into().unwrap();
        
        use uuid::timestamp::Timestamp as UuidTimestamp;
        let ts = UuidTimestamp::from_unix(uuid::NoContext, seconds, nanos);
        let builder = Builder::from_unix_timestamp_millis(ts.to_unix().0, &arr);
        let uuid = builder.into_uuid();
        
        Ok(PyBuilder {
            bytes: *uuid.as_bytes(),
        })
    }

    fn with_variant(&mut self, variant: &str) -> PyResult<()> {
        let var = match variant {
            "NCS" => Variant::NCS,
            "RFC4122" => Variant::RFC4122,
            "Microsoft" => Variant::Microsoft,
            "Future" => Variant::Future,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid variant. Must be one of: NCS, RFC4122, Microsoft, Future"
            )),
        };
        
        let mut builder = Builder::from_bytes(self.bytes);
        builder = builder.with_variant(var);
        let uuid = builder.into_uuid();
        self.bytes = *uuid.as_bytes();
        Ok(())
    }

    fn with_version(&mut self, version: u8) -> PyResult<()> {
        let ver = match version {
            1 => Version::Mac,
            3 => Version::Md5,
            4 => Version::Random,
            5 => Version::Sha1,
            6 => Version::SortMac,
            7 => Version::SortRand,
            8 => Version::Custom,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid version. Must be 1, 3, 4, 5, 6, 7, or 8"
            )),
        };
        
        let mut builder = Builder::from_bytes(self.bytes);
        builder = builder.with_version(ver);
        let uuid = builder.into_uuid();
        self.bytes = *uuid.as_bytes();
        Ok(())
    }

    fn build(&self) -> PyResult<PyUuid> {
        let builder = Builder::from_bytes(self.bytes);
        Ok(PyUuid::new(builder.into_uuid()))
    }

    fn as_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.bytes.to_vec())
    }

    fn set_byte(&mut self, index: usize, value: u8) -> PyResult<()> {
        if index >= 16 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Index out of range. Must be 0-15"
            ));
        }
        self.bytes[index] = value;
        Ok(())
    }

    fn get_byte(&self, index: usize) -> PyResult<u8> {
        if index >= 16 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Index out of range. Must be 0-15"
            ));
        }
        Ok(self.bytes[index])
    }

    fn __str__(&self) -> PyResult<String> {
        let uuid = Uuid::from_bytes(self.bytes);
        Ok(format!("Builder({})", uuid))
    }

    fn __repr__(&self) -> PyResult<String> {
        let uuid = Uuid::from_bytes(self.bytes);
        Ok(format!("Builder('{}')", uuid))
    }
}