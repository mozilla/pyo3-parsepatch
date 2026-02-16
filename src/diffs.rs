use parsepatch::{BinaryHunk, Diff, FileMode, FileOp, Patch};
use pyo3::prelude::PyDictMethods;
use pyo3::types::{PyBytes, PyDict, PyTuple};
use pyo3::{Bound, IntoPyObject, Py, PyAny, PyResult, Python};

pub struct PyDiff<'a> {
    py: Python<'a>,
    diff: Bound<'a, PyDict>,
    lines: Vec<Py<PyAny>>,
    hunks: Vec<Vec<Py<PyAny>>>,
    has_hunks: bool,
}

impl<'a> PyDiff<'a> {
    fn new(py: Python<'a>, has_hunks: bool) -> Self {
        PyDiff {
            py,
            diff: PyDict::new(py),
            lines: Vec::new(),
            hunks: Vec::new(),
            has_hunks,
        }
    }

    fn get_line(&self, line: u32) -> Py<PyAny> {
        if line == 0 {
            self.py.None()
        } else {
            line.into_pyobject(self.py).unwrap().into_any().unbind()
        }
    }
}

pub struct PyPatch<'a> {
    py: Python<'a>,
    diffs: Vec<PyDiff<'a>>,
    hunks: bool,
}

impl<'a> PyPatch<'a> {
    pub fn new(py: Python, hunks: bool) -> PyPatch {
        PyPatch {
            py,
            diffs: Vec::new(),
            hunks,
        }
    }
}

impl<'a> Patch<PyDiff<'a>> for PyPatch<'a> {
    fn new_diff(&mut self) -> &mut PyDiff<'a> {
        self.diffs.push(PyDiff::new(self.py, self.hunks));
        self.diffs.last_mut().unwrap()
    }

    fn close(&mut self) {}
}

impl<'a> PyPatch<'a> {
    pub fn get_result(mut self) -> PyResult<Py<PyAny>> {
        let py = self.py;
        if self.hunks {
            let diffs: Vec<Py<PyAny>> = self
                .diffs
                .drain(..)
                .map(move |x| {
                    x.diff.set_item("hunks", x.hunks).unwrap();
                    x.diff.into_any().unbind()
                })
                .collect();
            Ok(diffs.into_pyobject(py)?.into_any().unbind())
        } else {
            let diffs: Vec<Py<PyAny>> = self
                .diffs
                .drain(..)
                .map(move |x| {
                    x.diff.set_item("lines", x.lines).unwrap();
                    x.diff.into_any().unbind()
                })
                .collect();
            Ok(diffs.into_pyobject(py)?.into_any().unbind())
        }
    }
}

impl<'a> Diff for PyDiff<'a> {
    fn set_info(
        &mut self,
        old_name: &str,
        new_name: &str,
        op: FileOp,
        binary_sizes: Option<Vec<BinaryHunk>>,
        file_mode: Option<FileMode>,
    ) {
        crate::common::set_info(
            &self.diff,
            old_name,
            new_name,
            op,
            binary_sizes,
            file_mode,
            &self.py,
        );
    }

    fn add_line(&mut self, old_line: u32, new_line: u32, line: &[u8]) {
        let line = PyTuple::new(
            self.py,
            &[
                self.get_line(old_line),
                self.get_line(new_line),
                PyBytes::new(self.py, line).into_any().unbind(),
            ],
        )
        .unwrap()
        .into_any()
        .unbind();

        if self.has_hunks {
            self.hunks.last_mut().unwrap().push(line);
        } else {
            self.lines.push(line);
        }
    }

    fn close(&mut self) {}

    fn new_hunk(&mut self) {
        if self.has_hunks {
            self.hunks.push(Vec::new());
        }
    }
}
