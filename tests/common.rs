// tests/common.rs
use std::fs::{self, File};
use std::io::Write;
use tempfile::{TempDir, tempdir};
use anyhow::Result;

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
        File::create(dir.path().join("file1.txt"))?
            .write_all(b"hello\nworld\nrust")?;
        File::create(dir.path().join("file2.log"))?
            .write_all(b"another file")?;
        fs::create_dir(dir.path().join("sub_dir"))?;
        File::create(dir.path().join("sub_dir/file3.dat"))?
            .write_all(b"data\nplus++plus")?; // Changed to have two '+' characters
        File::create(dir.path().join("sub_dir/.hidden_file"))?
            .write_all(b"secret")?;
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
            std::os::unix::fs::symlink(base.join("file_a.txt"), base.join("symlink_to_file_a.txt"))?;
            std::os::unix::fs::symlink(&sub_dir_path, base.join("symlink_to_sub_dir"))?;
        } else if cfg!(windows) {
            #[cfg(windows)]
            {
                // On Windows, symlink creation might require special privileges.
                // std::os::windows::fs::symlink_file for files, symlink_dir for directories.
                // These calls return Result, so they can fail gracefully if permissions are not met.
                let _ = std::os::windows::fs::symlink_file(base.join("file_a.txt"), base.join("symlink_to_file_a.txt"));
                let _ = std::os::windows::fs::symlink_dir(&sub_dir_path, base.join("symlink_to_sub_dir"));
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
        create_file_with_content(&base.join(".hidden_dir"), "content.txt", "hidden dir content")?;

        Ok(dir)
    }
}