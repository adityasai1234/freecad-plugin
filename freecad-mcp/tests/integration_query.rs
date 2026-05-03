/// Integration tests for query tools using fake FreeCADCmd binaries.
use freecad_mcp::config::Config;
use freecad_mcp::tools::query::{get_document_stats, get_object_info, list_objects};
use freecad_mcp::tools::ObjectIdInput;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn config_with_binary(path: PathBuf) -> Config {
    let work_dir = std::env::temp_dir().join("freecad_query_test");
    let session_doc = work_dir.join("session.FCStd");
    Config { freecadcmd_path: path, work_dir, session_doc, timeout_secs: 5, gui_mode: false }
}

fn make_static_binary(json: &str) -> PathBuf {
    let dir = tempfile::tempdir().unwrap().into_path();
    let p = dir.join("fake_freecad");
    let mut f = std::fs::File::create(&p).unwrap();
    writeln!(f, "#!/bin/sh").unwrap();
    writeln!(f, "echo '{json}'").unwrap();
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
    p
}

/// Binary that returns `exists_json` on first call, then `op_json` on subsequent calls.
fn make_binary_exists_then(exists_json: &str, op_json: &str) -> PathBuf {
    let dir = tempfile::tempdir().unwrap().into_path();
    let counter = dir.join("count");
    let bin_path = dir.join("fake_freecad");
    let script = format!(
        "#!/bin/sh\nif [ ! -f \"{counter}\" ]; then touch \"{counter}\"; echo '{exists}'\nelse echo '{op}'\nfi",
        counter = counter.display(),
        exists = exists_json,
        op = op_json,
    );
    std::fs::write(&bin_path, &script).unwrap();
    std::fs::set_permissions(&bin_path, std::fs::Permissions::from_mode(0o755)).unwrap();
    bin_path
}

#[test]
fn list_objects_parses_empty_list() {
    let bin = make_static_binary(r#"{"success":true,"objects":[]}"#);
    let cfg = config_with_binary(bin);
    let result = list_objects(&cfg).unwrap();
    assert!(result.success);
    assert_eq!(result.objects.len(), 0);
}

#[test]
fn list_objects_parses_three_objects() {
    let bin = make_static_binary(
        r#"{"success":true,"objects":[{"id":"Box001","label":"Box","type_id":"Part::Box"},{"id":"Cylinder001","label":"Cylinder","type_id":"Part::Cylinder"},{"id":"Cut001","label":"Cut","type_id":"Part::Cut"}]}"#,
    );
    let cfg = config_with_binary(bin);
    let result = list_objects(&cfg).unwrap();
    assert_eq!(result.objects.len(), 3);
    assert_eq!(result.objects[0].id, "Box001");
    assert_eq!(result.objects[2].type_id, "Part::Cut");
}

#[test]
fn get_object_info_parses_full_result() {
    let info_json = r#"{"success":true,"id":"Box001","label":"Box","type_id":"Part::Box","volume_mm3":125000.0,"surface_area_mm2":15000.0,"bounding_box":{"xmin":0.0,"xmax":100.0,"ymin":0.0,"ymax":50.0,"zmin":0.0,"zmax":25.0}}"#;
    let bin = make_binary_exists_then(r#"{"exists":true}"#, info_json);
    let cfg = config_with_binary(bin);
    let input = ObjectIdInput { object_id: "Box001".into() };
    let result = get_object_info(&input, &cfg).unwrap();
    assert_eq!(result.id, "Box001");
    assert_eq!(result.volume_mm3, 125000.0);
    assert_eq!(result.bounding_box.xmax, 100.0);
}

#[test]
fn get_document_stats_parses_result() {
    let bin = make_static_binary(
        r#"{"success":true,"object_count":3,"total_volume_mm3":250000.0,"units":"mm","session_doc":"/tmp/session.FCStd"}"#,
    );
    let cfg = config_with_binary(bin);
    let result = get_document_stats(&cfg).unwrap();
    assert!(result.success);
    assert_eq!(result.object_count, 3);
    assert_eq!(result.units, "mm");
}
