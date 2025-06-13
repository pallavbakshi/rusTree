use rustree::config::metadata::{ExternalFunction, FunctionOutputKind};
use rustree::core::metadata::file_info::apply_external_to_file;
use std::fs::File;
use std::io::Write;

#[test]
fn test_apply_external_function_number() {
    // Prepare a temporary file with known content (3 lines)
    let dir = tempfile::tempdir().expect("tempdir");
    let file_path = dir.path().join("test.txt");
    {
        let mut f = File::create(&file_path).unwrap();
        writeln!(f, "line1\nline2\nline3").unwrap();
    }

    let ext_fn = ExternalFunction {
        cmd_template: "wc -l < {}".to_string(),
        timeout_secs: 5,
        kind: FunctionOutputKind::Number,
    };

    let res = apply_external_to_file(&file_path, &ext_fn).expect("ok");
    assert_eq!(res.trim(), "3");
}
