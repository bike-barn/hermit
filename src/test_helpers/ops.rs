use file_operations::Op;
use std::path::PathBuf;

pub fn link_op_for(root_path: &PathBuf, op_root: &PathBuf, filename: &str) -> Op {
    Op::Link {
        target: root_path.join(filename),
        path: op_root.join(filename),
    }
}
