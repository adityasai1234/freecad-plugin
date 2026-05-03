# MISTAKES
<!-- Add an entry the moment you catch a mistake — format: date, what happened, fix applied -->

## 2026-05-03 — stray src/ created in parent directory
Write tool used absolute path `/Users/medow/Documents/freecad-plugin/src/` (parent of freecad-mcp/) for some files instead of `.../freecad-mcp/src/`. Git restore recovered committed versions; stray directory removed.
Fix: verify absolute paths always include `freecad-mcp/` prefix when writing source files.

## 2026-05-03 — tracing version wrong in spec
Spec listed `tracing = "1"` but tracing crate is on 0.1.x series.
Fix: changed to `tracing = "0.1"` in Cargo.toml.
