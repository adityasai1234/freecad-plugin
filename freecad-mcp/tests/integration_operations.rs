/// Integration tests for boolean operations using fake FreeCADCmd binaries.
use freecad_mcp::config::Config;
use freecad_mcp::tools::operations::{boolean_cut, boolean_union, boolean_intersection};
use freecad_mcp::tools::BooleanInput;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn config_with_binary(path: PathBuf) -> Config {
    let work_dir = std::env::temp_dir().join("freecad_ops_test");
    let session_doc = work_dir.join("session.FCStd");
    Config { freecadcmd_path: path, work_dir, session_doc, timeout_secs: 5, gui_mode: false }
}

/// Binary that returns `exists_json` for the first two calls (object_exists checks for
/// base + tool), then `op_json` for all subsequent calls (the actual operation).
fn make_binary_with_responses(exists_json: &str, op_json: &str) -> PathBuf {
    // Use keep() so the temp dir is NOT cleaned up when this function returns.
    let dir = tempfile::tempdir().unwrap().keep();
    let counter = dir.join("count");
    let bin_path = dir.join("fake_freecad");
    let script = format!(
        r#"#!/bin/sh
COUNTER="{counter}"
if [ ! -f "$COUNTER" ]; then
  echo 1 > "$COUNTER"
  echo '{exists}'
elif [ "$(cat $COUNTER)" -lt 2 ]; then
  echo 2 > "$COUNTER"
  echo '{exists}'
else
  echo '{op}'
fi"#,
        counter = counter.display(),
        exists = exists_json,
        op = op_json,
    );
    std::fs::write(&bin_path, &script).unwrap();
    std::fs::set_permissions(&bin_path, std::fs::Permissions::from_mode(0o755)).unwrap();
    bin_path
}

#[test]
fn boolean_cut_parses_result_object_id() {
    let bin = make_binary_with_responses(
        r#"{"exists":true}"#,
        r#"{"success":true,"object_id":"Cut001","label":"Cut"}"#,
    );
    let cfg = config_with_binary(bin);
    let input = BooleanInput { base_id: "Box001".into(), tool_id: "Cylinder001".into(), label: None };
    let result = boolean_cut(&input, &cfg).unwrap();
    assert_eq!(result.object_id, "Cut001");
    assert!(result.success);
}

#[test]
fn boolean_union_parses_result_object_id() {
    let bin = make_binary_with_responses(
        r#"{"exists":true}"#,
        r#"{"success":true,"object_id":"Fusion001","label":"Fusion"}"#,
    );
    let cfg = config_with_binary(bin);
    let input = BooleanInput { base_id: "Box001".into(), tool_id: "Box002".into(), label: None };
    let result = boolean_union(&input, &cfg).unwrap();
    assert_eq!(result.object_id, "Fusion001");
}

#[test]
fn boolean_intersection_parses_result_object_id() {
    let bin = make_binary_with_responses(
        r#"{"exists":true}"#,
        r#"{"success":true,"object_id":"Common001","label":"Common"}"#,
    );
    let cfg = config_with_binary(bin);
    let input = BooleanInput { base_id: "Sphere001".into(), tool_id: "Sphere002".into(), label: None };
    let result = boolean_intersection(&input, &cfg).unwrap();
    assert_eq!(result.object_id, "Common001");
}
