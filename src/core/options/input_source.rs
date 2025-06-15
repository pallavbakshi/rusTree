/// Options that describe the *source* that is being processed (typically the
/// root path that is passed to the walker).
///
/// Keeping these options together allows us to pass all relevant information
/// about the input in a single struct instead of having many loosely-coupled
/// parameters.  The majority of `rustree` operations only need to **read** the
/// fields, therefore a very small, cheap-to-clone struct is perfectly fine and
/// ergonomic.
#[derive(Debug, Clone)]
pub struct InputSourceOptions {
    /// The display name that should be used for the *root* of the processed
    /// tree (e.g. what is shown for `.`).  It is part of user-facing output so
    /// having a sensible non-empty default avoids a whole class of validation
    /// errors in higher-level code.
    pub root_display_name: String,
    /// Size of the root node, if it is known upfront and size reporting is
    /// enabled.
    pub root_node_size: Option<u64>,
    /// Indicates whether the configured *root path* represents a directory
    /// (`true`) or a single file (`false`).
    pub root_is_directory: bool,
}

impl Default for InputSourceOptions {
    fn default() -> Self {
        Self {
            // A non-empty string prevents validation errors in the default
            // configuration and still conveys meaningful information to the
            // user when printed.
            root_display_name: "root".to_string(),
            root_node_size: None,
            // Assume directory as that is by far the most common case; callers
            // can override it when they know the root is a file.
            root_is_directory: true,
        }
    }
}
