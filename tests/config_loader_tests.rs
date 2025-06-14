//! Tests for persistent configuration loader and precedence rules.

use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use rustree::config::{RustreeLibConfig, load_merged_config};

// Global lock to ensure environment & cwd mutation is not concurrent.
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn lock() -> std::sync::MutexGuard<'static, ()> {
    let mtx = TEST_LOCK.get_or_init(|| Mutex::new(()));
    match mtx.lock() {
        Ok(g) => g,
        Err(poison) => poison.into_inner(),
    }
}

#[test]
fn precedence_explicit_overrides_project_and_global() {
    let _guard = lock();

    let tmp = tempfile::tempdir().unwrap();
    let global_dir = tmp.path().join("global");
    let project_dir = tmp.path().join("project");
    let _ = fs::create_dir_all(global_dir.join("rustree"));
    let _ = fs::create_dir_all(project_dir.join(".rustree"));

    // 1. global config -> show_hidden = false
    fs::write(
        global_dir.join("rustree/config.toml"),
        "[listing]\nshow_hidden = false\n",
    )
    .unwrap();

    // 2. project config -> show_hidden = true
    fs::write(
        project_dir.join(".rustree/config.toml"),
        "[listing]\nshow_hidden = true\n",
    )
    .unwrap();

    // 3. explicit config -> show_hidden = false
    let explicit_file = tmp.path().join("explicit.toml");
    fs::write(&explicit_file, "[listing]\nshow_hidden = false\n").unwrap();

    // Set env vars & cwd
    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", &global_dir);
    }
    std::env::set_current_dir(&project_dir).unwrap();

    let (partial, _src) = load_merged_config(&[explicit_file.clone()], true).unwrap();
    let mut cfg = RustreeLibConfig::default();
    partial.merge_into(&mut cfg);

    assert!(
        !cfg.listing.show_hidden,
        "explicit config should win and disable show_hidden"
    );
}

#[test]
fn missing_file_returns_error() {
    let _guard = lock();
    let missing = PathBuf::from("/path/does/not/exist.toml");
    let err = load_merged_config(&[missing], true).expect_err("should error");
    assert!(err.to_string().contains("No such file") || err.to_string().contains("cannot find"));
}

#[test]
fn bad_toml_returns_error() {
    let _guard = lock();
    let tmp = tempfile::NamedTempFile::new().unwrap();
    fs::write(tmp.path(), "[listing\nshow_hidden = true\n").unwrap(); // missing closing bracket
    let result = load_merged_config(&[tmp.path().to_path_buf()], true);
    assert!(result.is_err(), "invalid TOML should return error");
}

#[test]
fn llm_api_key_env_indirection() {
    let _guard = lock();
    let tmp = tempfile::NamedTempFile::new().unwrap();
    fs::write(
        tmp.path(),
        "[llm]\napi_key_env = \"TEST_LLM_KEY\"\nprovider = \"openai\"\nmodel = \"gpt-4o\"\n",
    )
    .unwrap();

    unsafe {
        std::env::set_var("TEST_LLM_KEY", "dummy123");
    }

    let (partial, _) = load_merged_config(&[tmp.path().to_path_buf()], true).unwrap();
    assert!(partial.llm.is_some());
    let llm = partial.llm.unwrap();
    assert_eq!(llm.api_key_env.unwrap(), "TEST_LLM_KEY");
}
