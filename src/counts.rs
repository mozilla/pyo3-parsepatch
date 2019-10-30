use parsepatch::{BinaryHunk, Diff, FileMode, FileOp, Patch};
use pyo3::types::PyDict;
use pyo3::{PyObject, PyResult, Python, ToPyObject};

pub struct PyDiff<'a> {
    py: Python<'a>,
    diff: &'a PyDict,
    add: u32,
    del: u32,
}

impl<'a> PyDiff<'a> {
    fn new(py: Python<'a>) -> Self {
        PyDiff {
            py,
            diff: PyDict::new(py),
            add: 0,
            del: 0,
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
                x.diff.set_item("added_lines", x.add).unwrap();
                x.diff.set_item("deleted_lines", x.del).unwrap();
                x.diff.to_object(py)
            })
            .collect();

        Ok(diffs.to_object(self.py))
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
            self.diff,
            old_name,
            new_name,
            op,
            binary_sizes,
            file_mode,
            &self.py,
        );
    }

    fn add_line(&mut self, old_line: u32, new_line: u32, _line: &[u8]) {
        if old_line == 0 {
            self.add += 1;
        } else if new_line == 0 {
            self.del += 1;
        }
    }

    fn close(&mut self) {}

    fn new_hunk(&mut self) {}
}
