use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

mod common;

/// Creates a test directory structure with both files and directories.
/// Structure:
/// temp_dir/
/// ├── afile.txt
/// ├── bdir/
/// │   └── nested.txt
/// ├── cfile.md  
/// └── ddir/
///     └── another.txt
fn create_test_structure(temp_dir: &Path) -> std::io::Result<()> {
    // Create files
    fs::write(temp_dir.join("afile.txt"), "content1")?;
    fs::write(temp_dir.join("cfile.md"), "content2")?;

    // Create directories with files inside
    fs::create_dir(temp_dir.join("bdir"))?;
    fs::write(temp_dir.join("bdir").join("nested.txt"), "nested content")?;

    fs::create_dir(temp_dir.join("ddir"))?;
    fs::write(temp_dir.join("ddir").join("another.txt"), "another content")?;

    Ok(())
}

#[test]
fn test_dirs_first_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    create_test_structure(temp_dir.path()).expect("Failed to create test structure");

    let output = Command::new(common::get_binary_path())
        .arg(temp_dir.path())
        .arg("--dirs-first")
        .arg("-L")
        .arg("1")
        .output()
        .expect("Failed to execute rustree");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // With --dirs-first, directories should appear before files
    // Expected order: bdir/, ddir/, afile.txt, cfile.md
    let lines: Vec<&str> = stdout.lines().collect();

    // Find the relevant lines (skip the root directory line)
    let mut content_lines = Vec::new();
    for line in &lines {
        if line.contains("bdir")
            || line.contains("ddir")
            || line.contains("afile.txt")
            || line.contains("cfile.md")
        {
            content_lines.push(*line);
        }
    }

    assert!(
        content_lines.len() >= 4,
        "Should have at least 4 content lines"
    );

    // Check that directories come before files
    let mut dir_positions = Vec::new();
    let mut file_positions = Vec::new();

    for (i, line) in content_lines.iter().enumerate() {
        if line.contains("bdir") || line.contains("ddir") {
            dir_positions.push(i);
        } else if line.contains("afile.txt") || line.contains("cfile.md") {
            file_positions.push(i);
        }
    }

    // All directories should come before all files
    let max_dir_pos = dir_positions.iter().max().unwrap_or(&0);
    let min_file_pos = file_positions.iter().min().unwrap_or(&999);

    assert!(
        max_dir_pos < min_file_pos,
        "Directories should appear before files with --dirs-first. Max dir pos: {}, Min file pos: {}",
        max_dir_pos,
        min_file_pos
    );
}

#[test]
fn test_files_first_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    create_test_structure(temp_dir.path()).expect("Failed to create test structure");

    let output = Command::new(common::get_binary_path())
        .arg(temp_dir.path())
        .arg("--files-first")
        .arg("-L")
        .arg("1")
        .output()
        .expect("Failed to execute rustree");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // With --files-first, files should appear before directories
    // Expected order: afile.txt, cfile.md, bdir/, ddir/
    let lines: Vec<&str> = stdout.lines().collect();

    // Find the relevant lines (skip the root directory line)
    let mut content_lines = Vec::new();
    for line in &lines {
        if line.contains("bdir")
            || line.contains("ddir")
            || line.contains("afile.txt")
            || line.contains("cfile.md")
        {
            content_lines.push(*line);
        }
    }

    assert!(
        content_lines.len() >= 4,
        "Should have at least 4 content lines"
    );

    // Check that files come before directories
    let mut dir_positions = Vec::new();
    let mut file_positions = Vec::new();

    for (i, line) in content_lines.iter().enumerate() {
        if line.contains("bdir") || line.contains("ddir") {
            dir_positions.push(i);
        } else if line.contains("afile.txt") || line.contains("cfile.md") {
            file_positions.push(i);
        }
    }

    // All files should come before all directories
    let max_file_pos = file_positions.iter().max().unwrap_or(&0);
    let min_dir_pos = dir_positions.iter().min().unwrap_or(&999);

    assert!(
        max_file_pos < min_dir_pos,
        "Files should appear before directories with --files-first. Max file pos: {}, Min dir pos: {}",
        max_file_pos,
        min_dir_pos
    );
}

#[test]
fn test_default_ordering_mixed() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    create_test_structure(temp_dir.path()).expect("Failed to create test structure");

    let output = Command::new(common::get_binary_path())
        .arg(temp_dir.path())
        .arg("-L")
        .arg("1")
        .output()
        .expect("Failed to execute rustree");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // With default ordering, items should be sorted alphabetically (mixed)
    // Expected order: afile.txt, bdir/, cfile.md, ddir/
    let lines: Vec<&str> = stdout.lines().collect();

    let mut content_lines = Vec::new();
    for line in &lines {
        if line.contains("bdir")
            || line.contains("ddir")
            || line.contains("afile.txt")
            || line.contains("cfile.md")
        {
            content_lines.push(*line);
        }
    }

    // Should have mixed ordering (not all dirs first, not all files first)
    let mut has_file_before_dir = false;
    let mut has_dir_before_file = false;

    for i in 0..content_lines.len() - 1 {
        let current_is_dir = content_lines[i].contains("dir");
        let next_is_dir = content_lines[i + 1].contains("dir");

        if !current_is_dir && next_is_dir {
            has_file_before_dir = true;
        }
        if current_is_dir && !next_is_dir {
            has_dir_before_file = true;
        }
    }

    // In default mode with alphabetical ordering, we should have mixed ordering
    assert!(
        has_file_before_dir || has_dir_before_file,
        "Default ordering should be mixed (alphabetical), not grouped by type"
    );
}

#[test]
fn test_flags_conflict() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    create_test_structure(temp_dir.path()).expect("Failed to create test structure");

    let output = Command::new(common::get_binary_path())
        .arg(temp_dir.path())
        .arg("--dirs-first")
        .arg("--files-first")
        .output()
        .expect("Failed to execute rustree");

    // Should fail with error about conflicting arguments
    assert!(
        !output.status.success(),
        "Command should fail when both flags are provided"
    );

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(
        stderr.contains("cannot be used with"),
        "Error message should mention conflicting arguments"
    );
}

#[test]
fn test_dirs_first_with_different_sort_keys() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    create_test_structure(temp_dir.path()).expect("Failed to create test structure");

    // Test with size sorting
    let output = Command::new(common::get_binary_path())
        .arg(temp_dir.path())
        .arg("--dirs-first")
        .arg("--sort-by")
        .arg("size")
        .arg("-L")
        .arg("1")
        .output()
        .expect("Failed to execute rustree");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Even with size sorting, directories should still come first
    let lines: Vec<&str> = stdout.lines().collect();
    let mut content_lines = Vec::new();
    for line in &lines {
        if line.contains("bdir")
            || line.contains("ddir")
            || line.contains("afile.txt")
            || line.contains("cfile.md")
        {
            content_lines.push(*line);
        }
    }

    let mut dir_positions = Vec::new();
    let mut file_positions = Vec::new();

    for (i, line) in content_lines.iter().enumerate() {
        if line.contains("bdir") || line.contains("ddir") {
            dir_positions.push(i);
        } else if line.contains("afile.txt") || line.contains("cfile.md") {
            file_positions.push(i);
        }
    }

    if !dir_positions.is_empty() && !file_positions.is_empty() {
        let max_dir_pos = dir_positions.iter().max().unwrap();
        let min_file_pos = file_positions.iter().min().unwrap();

        assert!(
            max_dir_pos < min_file_pos,
            "Directories should come before files even with size sorting"
        );
    }
}

#[test]
fn test_files_first_with_reverse_sort() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    create_test_structure(temp_dir.path()).expect("Failed to create test structure");

    let output = Command::new(common::get_binary_path())
        .arg(temp_dir.path())
        .arg("--files-first")
        .arg("--reverse-sort")
        .arg("-L")
        .arg("1")
        .output()
        .expect("Failed to execute rustree");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // With --files-first and reverse sort, files should come before directories
    // and within each group they should be reverse sorted
    let lines: Vec<&str> = stdout.lines().collect();
    let mut content_lines = Vec::new();
    for line in &lines {
        if line.contains("bdir")
            || line.contains("ddir")
            || line.contains("afile.txt")
            || line.contains("cfile.md")
        {
            content_lines.push(*line);
        }
    }

    // Find first directory and last file
    let mut last_file_pos = None;
    let mut first_dir_pos = None;

    for (i, line) in content_lines.iter().enumerate() {
        if line.contains("afile.txt") || line.contains("cfile.md") {
            last_file_pos = Some(i);
        } else if (line.contains("bdir") || line.contains("ddir")) && first_dir_pos.is_none() {
            first_dir_pos = Some(i);
        }
    }

    if let (Some(last_file), Some(first_dir)) = (last_file_pos, first_dir_pos) {
        assert!(
            last_file < first_dir,
            "Files should still come before directories with --files-first and reverse sort"
        );
    }
}

#[test]
fn test_directory_ordering_in_subdirectories() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a more complex structure
    fs::create_dir(temp_dir.path().join("parent")).unwrap();
    fs::write(temp_dir.path().join("parent").join("zfile.txt"), "content").unwrap();
    fs::create_dir(temp_dir.path().join("parent").join("asubdir")).unwrap();
    fs::write(
        temp_dir
            .path()
            .join("parent")
            .join("asubdir")
            .join("nested.txt"),
        "nested",
    )
    .unwrap();
    fs::write(temp_dir.path().join("parent").join("bfile.txt"), "content2").unwrap();

    let output = Command::new(common::get_binary_path())
        .arg(temp_dir.path())
        .arg("--dirs-first")
        .arg("-L")
        .arg("2")
        .output()
        .expect("Failed to execute rustree");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Check that directory ordering applies at all levels
    let lines: Vec<&str> = stdout.lines().collect();

    // Find lines in the parent directory
    let mut parent_content = Vec::new();
    let mut in_parent = false;

    for line in &lines {
        if line.contains("parent/") || line.contains("parent\\") {
            in_parent = true;
            continue;
        }
        if in_parent {
            if line.contains("asubdir") || line.contains("bfile.txt") || line.contains("zfile.txt")
            {
                parent_content.push(*line);
            }
            // Stop when we reach another top-level item or end
            if !line.starts_with("│")
                && !line.starts_with("├")
                && !line.starts_with("└")
                && !line.trim().is_empty()
            {
                break;
            }
        }
    }

    // In the parent directory, asubdir/ should come before bfile.txt and zfile.txt
    if parent_content.len() >= 2 {
        let dir_found = parent_content
            .iter()
            .position(|line| line.contains("asubdir"));
        let file_found = parent_content
            .iter()
            .position(|line| line.contains("file.txt"));

        if let (Some(dir_pos), Some(file_pos)) = (dir_found, file_found) {
            assert!(
                dir_pos < file_pos,
                "Directory should come before files in subdirectory too"
            );
        }
    }
}
