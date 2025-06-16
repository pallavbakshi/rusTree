use crate::core::options::{
    HtmlOptions, InputSourceOptions, ListingOptions, MetadataOptions, MiscOptions,
};

/// Context for formatting operations (borrowed references)
///
/// This context provides all the information needed for output formatting
/// operations in a focused, efficient manner. It uses borrowed references
/// to avoid unnecessary cloning for short-lived CLI operations.
#[derive(Debug)]
pub struct FormattingContext<'a> {
    pub input_source: &'a InputSourceOptions,
    pub listing: &'a ListingOptions,
    pub metadata: &'a MetadataOptions,
    pub misc: &'a MiscOptions,
    pub html: &'a HtmlOptions,
}

impl<'a> FormattingContext<'a> {
    /// Create a new formatting context from individual option references
    pub fn new(
        input_source: &'a InputSourceOptions,
        listing: &'a ListingOptions,
        metadata: &'a MetadataOptions,
        misc: &'a MiscOptions,
        html: &'a HtmlOptions,
    ) -> Self {
        Self {
            input_source,
            listing,
            metadata,
            misc,
            html,
        }
    }
}

/// Owned version for advanced scenarios
///
/// This context owns all its data and is designed for scenarios where
/// the context needs to live independently or be modified over time,
/// such as in interactive applications where users can change formatting options.
#[derive(Debug, Clone)]
pub struct OwnedFormattingContext {
    pub input_source: InputSourceOptions,
    pub listing: ListingOptions,
    pub metadata: MetadataOptions,
    pub misc: MiscOptions,
    pub html: HtmlOptions,
}

impl OwnedFormattingContext {
    /// Create a new owned formatting context
    pub fn new(
        input_source: InputSourceOptions,
        listing: ListingOptions,
        metadata: MetadataOptions,
        misc: MiscOptions,
        html: HtmlOptions,
    ) -> Self {
        Self {
            input_source,
            listing,
            metadata,
            misc,
            html,
        }
    }

    /// Validate context for consistency and correctness
    ///
    /// This method checks for invalid combinations of formatting options
    /// and provides helpful error messages for fixing configuration issues.
    pub fn validate(&self) -> Result<(), String> {
        // Validate root display name is not empty for certain operations
        if self.input_source.root_display_name.trim().is_empty() {
            return Err("root_display_name cannot be empty".to_string());
        }

        // Validate HTML options when HTML links are enabled
        if self.html.include_links {
            if let Some(ref base_href) = self.html.base_href {
                if base_href.trim().is_empty() {
                    return Err("base_href cannot be empty when specified".to_string());
                }

                // Basic URL validation - should start with http:// or https:// or be relative
                if !base_href.starts_with("http://")
                    && !base_href.starts_with("https://")
                    && !base_href.starts_with("/")
                    && !base_href.starts_with("./")
                    && !base_href.starts_with("../")
                {
                    return Err("base_href should be a valid URL or relative path".to_string());
                }
            }
        }

        // Validate custom HTML files exist if specified
        if let Some(ref intro_file) = self.html.custom_intro {
            if !intro_file.exists() {
                return Err(format!(
                    "custom_intro file does not exist: {}",
                    intro_file.display()
                ));
            }
        }

        if let Some(ref outro_file) = self.html.custom_outro {
            if !outro_file.exists() {
                return Err(format!(
                    "custom_outro file does not exist: {}",
                    outro_file.display()
                ));
            }
        }

        Ok(())
    }

    /// Create a borrowed context from this owned context
    ///
    /// This allows you to use the owned context with APIs that expect
    /// borrowed contexts, providing flexibility in how you manage context lifetimes.
    pub fn as_borrowed(&self) -> FormattingContext<'_> {
        FormattingContext {
            input_source: &self.input_source,
            listing: &self.listing,
            metadata: &self.metadata,
            misc: &self.misc,
            html: &self.html,
        }
    }

    /// Check if any metadata display is enabled
    ///
    /// This is a convenience method to determine if any metadata
    /// will be shown in the output, useful for optimizing formatting.
    pub fn has_metadata_display(&self) -> bool {
        self.metadata.show_size_bytes
            || self.metadata.show_last_modified
            || self.metadata.calculate_line_count
            || self.metadata.calculate_word_count
            || self.metadata.apply_function.is_some()
    }

    /// Check if this is a minimal output configuration
    ///
    /// Returns true if the configuration is set up for minimal,
    /// clean output with no extra information.
    pub fn is_minimal_output(&self) -> bool {
        !self.has_metadata_display() && self.misc.no_summary_report && !self.listing.show_full_path
    }
}

impl Default for OwnedFormattingContext {
    fn default() -> Self {
        Self::new(
            InputSourceOptions::default(),
            ListingOptions::default(),
            MetadataOptions::default(),
            MiscOptions::default(),
            HtmlOptions::default(),
        )
    }
}

impl<'a> From<FormattingContext<'a>> for OwnedFormattingContext {
    fn from(ctx: FormattingContext<'a>) -> Self {
        Self::new(
            ctx.input_source.clone(),
            ctx.listing.clone(),
            ctx.metadata.clone(),
            ctx.misc.clone(),
            ctx.html.clone(),
        )
    }
}

impl
    TryFrom<(
        InputSourceOptions,
        ListingOptions,
        MetadataOptions,
        MiscOptions,
        HtmlOptions,
    )> for OwnedFormattingContext
{
    type Error = String;

    fn try_from(
        (input_source, listing, metadata, misc, html): (
            InputSourceOptions,
            ListingOptions,
            MetadataOptions,
            MiscOptions,
            HtmlOptions,
        ),
    ) -> Result<Self, Self::Error> {
        let owned = Self::new(input_source, listing, metadata, misc, html);
        owned.validate()?;
        Ok(owned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::options::{ApplyFunction, BuiltInFunction};

    #[test]
    fn test_owned_formatting_context_creation() {
        let input_source = InputSourceOptions {
            root_display_name: "my_project".to_string(),
            root_is_directory: true,
            root_node_size: Some(1024),
        };

        let listing = ListingOptions {
            max_depth: Some(3),
            show_hidden: false,
            show_full_path: true,
            list_directories_only: false,
        };

        let metadata = MetadataOptions {
            show_size_bytes: true,
            show_last_modified: true,
            calculate_line_count: false,
            calculate_word_count: true,
            apply_function: Some(ApplyFunction::BuiltIn(BuiltInFunction::Cat)),
            ..Default::default()
        };

        let misc = MiscOptions {
            no_summary_report: false,
            human_friendly: false,
            no_color: false,
            verbose: false,
        };

        let html = HtmlOptions {
            include_links: true,
            base_href: Some("https://example.com".to_string()),
            strip_first_component: false,
            custom_intro: None,
            custom_outro: None,
        };

        let ctx = OwnedFormattingContext::new(input_source, listing, metadata, misc, html);

        assert_eq!(ctx.input_source.root_display_name, "my_project");
        assert!(ctx.input_source.root_is_directory);
        assert_eq!(ctx.input_source.root_node_size, Some(1024));
        assert_eq!(ctx.listing.max_depth, Some(3));
        assert!(ctx.listing.show_full_path);
        assert!(ctx.metadata.show_size_bytes);
        assert!(ctx.metadata.show_last_modified);
        assert!(ctx.metadata.calculate_word_count);
        assert!(!ctx.misc.no_summary_report);
        assert!(ctx.html.include_links);
        assert_eq!(ctx.html.base_href, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_validation_valid_context() {
        let ctx = OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "valid_name".to_string(),
                ..Default::default()
            },
            html: HtmlOptions {
                include_links: true,
                base_href: Some("https://example.com/docs".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(ctx.validate().is_ok());
    }

    #[test]
    fn test_validation_empty_root_name() {
        let ctx = OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "   ".to_string(), // Only whitespace
                ..Default::default()
            },
            ..Default::default()
        };

        let result = ctx.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("root_display_name cannot be empty")
        );
    }

    #[test]
    fn test_validation_invalid_base_href() {
        let ctx = OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "valid_name".to_string(),
                ..Default::default()
            },
            html: HtmlOptions {
                include_links: true,
                base_href: Some("invalid-url".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let result = ctx.validate();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("base_href should be a valid URL"));
    }

    #[test]
    fn test_validation_relative_paths_valid() {
        let relative_paths = vec![
            "/absolute/path",
            "./relative/path",
            "../parent/path",
            "https://example.com",
            "http://example.com",
        ];

        for path in relative_paths {
            let ctx = OwnedFormattingContext {
                input_source: InputSourceOptions {
                    root_display_name: "valid_name".to_string(),
                    ..Default::default()
                },
                html: HtmlOptions {
                    include_links: true,
                    base_href: Some(path.to_string()),
                    ..Default::default()
                },
                ..Default::default()
            };

            assert!(ctx.validate().is_ok(), "Failed for path: {}", path);
        }
    }

    #[test]
    fn test_has_metadata_display() {
        let ctx_with_metadata = OwnedFormattingContext {
            metadata: MetadataOptions {
                show_size_bytes: true,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(ctx_with_metadata.has_metadata_display());

        let ctx_with_line_count = OwnedFormattingContext {
            metadata: MetadataOptions {
                calculate_line_count: true,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(ctx_with_line_count.has_metadata_display());

        let ctx_minimal = OwnedFormattingContext::default();
        assert!(!ctx_minimal.has_metadata_display());
    }

    #[test]
    fn test_is_minimal_output() {
        let ctx_minimal = OwnedFormattingContext {
            misc: MiscOptions {
                no_summary_report: true,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(ctx_minimal.is_minimal_output());

        let ctx_with_metadata = OwnedFormattingContext {
            metadata: MetadataOptions {
                show_size_bytes: true,
                ..Default::default()
            },
            misc: MiscOptions {
                no_summary_report: true,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(!ctx_with_metadata.is_minimal_output());

        let ctx_with_summary = OwnedFormattingContext {
            misc: MiscOptions {
                no_summary_report: false,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(!ctx_with_summary.is_minimal_output());
    }

    #[test]
    fn test_borrowed_context_creation() {
        let input_source = InputSourceOptions::default();
        let listing = ListingOptions::default();
        let metadata = MetadataOptions::default();
        let misc = MiscOptions::default();
        let html = HtmlOptions::default();

        let ctx = FormattingContext::new(&input_source, &listing, &metadata, &misc, &html);

        // Verify references work
        assert!(!ctx.listing.show_hidden);
        assert!(!ctx.metadata.show_size_bytes);
    }

    #[test]
    fn test_conversion_from_borrowed_to_owned() {
        let input_source = InputSourceOptions {
            root_display_name: "test".to_string(),
            ..Default::default()
        };
        let listing = ListingOptions::default();
        let metadata = MetadataOptions::default();
        let misc = MiscOptions::default();
        let html = HtmlOptions::default();

        let borrowed_ctx = FormattingContext::new(&input_source, &listing, &metadata, &misc, &html);
        let owned_ctx: OwnedFormattingContext = borrowed_ctx.into();

        assert_eq!(owned_ctx.input_source.root_display_name, "test");
    }

    #[test]
    fn test_as_borrowed_method() {
        let owned_ctx = OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "test_project".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let borrowed_ctx = owned_ctx.as_borrowed();
        assert_eq!(borrowed_ctx.input_source.root_display_name, "test_project");
    }
}
