// tests/common.rs
use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use tempfile::{TempDir, tempdir};

pub mod common_test_utils {
    use super::*; // To bring fs, File, Write, TempDir, Result into this module's scope
    use std::path::Path; // Add this

    #[allow(dead_code)] // This function is used by other test files
    pub fn setup_test_directory() -> Result<TempDir> {
        let dir = tempdir()?;
        // Create a structure:
        // test_dir/
        //   file1.txt (3 lines, "hello\nworld\nrust")
        //   file2.log (1 line, "another file")
        //   sub_dir/
        //     file3.dat (2 lines, "data\nplus+plus")
        //     .hidden_file (1 line, "secret")
        File::create(dir.path().join("file1.txt"))?.write_all(b"hello\nworld\nrust")?;
        File::create(dir.path().join("file2.log"))?.write_all(b"another file")?;
        fs::create_dir(dir.path().join("sub_dir"))?;
        File::create(dir.path().join("sub_dir/file3.dat"))?.write_all(b"data\nplus++plus")?; // Changed to have two '+' characters
        File::create(dir.path().join("sub_dir/.hidden_file"))?.write_all(b"secret")?;
        Ok(dir)
    }

    #[allow(dead_code)] // This function is used by other test files, but not within common.rs tests
    pub fn create_file_with_content(dir_path: &Path, file_name: &str, content: &str) -> Result<()> {
        let mut file = File::create(dir_path.join(file_name))?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[allow(dead_code)] // Used by pattern_matching_tests.rs
    pub fn setup_complex_test_directory() -> Result<TempDir> {
        let dir = tempdir()?;
        let base = dir.path();

        // Root level files
        create_file_with_content(base, "file_a.txt", "content of file_a.txt")?;
        create_file_with_content(base, "file_b.log", "content of file_b.log")?;
        create_file_with_content(base, ".hidden_file.txt", "hidden content")?;
        create_file_with_content(base, "image.JPG", "image data")?;
        create_file_with_content(base, "script.sh", "#!/bin/bash\necho hello")?;

        // sub_dir/
        let sub_dir_path = base.join("sub_dir");
        fs::create_dir(&sub_dir_path)?;
        create_file_with_content(&sub_dir_path, "sub_file.rs", "fn main() {}")?;
        create_file_with_content(&sub_dir_path, ".sub_hidden_file", "sub hidden content")?;

        // another_dir/
        let another_dir_path = base.join("another_dir");
        fs::create_dir(&another_dir_path)?;
        create_file_with_content(&another_dir_path, "another_file.dat", "data content")?;

        // empty_dir/
        fs::create_dir(base.join("empty_dir"))?;

        // Symlinks (conditionally created for Unix/Windows)
        if cfg!(unix) {
            std::os::unix::fs::symlink(
                base.join("file_a.txt"),
                base.join("symlink_to_file_a.txt"),
            )?;
            std::os::unix::fs::symlink(&sub_dir_path, base.join("symlink_to_sub_dir"))?;
        } else if cfg!(windows) {
            #[cfg(windows)]
            {
                // On Windows, symlink creation might require special privileges.
                // std::os::windows::fs::symlink_file for files, symlink_dir for directories.
                // These calls return Result, so they can fail gracefully if permissions are not met.
                let _ = std::os::windows::fs::symlink_file(
                    base.join("file_a.txt"),
                    base.join("symlink_to_file_a.txt"),
                );
                let _ = std::os::windows::fs::symlink_dir(
                    &sub_dir_path,
                    base.join("symlink_to_sub_dir"),
                );
            }
        }

        Ok(dir)
    }

    #[allow(dead_code)]
    pub fn setup_gitignore_test_dir() -> Result<TempDir> {
        let dir = tempfile::tempdir()?;
        let base = dir.path();

        // Root files and dirs
        create_file_with_content(base, "file.txt", "content")?;
        create_file_with_content(base, "file.log", "log content")?;
        fs::create_dir(base.join("docs"))?;
        create_file_with_content(&base.join("docs"), "api.md", "api docs")?;
        fs::create_dir(base.join("target"))?;
        create_file_with_content(&base.join("target"), "app.exe", "binary")?;
        create_file_with_content(base, "image.PNG", "image data")?;
        create_file_with_content(base, "image.png", "image data lowercase")?;

        // .gitignore at root
        let mut root_gitignore = File::create(base.join(".gitignore"))?;
        writeln!(root_gitignore, "*.log")?;
        writeln!(root_gitignore, "target/")?;
        writeln!(root_gitignore, "IMAGE.PNG")?; // Test case sensitivity for gitignore

        // Nested dir with its own .gitignore
        fs::create_dir(base.join("src"))?;
        create_file_with_content(&base.join("src"), "main.rs", "rust code")?;
        create_file_with_content(&base.join("src"), "module.temp", "temp file")?;
        let mut src_gitignore = File::create(base.join("src/.gitignore"))?;
        writeln!(src_gitignore, "*.temp")?;

        // Hidden files/dirs
        create_file_with_content(base, ".secret_file", "secret")?;
        fs::create_dir(base.join(".hidden_dir"))?;
        create_file_with_content(
            &base.join(".hidden_dir"),
            "content.txt",
            "hidden dir content",
        )?;

        Ok(dir)
    }

    #[allow(dead_code)] // Helper function to get root name
    pub fn get_root_name_from_path(path: &Path) -> String {
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    }
}

// Context testing utilities
pub mod context_utils {
    use rustree::core::options::DirectoryFileOrder;
    use rustree::core::options::contexts::*;
    use rustree::*;

    #[allow(dead_code)]
    pub fn create_test_walking_context() -> OwnedWalkingContext {
        OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(2),
                show_hidden: false,
                list_directories_only: false,
                show_full_path: false,
            },
            FilteringOptions {
                ignore_patterns: Some(vec!["*.tmp".to_string()]),
                match_patterns: Some(vec!["*.rs".to_string()]),
                case_insensitive_filter: false,
                ..Default::default()
            },
            MetadataOptions {
                show_size_bytes: true,
                calculate_line_count: true,
                calculate_word_count: false,
                ..Default::default()
            },
        )
    }

    #[allow(dead_code)]
    pub fn create_test_formatting_context() -> OwnedFormattingContext {
        OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "test".to_string(),
                root_is_directory: true,
                root_node_size: None,
            },
            listing: ListingOptions {
                max_depth: Some(2),
                show_hidden: false,
                list_directories_only: false,
                show_full_path: false,
            },
            metadata: MetadataOptions {
                show_size_bytes: true,
                show_last_modified: false,
                calculate_line_count: true,
                calculate_word_count: false,
                apply_function: None,
                human_readable_size: false,
                report_permissions: false,
                report_change_time: false,
                report_creation_time: false,
            },
            misc: MiscOptions {
                no_summary_report: false,
                human_friendly: false,
                no_color: false,
                verbose: false,
            },
            html: HtmlOptions {
                include_links: false,
                base_href: None,
                strip_first_component: false,
                custom_intro: None,
                custom_outro: None,
            },
        }
    }

    #[allow(dead_code)]
    pub fn create_test_sorting_context() -> OwnedSortingContext {
        OwnedSortingContext {
            sorting: SortingOptions {
                sort_by: Some(SortKey::Name),
                reverse_sort: false,
                files_before_directories: false,
                directory_file_order: DirectoryFileOrder::DirsFirst,
            },
        }
    }

    #[allow(dead_code)]
    pub fn create_test_processing_context() -> OwnedProcessingContext {
        OwnedProcessingContext {
            walking: create_test_walking_context(),
            sorting: Some(create_test_sorting_context()),
            formatting: create_test_formatting_context(),
        }
    }

    #[allow(dead_code)]
    pub fn create_minimal_walking_context() -> OwnedWalkingContext {
        OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(1),
                show_hidden: false,
                list_directories_only: false,
                show_full_path: false,
            },
            FilteringOptions::default(),
            MetadataOptions::default(),
        )
    }

    #[allow(dead_code)]
    pub fn create_deep_walking_context() -> OwnedWalkingContext {
        OwnedWalkingContext::new(
            ListingOptions {
                max_depth: Some(5),
                show_hidden: true,
                list_directories_only: false,
                show_full_path: true,
            },
            FilteringOptions {
                ignore_patterns: Some(vec!["*.tmp".to_string(), "*.bak".to_string()]),
                match_patterns: None,
                case_insensitive_filter: true,
                ..Default::default()
            },
            MetadataOptions {
                show_size_bytes: true,
                show_last_modified: true,
                calculate_line_count: true,
                calculate_word_count: true,
                ..Default::default()
            },
        )
    }

    #[allow(dead_code)]
    pub fn create_minimal_formatting_context() -> OwnedFormattingContext {
        OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "minimal".to_string(),
                root_is_directory: true,
                root_node_size: None,
            },
            listing: ListingOptions::default(),
            metadata: MetadataOptions::default(),
            misc: MiscOptions {
                no_summary_report: true,
                ..Default::default()
            },
            html: HtmlOptions::default(),
        }
    }

    #[allow(dead_code)]
    pub fn create_html_formatting_context() -> OwnedFormattingContext {
        OwnedFormattingContext {
            input_source: InputSourceOptions {
                root_display_name: "html_test".to_string(),
                root_is_directory: true,
                root_node_size: None,
            },
            listing: ListingOptions::default(),
            metadata: MetadataOptions {
                show_size_bytes: true,
                ..Default::default()
            },
            misc: MiscOptions::default(),
            html: HtmlOptions {
                include_links: true,
                base_href: Some("https://example.com".to_string()),
                strip_first_component: false,
                custom_intro: None,
                custom_outro: None,
            },
        }
    }

    #[allow(dead_code)]
    pub fn assert_contexts_equivalent(
        config: &RustreeLibConfig,
        walking_ctx: &WalkingContext,
        formatting_ctx: &FormattingContext,
        sorting_ctx: Option<&SortingContext>,
    ) {
        // Verify walking context equivalence
        assert_eq!(walking_ctx.listing.max_depth, config.listing.max_depth);
        assert_eq!(walking_ctx.listing.show_hidden, config.listing.show_hidden);
        assert_eq!(
            walking_ctx.filtering.ignore_patterns,
            config.filtering.ignore_patterns
        );
        assert_eq!(
            walking_ctx.metadata.show_size_bytes,
            config.metadata.show_size_bytes
        );

        // Verify formatting context equivalence
        assert_eq!(
            formatting_ctx.input_source.root_display_name,
            config.input_source.root_display_name
        );
        assert_eq!(
            formatting_ctx.misc.no_summary_report,
            config.misc.no_summary_report
        );
        assert_eq!(formatting_ctx.html.include_links, config.html.include_links);

        // Verify sorting context equivalence
        match (sorting_ctx, &config.sorting.sort_by) {
            (Some(ctx), Some(_)) => {
                assert_eq!(ctx.sorting.sort_by, config.sorting.sort_by);
                assert_eq!(ctx.sorting.reverse_sort, config.sorting.reverse_sort);
            }
            (None, None) => {} // Both None is valid
            _ => panic!("Sorting context mismatch"),
        }
    }
}

#[allow(dead_code)] // Used by CLI integration tests
pub fn get_binary_path() -> String {
    // Use the path to the built binary in target/debug/
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{}/target/debug/rustree", cargo_manifest_dir)
}
