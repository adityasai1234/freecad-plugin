# CLAUDE.md — freecad-mcp (Rust)

## What this project is

A production-grade MCP server written in Rust that exposes FreeCAD as a set of
tools Claude can call autonomously. Claude creates, modifies, queries, and exports
3D parametric designs by chaining tool calls against a persistent FreeCAD document.
Transport: stdio (Claude Desktop + Claude Code). FreeCAD runs headless in an isolated
subprocess — the Rust process never links against FreeCAD directly.

---

## Absolute rules — read before touching any file

- **One commit per file.** Never stage multiple new files in one commit.
- **Max 150 lines of net-new code per commit** during implementation passes.
- **Push after every commit.** No local-only chains.
- **Scaffold first, implement second.** Scaffold = file + struct/trait stubs +
  `todo!()` bodies + doc comments. Logic comes in the next pass.
- **`cargo check` must pass before every commit.** No commit with compiler errors.
- **`cargo test` must pass before every commit.** No commit with failing tests.
- **No `unwrap()` or `expect()` in non-test code.** Use `?` and `anyhow::Result`.
- **No `async` in tool logic.** Only `mcp_server.rs` and `main.rs` are async.
  Tool functions are synchronous and called from async wrappers.
- **No FreeCAD binary calls outside `bridge/runner.rs`.** The runner is the only
  file allowed to spawn subprocesses.
- **All paths from env or `Config`.** Zero hardcoded paths anywhere.
- **TASKS.md and MISTAKES.md stay updated** throughout the session.

---

## Stop — ask me before proceeding if:

- Any file would exceed 300 lines
- You need a crate not in the approved list below
- The subprocess protocol needs to change
- A tool needs filesystem access outside `FREECAD_WORK_DIR`
- You are unsure whether a FreeCAD Python API call works headless

---

## Approved crates — add nothing else without asking

```toml
[dependencies]
rmcp               = { version = "0.1", features = ["server", "transport-io"] }
tokio              = { version = "1", features = ["full"] }
serde              = { version = "1", features = ["derive"] }
serde_json         = "1"
anyhow             = "1"
thiserror          = "1"
tracing            = "1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tempfile           = "3"
which              = "6"

[dev-dependencies]
mockall    = "0.12"
tokio-test = "0.4"
```

---

## Project structure — final target

```
freecad-mcp/
├── CLAUDE.md
├── TASKS.md
├── MISTAKES.md
├── README.md
├── Cargo.toml
├── Cargo.lock
├── Makefile
├── .env.example
├── .gitignore
├── claude_desktop_config.json
│
├── src/
│   ├── main.rs                  # Entry: env, tracing, run MCP server
│   ├── mcp_server.rs            # rmcp tool registration, async wrappers only
│   ├── config.rs                # Config struct from env vars
│   ├── error.rs                 # FreeCADError enum (thiserror)
│   │
│   ├── bridge/
│   │   ├── mod.rs
│   │   ├── runner.rs            # subprocess execution + JSON parsing
│   │   └── session.rs           # document helpers + path helpers
│   │
│   ├── tools/
│   │   ├── mod.rs               # All input/output types + re-exports
│   │   ├── primitives.rs        # create_box, create_cylinder, create_sphere, create_cone
│   │   ├── placement.rs         # place_object, rotate_object
│   │   ├── operations.rs        # boolean_union, boolean_cut, boolean_intersection
│   │   ├── sketcher.rs          # create_sketch, add_rectangle, add_circle, extrude, revolve
│   │   ├── query.rs             # get_object_info, list_objects, get_document_stats
│   │   ├── export.rs            # export_stl, export_step, export_obj, save_document
│   │   └── measure.rs           # measure_distance
│   │
│   └── scripts/
│       └── mod.rs               # ScriptBuilder struct
│
└── tests/
    ├── integration_primitives.rs
    ├── integration_operations.rs
    └── integration_query.rs
```

---

## The 50 commits — log every error in MISTAKES.md immediately.

---

### Phase 1 — Project Skeleton (commits 1–7)

**Commit 1** `chore: init cargo project with approved crates`
- `cargo new freecad-mcp --name freecad-mcp`
- Set Rust edition 2021 in Cargo.toml
- Add all approved crates to [dependencies] and [dev-dependencies]
- Add .gitignore: `target/`, `.env`, `*.FCStd`, `*.stl`, `*.step`
- Files: `Cargo.toml`, `Cargo.lock`, `.gitignore`

**Commit 2** `feat: add config.rs with env-driven Config struct`
- `Config` fields: `freecadcmd_path: PathBuf`, `work_dir: PathBuf`,
  `session_doc: PathBuf` (computed), `timeout_secs: u64`, `gui_mode: bool`
- `Config::from_env() -> anyhow::Result<Config>` reads env vars with fallback defaults
- `Config::validate(&self) -> anyhow::Result<()>` → stub with `todo!()`
- File: `src/config.rs`

**Commit 3** `feat: add error.rs with FreeCADError enum`
- Variants (thiserror derives):
  - `SubprocessFailed(String)`
  - `ParseError(String)`
  - `Timeout`
  - `BinaryNotFound(PathBuf)`
  - `ObjectNotFound(String)`
  - `InvalidInput(String)`
  - `IoError(#[from] std::io::Error)`
  - `JsonError(#[from] serde_json::Error)`
- File: `src/error.rs`

**Commit 4** `feat: scaffold bridge/runner.rs with run_freecad_script stub`
- `pub fn run_freecad_script(script: &str, config: &Config) -> Result<serde_json::Value, FreeCADError>`
- Body: `todo!()`
- Doc comment: explains temp-file → FreeCADCmd → parse-last-JSON-line contract
- Files: `src/bridge/runner.rs`, `src/bridge/mod.rs`

**Commit 5** `feat: scaffold bridge/session.rs`
- `pub fn ensure_workspace(config: &Config) -> Result<(), FreeCADError>` → `todo!()`
- `pub fn session_doc_path(config: &Config) -> PathBuf` → `config.session_doc.clone()`
- `pub fn object_exists(object_id: &str, config: &Config) -> Result<bool, FreeCADError>` → `todo!()`
- File: `src/bridge/session.rs`

**Commit 6** `feat: scaffold all tool files with function stubs`
- Create all 7 files in `src/tools/`
- Each: struct stubs + function signatures + `todo!()` + doc comments
- Zero logic — stubs only
- Exception to one-file rule: this is the scaffolding commit for all tools
- Files: all `src/tools/*.rs`

**Commit 7** `feat: scaffold main.rs and mcp_server.rs`
- `main.rs`: `#[tokio::main] async fn main() -> anyhow::Result<()>` → `todo!()`
- `mcp_server.rs`: `pub struct FreeCADMcpServer { config: Arc<Config> }` + empty `impl`
- Files: `src/main.rs`, `src/mcp_server.rs`

---

### Phase 2 — FreeCAD Bridge (commits 8–14)

**Commit 8** `feat: implement Config::from_env and Config::validate`
- `from_env()`: read all env vars, compute `session_doc = work_dir/session.FCStd`
- `validate()`: use `which` crate to verify `freecadcmd_path` exists;
  call `fs::create_dir_all(&self.work_dir)`
- Unit tests: valid config passes, missing binary fails, default fallback works
- File: `src/config.rs`

**Commit 9** `feat: implement run_freecad_script in bridge/runner.rs`
- Write script to `tempfile::NamedTempFile` with `.py` suffix
- Spawn `std::process::Command` with freecadcmd_path + temp path
- `wait_with_output` with manual timeout via `std::thread::spawn` + channel
- Non-zero exit → `SubprocessFailed(stderr[-500..])`
- Timeout → `FreeCADError::Timeout`
- Parse last non-empty stdout line as JSON
- Bad JSON → `ParseError`
- Delete temp file in all branches
- File: `src/bridge/runner.rs`

**Commit 10** `test: unit tests for run_freecad_script`
- Mock subprocess using a tiny shell script written to tempfile
- Test: valid JSON → `Ok(Value)`
- Test: non-zero exit → `SubprocessFailed`
- Test: non-JSON stdout → `ParseError`
- File: `tests/integration_primitives.rs` (create and add test stubs)

**Commit 11** `feat: implement ensure_workspace in bridge/session.rs`
- `fs::create_dir_all(&config.work_dir)`
- Idempotent: `Ok(())` if already exists
- Unit test: call twice, assert no error
- File: `src/bridge/session.rs`

**Commit 12** `feat: add ScriptBuilder in scripts/mod.rs`
- `pub struct ScriptBuilder { lines: Vec<String> }`
- `fn new(session_doc: &Path) -> Self` → injects header:
  ```python
  import FreeCAD, os, sys
  DOC_PATH = "{path}"
  try:
      if os.path.exists(DOC_PATH):
          doc = FreeCAD.openDocument(DOC_PATH)
      else:
          doc = FreeCAD.newDocument("session")
  ```
- `fn push(&mut self, line: impl Into<String>)`
- `fn finish(self, result_expr: &str) -> String`
  appends `print(json.dumps({result_expr}))` inside try, plus:
  ```python
  except Exception as e:
      print(json.dumps({"success": False, "error": str(e)}))
  ```
- `fn build(self) -> String` → `self.lines.join("\n")`
- File: `src/scripts/mod.rs`

**Commit 13** `feat: implement bridge/mod.rs public convenience API`
- Re-export `run_freecad_script`, `ensure_workspace`, `object_exists`, `ScriptBuilder`
- `pub fn run(script: String, config: &Config) -> Result<serde_json::Value, FreeCADError>`
  → calls `run_freecad_script(&script, config)`
- File: `src/bridge/mod.rs`

**Commit 14** `feat: implement object_exists in bridge/session.rs`
- Script: `obj = doc.getObject("{id}"); print(json.dumps({"exists": obj is not None}))`
- Returns `Ok(true/false)`
- Used by boolean ops for early `ObjectNotFound` return
- File: `src/bridge/session.rs`

---

### Phase 3 — Input/Output Types (commits 15–17)

**Commit 15** `feat: define input types for primitives and placement in tools/mod.rs`
- `CreateBoxInput { length: f64, width: f64, height: f64, label: Option<String> }`
- `CreateCylinderInput { radius: f64, height: f64, label: Option<String> }`
- `CreateSphereInput { radius: f64, label: Option<String> }`
- `CreateConeInput { radius1: f64, radius2: f64, height: f64, label: Option<String> }`
- `PlaceObjectInput { object_id: String, x: f64, y: f64, z: f64, yaw: Option<f64>, pitch: Option<f64>, roll: Option<f64> }`
- All derive `Serialize, Deserialize, Debug, Clone`
- Each has `fn validate(&self) -> Result<(), FreeCADError>` checking positive floats
- File: `src/tools/mod.rs`

**Commit 16** `feat: define input types for booleans, query, export, sketcher`
- `BooleanInput { base_id: String, tool_id: String, label: Option<String> }`
- `ObjectIdInput { object_id: String }`
- `ExportInput { object_id: String, filename: Option<String> }`
- `SketchPlane` enum `{ XY, XZ, YZ }` with `Serialize, Deserialize`
- `CreateSketchInput { plane: SketchPlane }`
- `AddRectangleInput { sketch_id: String, x: f64, y: f64, width: f64, height: f64 }`
- `AddCircleInput { sketch_id: String, cx: f64, cy: f64, radius: f64 }`
- `ExtrudeInput { sketch_id: String, depth: f64, symmetric: Option<bool> }`
- `RevolveAxis` enum `{ X, Y, Z }`
- `RevolveInput { sketch_id: String, axis: RevolveAxis, angle_deg: f64 }`
- File: `src/tools/mod.rs` (append)

**Commit 17** `feat: define all output types in tools/mod.rs`
- `ObjectResult { success: bool, object_id: String, label: String }`
- `ObjectSummary { id: String, label: String, type_id: String }`
- `ListResult { success: bool, objects: Vec<ObjectSummary> }`
- `BoundingBox { xmin: f64, xmax: f64, ymin: f64, ymax: f64, zmin: f64, zmax: f64 }`
- `InfoResult { success: bool, id: String, label: String, type_id: String, volume_mm3: f64, surface_area_mm2: f64, bounding_box: BoundingBox }`
- `ExportResult { success: bool, path: String, size_bytes: u64 }`
- `DistanceResult { success: bool, distance_mm: f64 }`
- `ErrorResult { success: bool, error: String }`
- File: `src/tools/mod.rs` (append)

---

### Phase 4 — Primitives (commits 18–23)

**Commit 18** `feat: implement create_box in tools/primitives.rs`
- `pub fn create_box(input: &CreateBoxInput, config: &Config) -> Result<ObjectResult, FreeCADError>`
- Validate input, build script via `ScriptBuilder`, call `bridge::run`
- Script body:
  ```python
  label = "Box"           # from input
  obj = doc.addObject("Part::Box", label)
  obj.Length = 100.0      # from input
  obj.Width  = 50.0
  obj.Height = 25.0
  doc.recompute()
  doc.saveAs(DOC_PATH)
  ```
- Result expr: `"success": True, "object_id": obj.Name, "label": obj.Label`
- Deserialize response into `ObjectResult`
- Unit test with mocked `run_freecad_script` returning fixture JSON
- File: `src/tools/primitives.rs`

**Commit 19** `feat: implement create_cylinder in tools/primitives.rs`
- `Part::Cylinder` with `Radius` and `Height`
- Same pattern: validate → build → run → deserialize
- Unit test
- File: `src/tools/primitives.rs`

**Commit 20** `feat: implement create_sphere in tools/primitives.rs`
- `Part::Sphere` with `Radius`
- Unit test
- File: `src/tools/primitives.rs`

**Commit 21** `feat: implement create_cone in tools/primitives.rs`
- `Part::Cone` with `Radius1`, `Radius2`, `Height`
- Unit test
- File: `src/tools/primitives.rs`

**Commit 22** `feat: implement place_object and rotate_object in tools/placement.rs`
- `place_object`: `obj.Placement.Base = FreeCAD.Vector(x, y, z)`
- `rotate_object`: `obj.Placement.Rotation = FreeCAD.Rotation(yaw, pitch, roll)`
- Both check `object_exists` first, return `ObjectNotFound` if missing
- Unit tests for both, including not-found case
- File: `src/tools/placement.rs`

**Commit 23** `test: primitives integration tests`
- Mock subprocess with fixture JSON responses
- Test chain: create_box → create_cylinder → place_object
- Assert object_ids match fixtures
- File: `tests/integration_primitives.rs`

---

### Phase 5 — Boolean Operations (commits 24–28)

**Commit 24** `feat: implement boolean_union in tools/operations.rs`
- Check both objects exist via `object_exists`
- Script: `Part::Fuse` with `Shapes = [base_obj, tool_obj]`
- `doc.recompute()` + `doc.saveAs(DOC_PATH)`
- Return `ObjectResult`
- Unit test: happy path + one object missing
- File: `src/tools/operations.rs`

**Commit 25** `feat: implement boolean_cut in tools/operations.rs`
- Script: `Part::Cut` with `Base` and `Tool` properties
- Unit test
- File: `src/tools/operations.rs`

**Commit 26** `feat: implement boolean_intersection in tools/operations.rs`
- Script: `Part::Common`
- Unit test
- File: `src/tools/operations.rs`

**Commit 27** `refactor: extract script_for_boolean helper in operations.rs`
- Reduce duplication across union/cut/intersection
- `fn script_for_boolean(op_type: &str, base: &str, tool: &str, label: &str, doc_path: &str) -> String`
- No behavior change — refactor only
- File: `src/tools/operations.rs`

**Commit 28** `test: boolean operations integration tests`
- Test cut chain: box → cylinder → cut → verify result id format
- Test union: two boxes → fuse
- Test intersection: two overlapping spheres
- File: `tests/integration_operations.rs`

---

### Phase 6 — Query Tools (commits 29–32)

**Commit 29** `feat: implement list_objects in tools/query.rs`
- Script: iterate `doc.Objects`, filter `obj.TypeId.startswith("Part::")`
- Return `[{"id": obj.Name, "label": obj.Label, "type_id": obj.TypeId}]`
- Deserialize into `ListResult`
- Unit test: empty doc, doc with 3 objects
- File: `src/tools/query.rs`

**Commit 30** `feat: implement get_object_info in tools/query.rs`
- Script:
  ```python
  obj = doc.getObject(object_id)
  shape = obj.Shape
  bb = shape.BoundBox
  ```
- Return volume, surface area, bounding box
- Return `ObjectNotFound` when `obj is None`
- Unit tests: found + not found
- File: `src/tools/query.rs`

**Commit 31** `feat: implement get_document_stats in tools/query.rs`
- Returns: object count, sum of all volumes, units string `"mm"`, session doc path
- Script: iterate all Part objects, sum `obj.Shape.Volume`
- File: `src/tools/query.rs`

**Commit 32** `test: query integration tests`
- Mock multi-object response fixture
- Test list → get_info → stats chain
- File: `tests/integration_query.rs`

---

### Phase 7 — Export Tools (commits 33–37)

**Commit 33** `feat: implement save_document in tools/export.rs`
- Script: `doc.saveAs(DOC_PATH)`
- Return `{"success": true, "path": DOC_PATH}`
- File: `src/tools/export.rs`

**Commit 34** `feat: implement export_stl in tools/export.rs`
- Output path: `{work_dir}/{object_id}_{unix_timestamp}.stl`
- Script:
  ```python
  import Mesh
  Mesh.export([doc.getObject(object_id)], output_path)
  ```
- Return `ExportResult` with path + file size via `fs::metadata`
- Unit test with mocked runner
- File: `src/tools/export.rs`

**Commit 35** `feat: implement export_step in tools/export.rs`
- Script:
  ```python
  import Part
  Part.export([doc.getObject(object_id).Shape], output_path)
  ```
- Unit test
- File: `src/tools/export.rs`

**Commit 36** `feat: implement export_obj in tools/export.rs`
- Script: `Mesh.export([obj], output_path)` with `.obj` extension
- Unit test
- File: `src/tools/export.rs`

**Commit 37** `feat: implement measure_distance in tools/measure.rs`
- `MeasureInput { object_id_a: String, object_id_b: String }`
- Script: `dist, _, _ = shape_a.distToShape(shape_b); result = dist`
- Return `DistanceResult { success: true, distance_mm: f64 }`
- Check both objects exist first
- Unit test
- File: `src/tools/measure.rs`

---

### Phase 8 — Sketcher (commits 38–42)

**Commit 38** `feat: implement create_sketch in tools/sketcher.rs`
- `Sketcher::SketchObject` added to doc
- Plane mapping: `XY` → default placement, `XZ` → rotate 90° around X, `YZ` → rotate 90° around Y
- Return sketch `object_id`
- Unit test
- File: `src/tools/sketcher.rs`

**Commit 39** `feat: implement add_rectangle in tools/sketcher.rs`
- `AddRectangleInput { sketch_id, x, y, width, height }`
- Script: four `Part.LineSegment` geometries → `sketch.addGeometry()`
- Return list of edge indices
- Unit test
- File: `src/tools/sketcher.rs`

**Commit 40** `feat: implement add_circle in tools/sketcher.rs`
- Script: `Part.Circle` via `sketch.addGeometry()`
- Return edge index
- Unit test
- File: `src/tools/sketcher.rs`

**Commit 41** `feat: implement extrude in tools/sketcher.rs`
- `ExtrudeInput { sketch_id, depth, symmetric }`
- Script: `PartDesign::Pad` linked to sketch
- Return solid `object_id`
- Unit test
- File: `src/tools/sketcher.rs`

**Commit 42** `feat: implement revolve in tools/sketcher.rs`
- `RevolveInput { sketch_id, axis, angle_deg }`
- Script: `PartDesign::Revolution`
- Axis mapping: `X` → `FreeCAD.Vector(1,0,0)` etc.
- Return solid `object_id`
- Unit test
- File: `src/tools/sketcher.rs`

---

### Phase 9 — MCP Server Wiring (commits 43–46)

**Commit 43** `feat: register primitives and placement tools in mcp_server.rs`
- Use `#[tool]` or equivalent rmcp macro for:
  `create_box`, `create_cylinder`, `create_sphere`, `create_cone`,
  `place_object`, `rotate_object`
- Each wrapper: deserialize JSON args → call sync tool fn → serialize to JSON
- Tool descriptions written for Claude (units, return values, how to use object_id)
- File: `src/mcp_server.rs`

**Commit 44** `feat: register boolean, query, and export tools in mcp_server.rs`
- Register: `boolean_union`, `boolean_cut`, `boolean_intersection`
- Register: `list_objects`, `get_object_info`, `get_document_stats`
- Register: `export_stl`, `export_step`, `export_obj`, `save_document`
- File: `src/mcp_server.rs`

**Commit 45** `feat: register sketcher and measure tools in mcp_server.rs`
- Register: `create_sketch`, `add_rectangle`, `add_circle`, `extrude`, `revolve`
- Register: `measure_distance`
- File: `src/mcp_server.rs`

**Commit 46** `feat: implement main.rs with full startup sequence`
- `Config::from_env()` → `config.validate()` → `ensure_workspace(&config)`
- Init tracing: `tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init()`
- Wrap config in `Arc<Config>`, construct `FreeCADMcpServer`
- `rmcp::serve_server(server, tokio::io::stdin(), tokio::io::stdout()).await?`
- File: `src/main.rs`

---

### Phase 10 — Polish, Docs, Ship (commits 47–50)

**Commit 47** `feat: add .env.example and claude_desktop_config.json`
- `.env.example`:
  ```
  FREECADCMD_PATH=/usr/bin/FreeCADCmd
  FREECAD_WORK_DIR=/tmp/freecad_workspace
  FREECAD_TIMEOUT=30
  RUST_LOG=freecad_mcp=info
  ```
- `claude_desktop_config.json`:
  ```json
  {
    "mcpServers": {
      "freecad": {
        "command": "/absolute/path/to/target/release/freecad-mcp",
        "env": {
          "FREECADCMD_PATH": "/usr/bin/FreeCADCmd",
          "FREECAD_WORK_DIR": "/tmp/freecad_workspace",
          "RUST_LOG": "freecad_mcp=info"
        }
      }
    }
  }
  ```
- Files: `.env.example`, `claude_desktop_config.json`

**Commit 48** `docs: write README.md`
- Full README — see the companion README.md
- File: `README.md`

**Commit 49** `chore: add Makefile with dev shortcuts`
- `make build`   → `cargo build --release`
- `make test`    → `cargo test`
- `make check`   → `cargo check && cargo clippy -- -D warnings`
- `make run`     → `RUST_LOG=debug cargo run`
- `make install` → `cp target/release/freecad-mcp ~/.local/bin/`
- `make clean`   → `cargo clean`
- File: `Makefile`

**Commit 50** `chore: clippy clean, release build verified, TASKS done`
- `cargo clippy -- -D warnings` passes with zero warnings
- `cargo build --release` succeeds
- `cargo test` all green
- Update TASKS.md: all items marked done
- Files: `Cargo.lock`, `TASKS.md`

---

## TASKS.md starter

```markdown
# TASKS

## Active
- [ ] Commit 1: init cargo project

## Done

## Blocked
```

---

## MISTAKES.md starter

```markdown
# MISTAKES
<!-- Add an entry the moment you catch a mistake — format: date, what happened, fix applied -->
```

---

## Tool description template — paste for every tool registration

```
Create a box in the active FreeCAD document.
All dimensions in millimetres. Returns object_id (e.g. "Box001") which you must
pass to boolean_cut, boolean_union, place_object, export_stl, or get_object_info
to use this object in subsequent operations. label is optional and only affects
the FreeCAD model tree display — it does not change object_id.
Returns: {"success": true, "object_id": "Box001", "label": "MyBox"}
On error: {"success": false, "error": "..."}
```

---

## FreeCAD script contract — every script must follow exactly

```python
import FreeCAD, Part, json, os, sys
DOC_PATH = "/tmp/freecad_workspace/session.FCStd"   # injected by ScriptBuilder

try:
    if os.path.exists(DOC_PATH):
        doc = FreeCAD.openDocument(DOC_PATH)
    else:
        doc = FreeCAD.newDocument("session")

    # --- tool logic here ---

    doc.recompute()
    doc.saveAs(DOC_PATH)

    # LAST LINE of stdout — always valid JSON
    print(json.dumps({"success": True, "object_id": obj.Name, "label": obj.Label}))

except Exception as e:
    print(json.dumps({"success": False, "error": str(e)}))
```

Rules:
- `print(json.dumps(...))` is always the **last line** stdout produces
- Always `doc.recompute()` before `doc.saveAs()`
- Never print anything after the JSON line
- Always wrap all logic in `try/except Exception`

---

## v1 acceptance test

These five calls chained autonomously by Claude = v1 complete:

```
1. create_box(200, 100, 50)             → "Box001"
2. create_cylinder(20, 60)              → "Cylinder001"
3. place_object("Cylinder001", 100, 50, 0)
4. boolean_cut("Box001", "Cylinder001") → "Cut001"
5. export_stl("Cut001")                 → "/tmp/freecad_workspace/Cut001_<ts>.stl"
```
