# freecad-mcp

A Rust MCP server that gives Claude the ability to create, modify, query, and export
3D designs in FreeCAD — entirely through natural language.

```
You: "Make a 200×100×50mm box, drill a 20mm hole through the center, export it as STL"
Claude: [calls create_box → create_cylinder → place_object → boolean_cut → export_stl]
You: /tmp/freecad_workspace/Cut001_1714123456.stl ✓
```

No clicks. No FreeCAD GUI. Claude drives the whole thing.

---

## How it works

```
Claude Desktop / Claude Code
        │  MCP stdio
        ▼
freecad-mcp (Rust binary)
        │  JSON over stdout/stderr
        ▼
FreeCADCmd subprocess (headless Python)
        │
        ▼
session.FCStd  →  .stl / .step / .obj exports
```

The Rust binary never links against FreeCAD. It writes small Python scripts to a
temp file, runs them through `FreeCADCmd`, and parses the JSON output. FreeCAD's
full parametric engine does the geometry work. A single `session.FCStd` document
persists across all tool calls — objects accumulate and reference each other.

---

## Prerequisites

| Requirement | Version | Notes |
|---|---|---|
| Rust | 1.78+ | `curl https://sh.rustup.rs -sSf \| sh` |
| FreeCAD | 0.21+ | Must include `FreeCADCmd` headless binary |
| OS | Linux / macOS / Windows | All supported |

### Finding FreeCADCmd

**Linux (apt)**
```bash
sudo apt install freecad
which FreeCADCmd   # → /usr/bin/FreeCADCmd
```

**Linux (AppImage)**
```bash
# FreeCADCmd is inside the AppImage
./FreeCAD.AppImage --appimage-extract
# binary at: squashfs-root/usr/bin/FreeCADCmd
```

**macOS (DMG)**
```bash
/Applications/FreeCAD.app/Contents/MacOS/FreeCADCmd
```

**Windows**
```
C:\Program Files\FreeCAD 0.21\bin\FreeCADCmd.exe
```

---

## Installation

```bash
# 1. Clone
git clone https://github.com/youruser/freecad-mcp
cd freecad-mcp

# 2. Configure
cp .env.example .env
# Edit .env — set FREECADCMD_PATH to your FreeCADCmd binary

# 3. Build
cargo build --release

# 4. Verify
FREECADCMD_PATH=/usr/bin/FreeCADCmd ./target/release/freecad-mcp --version
```

---

## Claude Desktop setup

Add this to your Claude Desktop config file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows:** `%APPDATA%\Claude\claude_desktop_config.json`
**Linux:** `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "freecad": {
      "command": "/absolute/path/to/freecad-mcp/target/release/freecad-mcp",
      "env": {
        "FREECADCMD_PATH": "/usr/bin/FreeCADCmd",
        "FREECAD_WORK_DIR": "/tmp/freecad_workspace",
        "RUST_LOG": "freecad_mcp=info"
      }
    }
  }
}
```

Restart Claude Desktop. You should see **freecad** in the connected tools list.

---

## Claude Code setup

```bash
# Add to your project's CLAUDE.md or run interactively:
claude mcp add freecad \
  --command /absolute/path/to/target/release/freecad-mcp \
  --env FREECADCMD_PATH=/usr/bin/FreeCADCmd \
  --env FREECAD_WORK_DIR=/tmp/freecad_workspace
```

---

## Environment variables

| Variable | Default | Description |
|---|---|---|
| `FREECADCMD_PATH` | `/usr/bin/FreeCADCmd` | Absolute path to FreeCADCmd binary |
| `FREECAD_WORK_DIR` | `/tmp/freecad_workspace` | Directory for session doc and exports |
| `FREECAD_TIMEOUT` | `30` | Seconds before a FreeCAD subprocess is killed |
| `RUST_LOG` | `freecad_mcp=info` | Tracing filter — use `debug` for verbose output |

---

## Available tools

All units are **millimetres** unless noted. Every tool returns
`{"success": true, ...}` on success or `{"success": false, "error": "..."}` on failure.

### Primitives

| Tool | Parameters | Returns |
|---|---|---|
| `create_box` | `length, width, height, label?` | `object_id` |
| `create_cylinder` | `radius, height, label?` | `object_id` |
| `create_sphere` | `radius, label?` | `object_id` |
| `create_cone` | `radius1, radius2, height, label?` | `object_id` |

### Placement

| Tool | Parameters | Returns |
|---|---|---|
| `place_object` | `object_id, x, y, z, yaw?, pitch?, roll?` | updated placement |
| `rotate_object` | `object_id, yaw, pitch, roll` | updated placement |

### Boolean Operations

| Tool | Parameters | Returns |
|---|---|---|
| `boolean_union` | `base_id, tool_id, label?` | `object_id` of fused solid |
| `boolean_cut` | `base_id, tool_id, label?` | `object_id` of cut solid |
| `boolean_intersection` | `base_id, tool_id, label?` | `object_id` of intersection |

### Sketch & Extrude

| Tool | Parameters | Returns |
|---|---|---|
| `create_sketch` | `plane` (`XY`\|`XZ`\|`YZ`) | `sketch_id` |
| `add_rectangle` | `sketch_id, x, y, width, height` | edge indices |
| `add_circle` | `sketch_id, cx, cy, radius` | edge index |
| `extrude` | `sketch_id, depth, symmetric?` | `object_id` of solid |
| `revolve` | `sketch_id, axis` (`X`\|`Y`\|`Z`), `angle_deg` | `object_id` of solid |

### Query & Inspect

| Tool | Parameters | Returns |
|---|---|---|
| `list_objects` | — | array of `{id, label, type_id}` |
| `get_object_info` | `object_id` | volume mm³, surface area mm², bounding box |
| `get_document_stats` | — | object count, total volume, units |

### Export

| Tool | Parameters | Returns |
|---|---|---|
| `export_stl` | `object_id, filename?` | file path, size bytes |
| `export_step` | `object_id, filename?` | file path, size bytes |
| `export_obj` | `object_id, filename?` | file path, size bytes |
| `save_document` | — | session doc path |

### Measure

| Tool | Parameters | Returns |
|---|---|---|
| `measure_distance` | `object_id_a, object_id_b` | minimum distance mm |

---

## Example conversations

### A box with a hole

```
You: Create a 150×80×30mm aluminum bracket. Put a 12mm hole centered at x=75, y=40.
     Export as STEP.

Claude:
  → create_box(150, 80, 30, "Bracket")         → Box001
  → create_cylinder(6, 40, "Hole")             → Cylinder001
  → place_object("Cylinder001", 75, 40, -5)
  → boolean_cut("Box001", "Cylinder001")        → Cut001
  → export_step("Cut001", "bracket.step")
  → /tmp/freecad_workspace/bracket.step (4.2 KB)
```

### Inspecting a design

```
You: What's the volume of Cut001 and does the hole go all the way through?

Claude:
  → get_object_info("Cut001")
  → Volume: 347,820 mm³. Bounding box Z: 0–30mm. The cylinder was placed at
    z=-5 with height 40mm, so yes — it exits the top face with 5mm clearance.
```

### Sketch-based workflow

```
You: Make an L-bracket by sketching on the XZ plane. Flange: 60×40mm,
     web: 60×60mm, thickness 5mm. Extrude 80mm.

Claude:
  → create_sketch("XZ")                         → Sketch001
  → add_rectangle("Sketch001", 0, 0, 60, 5)    → flange bottom
  → add_rectangle("Sketch001", 0, 0, 5, 65)    → web vertical
  → extrude("Sketch001", 80)                    → Pad001
  → export_stl("Pad001", "l_bracket.stl")
```

---

## Development

```bash
# Run tests (no FreeCAD needed — subprocess is mocked)
cargo test

# Lint
cargo clippy -- -D warnings

# Debug a single tool call manually
echo '{"method":"tools/call","params":{"name":"create_box","arguments":{"length":100,"width":50,"height":25}}}' \
  | FREECADCMD_PATH=/usr/bin/FreeCADCmd cargo run

# Watch mode
cargo watch -x check
```

### Adding a new tool

1. Add input/output types to `src/tools/mod.rs`
2. Implement the function in the relevant `src/tools/*.rs` file
3. Write the FreeCAD Python script body
4. Add unit test with mocked runner
5. Register in `src/mcp_server.rs` with a Claude-oriented description
6. Run `cargo test && cargo clippy`

---

## Architecture decisions

**Why Rust?**
The MCP SDK (`rmcp`) is production-quality in Rust. `tokio` handles concurrent tool
calls cleanly. One static binary with no runtime deps.

**Why subprocess isolation?**
FreeCAD ships its own Python interpreter (often 3.10 or 3.11) bundled inside the
app. Linking against it from Rust would require matching exact Python ABI versions.
Subprocess isolation means FreeCAD's Python and our Rust binary are completely
independent — no conflicts ever.

**Why a persistent session document?**
Object IDs like `Box001` are only meaningful within a single `.FCStd` document.
If each tool call opened a fresh document, Claude couldn't reference objects across
calls. The session doc is the shared state that makes multi-step design workflows
possible.

**Why JSON on stdout?**
FreeCAD Python scripts can print arbitrary debug output. We always parse the
**last non-empty line** of stdout as JSON. Scripts must never print anything after
the result JSON line. The `ScriptBuilder::finish()` method enforces this contract.

---

## Known limitations

- **No live preview.** Headless FreeCAD cannot render to screen. Export STL and
  open in your viewer of choice.
- **Single session.** One `session.FCStd` per server process. To start fresh,
  delete the file or set a new `FREECAD_WORK_DIR`.
- **FreeCAD version sensitivity.** Some `Part::` object types differ between
  FreeCAD 0.20 and 0.21. This project targets 0.21+.
- **No constraint solving feedback.** Sketcher constraints are applied but
  over/under-constrained states are not yet detected and reported to Claude.
- **Windows paths.** Paths with spaces in `FREECADCMD_PATH` on Windows must be
  quoted in the config. Use forward slashes or escaped backslashes in JSON.

---

## Roadmap

- [ ] `render_preview(object_id)` → base64 PNG via FreeCAD's offscreen renderer
- [ ] `parametric_update(object_id, param, value)` — live constraint updates
- [ ] `import_step(file_path)` — load external STEP files
- [ ] SSE transport for remote MCP server mode
- [ ] Multi-document support
- [ ] Constraint validation feedback to Claude

---

## License

MIT
