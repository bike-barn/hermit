use std::path::PathBuf;
use file_operations::Op;

pub fn link_op_for(root_path: &PathBuf, op_root: &PathBuf, filename: &str) -> Op {
    Op::Link { target: root_path.join(filename), path: op_root.join(filename) }
}
