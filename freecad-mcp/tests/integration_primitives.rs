/// Tests for bridge::runner::run_freecad_script and tool-level primitives.
///
/// These tests do NOT require FreeCAD — they use tiny shell scripts to simulate
/// the FreeCADCmd binary and verify the runner's parsing and error-handling logic.
use freecad_mcp::bridge::runner::run_freecad_script;
use freecad_mcp::config::Config;
use freecad_mcp::tools::primitives::{create_box, create_cylinder};
use freecad_mcp::tools::placement::place_object;
use freecad_mcp::tools::{CreateBoxInput, CreateCylinderInput, PlaceObjectInput};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

/// Build a minimal Config pointing at a custom fake binary.
fn config_with_binary(path: PathBuf) -> Config {
    let work_dir = std::env::temp_dir().join("freecad_test_workspace");
    let session_doc = work_dir.join("session.FCStd");
    Config { freecadcmd_path: path, work_dir, session_doc, timeout_secs: 5, gui_mode: false }
}

/// Write an executable shell script to a temp file and return its path.
fn make_fake_binary(body: &str) -> PathBuf {
    let dir = tempfile::tempdir().unwrap().keep();
    let p = dir.join("fake_freecad");
    let mut f = std::fs::File::create(&p).unwrap();
    writeln!(f, "#!/bin/sh").unwrap();
    f.write_all(body.as_bytes()).unwrap();
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
    p
}

#[test]
fn valid_json_output_is_returned_as_value() {
    let bin = make_fake_binary(r#"echo '{"success":true,"object_id":"Box001","label":"Box"}'"#);
    let cfg = config_with_binary(bin);
    let result = run_freecad_script("# dummy", &cfg).unwrap();
    assert_eq!(result["success"], true);
    assert_eq!(result["object_id"], "Box001");
}

#[test]
fn non_zero_exit_returns_subprocess_failed() {
    let bin = make_fake_binary("echo 'something went wrong' >&2\nexit 1");
    let cfg = config_with_binary(bin);
    let err = run_freecad_script("# dummy", &cfg).unwrap_err();
    assert!(
        matches!(err, freecad_mcp::error::FreeCADError::SubprocessFailed(_)),
        "expected SubprocessFailed, got {err:?}"
    );
}

#[test]
fn non_json_stdout_returns_parse_error() {
    let bin = make_fake_binary("echo 'this is not json'");
    let cfg = config_with_binary(bin);
    let err = run_freecad_script("# dummy", &cfg).unwrap_err();
    assert!(
        matches!(err, freecad_mcp::error::FreeCADError::ParseError(_)),
        "expected ParseError, got {err:?}"
    );
}

// ─── Chain test: create_box → create_cylinder → place_object ─────────────────
// Each step uses a fake binary that returns the expected fixture JSON.

#[test]
fn create_box_parses_object_result() {
    let bin = make_fake_binary(
        r#"echo '{"success":true,"object_id":"Box001","label":"Box"}'"#,
    );
    let cfg = config_with_binary(bin);
    let input = CreateBoxInput { length: 100.0, width: 50.0, height: 25.0, label: None };
    let result = create_box(&input, &cfg).unwrap();
    assert_eq!(result.object_id, "Box001");
    assert!(result.success);
}

#[test]
fn create_cylinder_parses_object_result() {
    let bin = make_fake_binary(
        r#"echo '{"success":true,"object_id":"Cylinder001","label":"Cylinder"}'"#,
    );
    let cfg = config_with_binary(bin);
    let input = CreateCylinderInput { radius: 10.0, height: 30.0, label: None };
    let result = create_cylinder(&input, &cfg).unwrap();
    assert_eq!(result.object_id, "Cylinder001");
}

#[test]
fn place_object_parses_object_result() {
    // First call: object_exists check returns {"exists": true}
    // Second call: the actual placement returns object result.
    // We need a binary that handles two invocations in sequence — use a counter file.
    let dir = tempfile::tempdir().unwrap();
    let counter = dir.path().join("count");
    let bin_path = dir.path().join("fake_freecad");
    let script = format!(
        r#"#!/bin/sh
COUNT_FILE="{counter}"
if [ -f "$COUNT_FILE" ]; then
  echo '{{"success":true,"object_id":"Cylinder001","label":"Cylinder"}}'
else
  touch "$COUNT_FILE"
  echo '{{"exists":true}}'
fi"#,
        counter = counter.display()
    );
    std::fs::write(&bin_path, &script).unwrap();
    std::fs::set_permissions(&bin_path, std::fs::Permissions::from_mode(0o755)).unwrap();

    let cfg = config_with_binary(bin_path);
    let input = PlaceObjectInput {
        object_id: "Cylinder001".into(),
        x: 100.0, y: 50.0, z: 0.0,
        yaw: None, pitch: None, roll: None,
    };
    let result = place_object(&input, &cfg).unwrap();
    assert_eq!(result.object_id, "Cylinder001");
}
