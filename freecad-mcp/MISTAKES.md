# MISTAKES
<!-- Add an entry the moment you catch a mistake — format: date, what happened, fix applied -->

## 2026-05-03 — tracing version wrong in spec
Spec listed `tracing = "1"` but tracing crate is on 0.1.x series.
Fix: changed to `tracing = "0.1"` in Cargo.toml.
