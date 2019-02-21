use parsepatch::PatchReader;
use pyo3::types::{PyBytes, PyModule};
use pyo3::Python;
use pyo3::prelude::*;

#[pyfunction]
fn get_diffs(py: Python, bytes: PyObject) -> PyResult<PyObject> {
    let mut patch = crate::diffs::PyPatch::new(py);
    let bytes = PyBytes::try_from(bytes.as_ref(py))?;
    PatchReader::by_buf(bytes.as_bytes(), &mut patch);
    patch.get_result()
}

#[pyfunction]
fn get_counts(py: Python, bytes: PyObject) -> PyResult<PyObject> {
    let mut patch = crate::counts::PyPatch::new(py);
    let bytes = PyBytes::try_from(bytes.as_ref(py))?;
    PatchReader::by_buf(bytes.as_bytes(), &mut patch);
    patch.get_result()
}

#[pymodinit]
fn rs_parsepatch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_function!(get_counts))?;
    m.add_function(wrap_function!(get_diffs))?;
    Ok(())
}
