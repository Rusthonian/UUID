use uuid::Uuid;
use crate::types::PyUuid;

pub struct UuidUtils;

impl UuidUtils {
    pub fn to_py_uuid(uuid: Uuid) -> PyUuid {
        PyUuid::new(uuid)
    }

    pub fn as_inner(py_uuid: &PyUuid) -> &Uuid {
        &py_uuid.inner
    }
}

