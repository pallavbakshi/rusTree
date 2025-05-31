use crate::config::metadata::ApplyFnError;
use std::path::PathBuf;
use std::time::SystemTime;

/// Represents information about a single file system entry (file, directory, or symlink).
///
/// This struct is populated by the directory walker and contains metadata and analysis
/// results for each node in the directory tree.
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// The full path to the file system entry.
    pub path: PathBuf,
    /// The name of the file or directory.
    pub name: String,
    /// The type of the node (File, Directory, or Symlink).
    pub node_type: NodeType,
    /// The depth of the node in the directory tree, relative to the scan root.
    /// The direct children of the scan root have depth 1.
    pub depth: usize,
    /// The size of the file in bytes. `None` for directories or if not reported.
    pub size: Option<u64>,
    /// File permissions, represented as a string (e.g., "rwxr-xr--"). `None` if not reported.
    /// (Note: Actual formatting of permissions is not yet implemented in output).
    pub permissions: Option<String>,
    /// The last modification time of the entry. `None` if not reported or error.
    pub mtime: Option<SystemTime>,
    /// The last status change time of the entry (ctime). `None` if not reported or error.
    pub change_time: Option<SystemTime>,
    /// The creation time of the entry (btime). `None` if not reported or error.
    pub create_time: Option<SystemTime>,
    /// The number of lines in the file. `None` for directories or if not calculated.
    pub line_count: Option<usize>,
    /// The number of words in the file. `None` for directories or if not calculated.
    pub word_count: Option<usize>,
    /// The output of a custom function applied to the file's content.
    /// `Some(Ok(String))` for successful execution, `Some(Err(ApplyFnError))` for failure,
    /// `None` if no function was applied or for directories.
    pub custom_function_output: Option<Result<String, ApplyFnError>>,
}

/// Enumerates the types of file system entries that `rustree` can represent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    /// Represents a regular file.
    File,
    /// Represents a directory.
    Directory,
    /// Represents a symbolic link.
    Symlink,
} 