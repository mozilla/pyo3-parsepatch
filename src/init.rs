use parsepatch::PatchReader;
use pyo3::exceptions::TypeError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Python;

macro_rules! parse_patch {
    ($py:path, $bytes:path, $v:ident) => {
        {
            let mut patch = crate::$v::PyPatch::new($py);

            if let Some(bytes) = crate::common::get_bytes($py, &$bytes) {
                match bytes {
                    crate::common::Bytes::Slice(bytes) => PatchReader::by_buf(bytes, &mut patch),
                    crate::common::Bytes::Vec(bytes) => PatchReader::by_buf(&bytes, &mut patch),
                }
            } else {
                return Err(TypeError::py_err("Invalid patch type"))
            };

            patch.get_result()
        }
    };
}

#[pyfunction]
/// Get the added/deleted/moved lines for each file in the patch.
/// Each line is a tuple (old_line_no, new_line_no, line_bytes).
/// If the line is added then old_line_no is None.
/// If the line is deleted then new_line_no is None.
fn get_diffs(py: Python, bytes: PyObject) -> PyResult<PyObject> {
    parse_patch!(py, bytes, diffs)
}

#[pyfunction]
/// Get the number of added/deleted lines for each file in the patch
fn get_counts(py: Python, bytes: PyObject) -> PyResult<PyObject> {
    parse_patch!(py, bytes, counts)
}

#[pyfunction]
/// Get the added/deleted line numbers for each file in the patch
fn get_lines(py: Python, bytes: PyObject) -> PyResult<PyObject> {
    parse_patch!(py, bytes, lines)
}

#[pymodule]
fn rs_parsepatch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_counts))?;
    m.add_wrapped(wrap_pyfunction!(get_diffs))?;
    m.add_wrapped(wrap_pyfunction!(get_lines))?;
    Ok(())
}
