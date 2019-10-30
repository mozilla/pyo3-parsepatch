use parsepatch::FileOp;
use pyo3::types::PyDict;
use pyo3::types::{PyBytes, PyByteArray, PyString};
use pyo3::prelude::*;
use pyo3::Python;

pub(crate) enum Bytes<'a> {
    Slice(&'a [u8]),
    Vec(Vec<u8>),
}

#[inline(always)]
pub fn set_info(
    diff: &PyDict,
    old_name: &str,
    new_name: &str,
    op: FileOp,
    binary: bool,
    py: &Python,
) {
    match op {
        FileOp::New => {
            diff.set_item("new", true).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("copied_from", py.None()).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
            diff.set_item("filename", new_name).unwrap();
        }
        FileOp::Deleted => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", true).unwrap();
            diff.set_item("copied_from", py.None()).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
            diff.set_item("filename", old_name).unwrap();
        }
        FileOp::Renamed => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("copied_from", py.None()).unwrap();
            diff.set_item("renamed_from", old_name).unwrap();
            diff.set_item("filename", new_name).unwrap();
        },
        FileOp::Copied => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("copied_from", old_name).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
            diff.set_item("filename", new_name).unwrap();
        }
        FileOp::None => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("copied_from", py.None()).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
            diff.set_item("filename", new_name).unwrap();
        }
    }
    diff.set_item("binary", binary).unwrap();
}

#[inline(always)]
pub(crate) fn get_bytes<'a>(py: Python, bytes: &'a PyObject) -> Option<Bytes<'a>> {
    if let Ok(bytes) = PyBytes::try_from(bytes.as_ref(py)) {
        Some(Bytes::Slice(bytes.as_bytes()))
    } else if let Ok(bytes) = PyString::try_from(bytes.as_ref(py)) {
        Some(Bytes::Slice(bytes.as_bytes().unwrap()))
    } else if let Ok(bytes) = PyByteArray::try_from(bytes.as_ref(py)) {
        Some(Bytes::Vec(bytes.to_vec()))
    } else {
        None
    }
}
