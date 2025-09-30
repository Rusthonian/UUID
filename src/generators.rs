use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyBytesMethods;
use std::sync::OnceLock;
use uuid::Uuid;
use crate::types::PyUuid;

#[pyfunction]
pub fn new_v4() -> PyResult<PyUuid> {
    let uuid = Uuid::new_v4();
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn nil() -> PyResult<PyUuid> {
    Ok(PyUuid::new(Uuid::nil()))
}

#[pyfunction]
pub fn max() -> PyResult<PyUuid> {
    Ok(PyUuid::new(Uuid::max()))
}


#[pyfunction]
pub fn new_v3(namespace: &PyUuid, name: &str) -> PyResult<PyUuid> {
    let uuid = Uuid::new_v3(&namespace.inner, name.as_bytes());
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn new_v5(namespace: &PyUuid, name: &str) -> PyResult<PyUuid> {
    let uuid = Uuid::new_v5(&namespace.inner, name.as_bytes());
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn new_v8(bytes: &Bound<'_, PyBytes>) -> PyResult<PyUuid> {
    let slice = bytes.as_bytes();
    if slice.len() != 16 {
        return Err(pyo3::exceptions::PyValueError::new_err("UUID v8 requires exactly 16 bytes"));
    }
    let arr: [u8; 16] = slice.try_into().unwrap();
    let uuid = Uuid::new_v8(arr);
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn now_v7() -> PyResult<PyUuid> {
    let uuid = Uuid::now_v7();
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn new_v1(secs: u64, nanos: u32, node: (u8, u8, u8, u8, u8, u8)) -> PyResult<PyUuid> {
    use uuid::v1::{Context, Timestamp};

    static V1_CONTEXT: OnceLock<Context> = OnceLock::new();
    let context = V1_CONTEXT.get_or_init(|| Context::new(0));

    let node_id = [node.0, node.1, node.2, node.3, node.4, node.5];
    let ts = Timestamp::from_unix(context, secs, nanos);
    let uuid = Uuid::new_v1(ts, &node_id);
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn new_v6(secs: u64, nanos: u32, node: (u8, u8, u8, u8, u8, u8)) -> PyResult<PyUuid> {
    use uuid::v1::{Context, Timestamp};

    static V1_CONTEXT: OnceLock<Context> = OnceLock::new();
    let context = V1_CONTEXT.get_or_init(|| Context::new(0));

    let node_id = [node.0, node.1, node.2, node.3, node.4, node.5];
    let ts = Timestamp::from_unix(context, secs, nanos);
    let uuid = Uuid::new_v6(ts, &node_id);
    Ok(PyUuid::new(uuid))
}


/// uuid4() -> UUID
#[pyfunction(name = "uuid4")]
#[pyo3(text_signature = "()")]
pub fn py_uuid4() -> PyResult<PyUuid> { new_v4() }

#[pyfunction(name = "uuid7")]
#[pyo3(text_signature = "()")]
pub fn py_uuid7() -> PyResult<PyUuid> { now_v7() }

#[pyfunction(name = "uuid3")]
#[pyo3(signature = (namespace, name), text_signature = "(namespace, name)")]
pub fn py_uuid3(namespace: &PyUuid, name: &str) -> PyResult<PyUuid> { new_v3(namespace, name) }

#[pyfunction(name = "uuid5")]
#[pyo3(signature = (namespace, name), text_signature = "(namespace, name)")]
pub fn py_uuid5(namespace: &PyUuid, name: &str) -> PyResult<PyUuid> { new_v5(namespace, name) }

#[pyfunction(name = "uuid8")]
#[pyo3(signature = (bytes), text_signature = "(bytes)")]
pub fn py_uuid8(bytes: &Bound<'_, PyBytes>) -> PyResult<PyUuid> { new_v8(bytes) }

#[pyfunction(name = "uuid1")]
#[pyo3(signature = (node=None, clock_seq=None), text_signature = "(*, node=None, clock_seq=None)")]
pub fn py_uuid1(node: Option<(u8,u8,u8,u8,u8,u8)>, clock_seq: Option<u16>) -> PyResult<PyUuid> {
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::v1::{Context, Timestamp};

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs = now.as_secs();
    let nanos = now.subsec_nanos();
    let n = node.unwrap_or((0,0,0,0,0,0));
    let seq = (clock_seq.unwrap_or(0)) & 0x3FFF;

    let context = Context::new(seq);
    let node_id = [n.0, n.1, n.2, n.3, n.4, n.5];
    let ts = Timestamp::from_unix(&context, secs, nanos);
    let uuid = Uuid::new_v1(ts, &node_id);
    Ok(PyUuid::new(uuid))
}

#[pyfunction(name = "uuid6")]
#[pyo3(signature = (node=None, clock_seq=None), text_signature = "(*, node=None, clock_seq=None)")]
pub fn py_uuid6(node: Option<(u8,u8,u8,u8,u8,u8)>, clock_seq: Option<u16>) -> PyResult<PyUuid> {
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::v1::{Context, Timestamp};

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs = now.as_secs();
    let nanos = now.subsec_nanos();
    let n = node.unwrap_or((0,0,0,0,0,0));
    let seq = (clock_seq.unwrap_or(0)) & 0x3FFF;

    let context = Context::new(seq);
    let node_id = [n.0, n.1, n.2, n.3, n.4, n.5];
    let ts = Timestamp::from_unix(&context, secs, nanos);
    let uuid = Uuid::new_v6(ts, &node_id);
    Ok(PyUuid::new(uuid))
}


/// Generate v1 UUID with explicit timestamp
#[pyfunction]
pub fn new_v1_from_timestamp(timestamp: &crate::timestamp::PyTimestamp, node: (u8, u8, u8, u8, u8, u8)) -> PyResult<PyUuid> {
    let node_id = [node.0, node.1, node.2, node.3, node.4, node.5];
    let uuid = Uuid::new_v1(timestamp.inner, &node_id);
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn new_v6_from_timestamp(timestamp: &crate::timestamp::PyTimestamp, node: (u8, u8, u8, u8, u8, u8)) -> PyResult<PyUuid> {
    let node_id = [node.0, node.1, node.2, node.3, node.4, node.5];
    let uuid = Uuid::new_v6(timestamp.inner, &node_id);
    Ok(PyUuid::new(uuid))
}

#[pyfunction]
pub fn new_v7_from_timestamp(timestamp: &crate::timestamp::PyTimestamp) -> PyResult<PyUuid> {
    use uuid::timestamp::Timestamp as UuidTimestamp;
    use uuid::Uuid;
    
    let (secs, nanos) = timestamp.inner.to_unix();
    let ts = UuidTimestamp::from_unix(uuid::NoContext, secs, nanos);
    
    let uuid = Uuid::new_v7(ts);
    Ok(PyUuid::new(uuid))
}