use std::path::{Path, PathBuf};

// Create a new PathBuf from a root and a '/' delimited components
pub fn path_joins(root: &Path, sub_path: &str) -> PathBuf {
    let parts = sub_path.split('/');
    let mut path = root.to_owned();
    for part in parts {
        path.push(part);
    }
    path
}