/// Integration tests for export, save, and measure tools using fake FreeCADCmd binaries.
use freecad_mcp::config::Config;
use freecad_mcp::tools::export::{export_obj, export_stl, export_step, save_document};
use freecad_mcp::tools::measure::measure_distance;
use freecad_mcp::tools::{ExportInput, MeasureInput};
use std::path::PathBuf;

fn config_with_binary(path: PathBuf) -> Config {
    let work_dir = std::env::temp_dir().join("freecad_export_test");
    let session_doc = work_dir.join("session.FCStd");
    Config {
        freecadcmd_path: path,
        work_dir,
        session_doc,
        timeout_secs: 5,
        gui_mode: false,
    }
}

fn make_static_binary(json: &str) -> PathBuf {
    let dir = tempfile::tempdir().unwrap().keep();
    let bin_path = dir.join("fake_freecad");
    let script = format!("#!/bin/sh\necho '{json}'\n");
    std::fs::write(&bin_path, script).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&bin_path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    bin_path
}

/// One object_exists response, then the operation JSON on later calls.
fn make_binary_one_exists_then(op_json: &str) -> PathBuf {
    let dir = tempfile::tempdir().unwrap().keep();
    let counter = dir.join("count");
    let bin_path = dir.join("fake_freecad");
    let script = format!(
        r#"#!/bin/sh
COUNTER="{counter}"
if [ ! -f "$COUNTER" ]; then
  echo 1 > "$COUNTER"
  echo '{{"exists":true}}'
else
  echo '{op}'
fi"#,
        counter = counter.display(),
        op = op_json,
    );
    std::fs::write(&bin_path, &script).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&bin_path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    bin_path
}

/// Two object_exists responses (measured objects), then the operation JSON.
fn make_binary_two_exists_then(op_json: &str) -> PathBuf {
    let dir = tempfile::tempdir().unwrap().keep();
    let counter = dir.join("count");
    let bin_path = dir.join("fake_freecad");
    let script = format!(
        r#"#!/bin/sh
COUNTER="{counter}"
if [ ! -f "$COUNTER" ]; then
  echo 1 > "$COUNTER"
  echo '{{"exists":true}}'
elif [ "$(cat $COUNTER)" -lt 2 ]; then
  echo 2 > "$COUNTER"
  echo '{{"exists":true}}'
else
  echo '{op}'
fi"#,
        counter = counter.display(),
        op = op_json,
    );
    std::fs::write(&bin_path, &script).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&bin_path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    bin_path
}

#[test]
fn save_document_parses_path() {
    let bin = make_static_binary(r#"{"success":true,"path":"/tmp/ws/session.FCStd"}"#);
    let cfg = config_with_binary(bin);
    let res = save_document(&cfg).unwrap();
    assert!(res.success);
    assert!(res.path.ends_with("session.FCStd"));
}

#[test]
fn export_stl_parses_result() {
    let payload = r#"{"success":true,"path":"/tmp/ws/out.stl","size_bytes":2048}"#;
    let bin = make_binary_one_exists_then(payload);
    let cfg = config_with_binary(bin);
    let input = ExportInput {
        object_id: "Box001".into(),
        filename: Some("out.stl".into()),
    };
    let res = export_stl(&input, &cfg).unwrap();
    assert_eq!(res.size_bytes, 2048);
}

#[test]
fn export_step_accepts_stp_extension() {
    let payload = r#"{"success":true,"path":"/tmp/ws/m.part.stp","size_bytes":4096}"#;
    let bin = make_binary_one_exists_then(payload);
    let cfg = config_with_binary(bin);
    let input = ExportInput {
        object_id: "Cut001".into(),
        filename: Some("m.part.stp".into()),
    };
    let res = export_step(&input, &cfg).unwrap();
    assert!(res.success);
}

#[test]
fn export_obj_parses_result() {
    let payload = r#"{"success":true,"path":"/tmp/ws/m.obj","size_bytes":512}"#;
    let bin = make_binary_one_exists_then(payload);
    let cfg = config_with_binary(bin);
    let input = ExportInput {
        object_id: "Box001".into(),
        filename: None,
    };
    let res = export_obj(&input, &cfg).unwrap();
    assert!(res.success);
}

#[test]
fn measure_distance_parses_result() {
    let payload = r#"{"success":true,"distance_mm":12.5}"#;
    let bin = make_binary_two_exists_then(payload);
    let cfg = config_with_binary(bin);
    let input = MeasureInput {
        object_id_a: "Box001".into(),
        object_id_b: "Sphere001".into(),
    };
    let res = measure_distance(&input, &cfg).unwrap();
    assert!(res.success);
    assert!((res.distance_mm - 12.5).abs() < f64::EPSILON);
}
