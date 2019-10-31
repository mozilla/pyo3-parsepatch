use parsepatch::{BinaryHunk, FileMode, FileOp};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::{PyByteArray, PyBytes, PyString, PyTuple};
use pyo3::Python;

pub(crate) enum Bytes<'a> {
    Slice(&'a [u8]),
    Vec(Vec<u8>),
}

#[inline(always)]
pub fn create_mode(old: Option<u32>, new: Option<u32>, py: &Python) -> PyObject {
    let dict = PyDict::new(*py);
    if let Some(old) = old {
        dict.set_item("old", old).unwrap();
    }
    if let Some(new) = new {
        dict.set_item("new", new).unwrap();
    }
    dict.to_object(*py)
}

#[inline(always)]
pub fn create_file_mode(modes: Option<FileMode>, py: &Python) -> PyObject {
    let dict = PyDict::new(*py);
    if let Some(modes) = modes {
        dict.set_item("old", modes.old).unwrap();
        dict.set_item("new", modes.new).unwrap();
    }
    dict.to_object(*py)
}

#[inline(always)]
pub fn create_bin_size(h: BinaryHunk, py: &Python) -> PyObject {
    let x = match h {
        BinaryHunk::Literal(s) => ("literal", s),
        BinaryHunk::Delta(s) => ("delta", s),
    };
    PyTuple::new(*py, &[x.0.to_object(*py), x.1.to_object(*py)]).to_object(*py)
}

#[inline(always)]
pub fn set_info(
    diff: &PyDict,
    old_name: &str,
    new_name: &str,
    op: FileOp,
    binary_sizes: Option<Vec<BinaryHunk>>,
    file_mode: Option<FileMode>,
    py: &Python,
) {
    match op {
        FileOp::New(m) => {
            diff.set_item("new", true).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("copied_from", py.None()).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
            diff.set_item("filename", new_name).unwrap();
            diff.set_item("modes", create_mode(None, Some(m), py))
                .unwrap();
        }
        FileOp::Deleted(m) => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", true).unwrap();
            diff.set_item("copied_from", py.None()).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
            diff.set_item("filename", old_name).unwrap();
            diff.set_item("modes", create_mode(Some(m), None, py))
                .unwrap();
        }
        FileOp::Renamed => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("copied_from", py.None()).unwrap();
            diff.set_item("renamed_from", old_name).unwrap();
            diff.set_item("filename", new_name).unwrap();
            diff.set_item("modes", create_file_mode(file_mode, py))
                .unwrap();
        }
        FileOp::Copied => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("copied_from", old_name).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
            diff.set_item("filename", new_name).unwrap();
            diff.set_item("modes", create_file_mode(file_mode, py))
                .unwrap();
        }
        FileOp::None => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("copied_from", py.None()).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
            diff.set_item("filename", new_name).unwrap();
            diff.set_item("modes", create_file_mode(file_mode, py))
                .unwrap();
        }
    }

    if let Some(mut binary_sizes) = binary_sizes {
        diff.set_item("binary", true).unwrap();
        let sizes: Vec<PyObject> = binary_sizes
            .drain(..)
            .map(move |x| create_bin_size(x, py))
            .collect();
        diff.set_item("binary_hunk_size", sizes.to_object(*py))
            .unwrap();
    } else {
        diff.set_item("binary", false).unwrap();
    }
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
