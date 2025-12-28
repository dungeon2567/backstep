# backstep

[![CI](https://github.com/dungeon2567/backstep/actions/workflows/ci.yml/badge.svg)](https://github.com/dungeon2567/backstep/actions/workflows/ci.yml)

backstep is a performance-focused ECS library for Rust.
- Hierarchical storage (Storage → Page → Chunk) with presence/fullness/changed bitmasks
- Per-tick rollback snapshots that track created/changed/removed states
- Dependency-driven scheduler that builds parallelizable wavefronts
- Macro-authored systems using `View`/`ViewMut` for fast, scoped access

