/// Tests for bridge::runner::run_freecad_script using real subprocesses.
///
/// These tests do NOT require FreeCAD — they use tiny shell scripts to simulate
/// the FreeCADCmd binary and verify the runner's parsing and error-handling logic.
use freecad_mcp::bridge::runner::run_freecad_script;
use freecad_mcp::config::Config;
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
    let dir = tempfile::tempdir().unwrap();
    let p = dir.into_path().join("fake_freecad");
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
