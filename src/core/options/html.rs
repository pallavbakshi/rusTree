use std::path::PathBuf;

/// Configuration specific to HTML output.
#[derive(Debug, Clone)]
pub struct HtmlOptions {
    /// If present, this string is prepended to every hyperlink that is
    /// generated (e.g. "https://example.org/").  It should **not** contain a
    /// trailing slash – the formatter takes care of inserting one.
    pub base_href: Option<String>,

    /// When `true`, the first path component of the generated relative link is
    /// stripped.  This mimics the behaviour of GNU tree's "-H -baseHREF".
    pub strip_first_component: bool,

    /// Use a custom intro fragment (HTML that comes *before* the `<pre>` tree)
    /// instead of the built-in default.
    pub custom_intro: Option<PathBuf>,

    /// Use a custom outro fragment (HTML that comes *after* the `<pre>` tree)
    /// instead of the built-in default.
    pub custom_outro: Option<PathBuf>,

    /// Whether to generate `<a href>` hyperlinks.  If `false`, only plain text
    /// (escaped) file names are shown.
    pub include_links: bool,
}

#[allow(clippy::derivable_impls)] // We have a custom default for include_links
impl Default for HtmlOptions {
    fn default() -> Self {
        Self {
            base_href: None,
            strip_first_component: false,
            custom_intro: None,
            custom_outro: None,
            // The default should be *false* so that plain text output is
            // produced unless users explicitly opt-in to hyperlink
            // generation.  This aligns the core defaults with CLI behaviour
            // and the expectations encoded in the test-suite.
            include_links: false,
        }
    }
}
