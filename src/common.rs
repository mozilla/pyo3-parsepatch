use parsepatch::FileOp;
use pyo3::types::PyDict;
use pyo3::Python;

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
            diff.set_item("renamed_from", py.None()).unwrap();
        }
        FileOp::Deleted => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", true).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
        }
        FileOp::Renamed => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("renamed_from", old_name).unwrap();
        }
        FileOp::None => {
            diff.set_item("new", false).unwrap();
            diff.set_item("deleted", false).unwrap();
            diff.set_item("renamed_from", py.None()).unwrap();
        }
    }
    diff.set_item("filename", new_name).unwrap();
    diff.set_item("binary", binary).unwrap();
}
