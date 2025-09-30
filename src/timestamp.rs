use pyo3::prelude::*;
use uuid::v1::{Context, Timestamp};
use crate::types::PyUuid;

#[pyclass(name = "Timestamp")]
#[derive(Clone, Debug)]
pub struct PyTimestamp {
    pub(crate) inner: Timestamp,
}

impl PyTimestamp {
    pub fn new(timestamp: Timestamp) -> Self {
        PyTimestamp { inner: timestamp }
    }
}

#[pymethods]
impl PyTimestamp {
    #[new]
    #[pyo3(signature = (seconds, nanos, counter=0))]
    fn py_new(seconds: u64, nanos: u32, counter: u16) -> PyResult<Self> {
        let context = Context::new(counter);
        let ts = Timestamp::from_unix(&context, seconds, nanos);
        Ok(PyTimestamp::new(ts))
    }

    #[staticmethod]
    fn from_unix(seconds: u64, nanos: u32, counter: u16) -> PyResult<Self> {
        let context = Context::new(counter);
        let ts = Timestamp::from_unix(&context, seconds, nanos);
        Ok(PyTimestamp::new(ts))
    }

    #[staticmethod]
    fn from_gregorian(ticks: u64, counter: u16) -> PyResult<Self> {
        let ts = Timestamp::from_gregorian(ticks, counter);
        Ok(PyTimestamp::new(ts))
    }

    fn to_unix(&self) -> PyResult<(u64, u32)> {
        Ok(self.inner.to_unix())
    }

    fn to_gregorian(&self) -> PyResult<(u64, u16)> {
        Ok(self.inner.to_gregorian())
    }

    #[getter]
    fn seconds(&self) -> PyResult<u64> {
        Ok(self.inner.to_unix().0)
    }

    #[getter]
    fn nanos(&self) -> PyResult<u32> {
        Ok(self.inner.to_unix().1)
    }

    #[getter]
    fn counter(&self) -> PyResult<u16> {
        Ok(self.inner.to_gregorian().1)
    }

    fn __str__(&self) -> PyResult<String> {
        let (secs, nanos) = self.inner.to_unix();
        Ok(format!("Timestamp(seconds={}, nanos={})", secs, nanos))
    }

    fn __repr__(&self) -> PyResult<String> {
        let (secs, nanos) = self.inner.to_unix();
        Ok(format!("Timestamp(seconds={}, nanos={})", secs, nanos))
    }

    fn __eq__(&self, other: &PyTimestamp) -> PyResult<bool> {
        Ok(self.inner.to_gregorian() == other.inner.to_gregorian())
    }
}

#[pyclass(name = "Context")]
#[derive(Debug)]
pub struct PyContext {
    pub(crate) inner: Context,
}

impl PyContext {
    pub fn new(context: Context) -> Self {
        PyContext { inner: context }
    }
}

#[pymethods]
impl PyContext {
    #[new]
    #[pyo3(signature = (counter))]
    fn py_new(counter: u16) -> PyResult<Self> {
        let ctx = Context::new(counter);
        Ok(PyContext::new(ctx))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Context()"))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Context()"))
    }
}

impl PyUuid {
    pub fn get_timestamp_inner(&self) -> Option<Timestamp> {
        self.inner.get_timestamp()
    }
}