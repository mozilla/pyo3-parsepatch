use parsepatch::{Diff, FileOp, Patch};
use pyo3::types::{PyBytes, PyDict, PyTuple};
use pyo3::{IntoPyObject, PyObject, PyResult, Python};

pub struct PyDiff<'a> {
    py: Python<'a>,
    diff: &'a PyDict,
    lines: Vec<PyObject>,
}

impl<'a> PyDiff<'a> {
    fn new(py: Python<'a>) -> Self {
        PyDiff {
            py,
            diff: PyDict::new(py),
            lines: Vec::new(),
        }
    }

    fn get_line(&self, line: u32) -> PyObject {
        if line == 0 {
            self.py.None()
        } else {
            line.into_object(self.py)
        }
    }
}

pub struct PyPatch<'a> {
    py: Python<'a>,
    diffs: Vec<PyDiff<'a>>,
}

impl<'a> PyPatch<'a> {
    pub fn new(py: Python) -> PyPatch {
        PyPatch {
            py,
            diffs: Vec::new(),
        }
    }
}

impl<'a> Patch<PyDiff<'a>> for PyPatch<'a> {
    fn new_diff(&mut self) -> &mut PyDiff<'a> {
        self.diffs.push(PyDiff::new(self.py));
        self.diffs.last_mut().unwrap()
    }

    fn close(&mut self) {}
}

impl<'a> PyPatch<'a> {
    pub fn get_result(mut self) -> PyResult<PyObject> {
        let py = self.py;
        let diffs: Vec<PyObject> = self
            .diffs
            .drain(..)
            .map(move |x| {
                x.diff.set_item("lines", x.lines.into_object(py)).unwrap();
                x.diff.into_object(py)
            })
            .collect();
        Ok(diffs.into_object(self.py))
    }
}

impl<'a> Diff for PyDiff<'a> {
    fn set_info(&mut self, old_name: &str, new_name: &str, op: FileOp, binary: bool) {
        crate::common::set_info(self.diff, old_name, new_name, op, binary, &self.py);
    }

    fn add_line(&mut self, old_line: u32, new_line: u32, line: &[u8]) {
        self.lines.push(
            PyTuple::new(
                self.py,
                &[
                    self.get_line(old_line),
                    self.get_line(new_line),
                    PyBytes::new(self.py, line).into_object(self.py),
                ],
            )
            .into_object(self.py),
        );
    }

    fn close(&mut self) {}
}
