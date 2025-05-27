// tests/common.rs
use std::fs::{self, File};
use std::io::Write;
use tempfile::{TempDir, tempdir};
use anyhow::Result;

pub mod common_test_utils {
    use super::*; // To bring fs, File, Write, TempDir, Result into this module's scope

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
            .write_all(b"data\nplus+plus")?;
        File::create(dir.path().join("sub_dir/.hidden_file"))?
            .write_all(b"secret")?;
        Ok(dir)
    }
}