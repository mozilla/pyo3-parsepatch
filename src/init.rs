use parsepatch::PatchReader;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};
use pyo3::Python;

#[pyfunction]
/// Get the added/deleted/moved lines for each file in the patch.
/// Each line is a tuple (old_line_no, new_line_no, line_bytes).
/// If the line is added then old_line_no is None.
/// If the line is deleted then new_line_no is None.
fn get_diffs(py: Python, bytes: PyObject) -> PyResult<PyObject> {
    let mut patch = crate::diffs::PyPatch::new(py);
    let bytes = PyBytes::try_from(bytes.as_ref(py))?;
    PatchReader::by_buf(bytes.as_bytes(), &mut patch);
    patch.get_result()
}

#[pyfunction]
/// Get the number of added/deleted lines for each file in the patch
fn get_counts(py: Python, bytes: PyObject) -> PyResult<PyObject> {
    let mut patch = crate::counts::PyPatch::new(py);
    let bytes = PyBytes::try_from(bytes.as_ref(py))?;
    PatchReader::by_buf(bytes.as_bytes(), &mut patch);
    patch.get_result()
}

#[pyfunction]
/// Get the added/deleted line numbers for each file in the patch
fn get_lines(py: Python, bytes: PyObject) -> PyResult<PyObject> {
    let mut patch = crate::lines::PyPatch::new(py);
    let bytes = PyBytes::try_from(bytes.as_ref(py))?;
    PatchReader::by_buf(bytes.as_bytes(), &mut patch);
    patch.get_result()
}

#[pymodinit]
fn rs_parsepatch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_function!(get_counts))?;
    m.add_function(wrap_function!(get_diffs))?;
    m.add_function(wrap_function!(get_lines))?;
    Ok(())
}
