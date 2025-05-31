/// Options related to the source of input data (e.g., specific path, current directory).
#[derive(Debug, Clone, Default)]
pub struct InputSourceOptions {
    /// Display name for the root directory/file.
    pub root_display_name: String,
    /// Size of the root node, if available and size reporting is enabled.
    pub root_node_size: Option<u64>,
    /// Whether the root path represents a directory (true) or file (false).
    pub root_is_directory: bool,
}
