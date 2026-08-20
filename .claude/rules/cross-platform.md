---
paths:
  - "crates/openlogi-hook/**"
  - "crates/openlogi-inject/**"
  - "crates/openlogi-hid/**"
---

# Platform / cfg-gated code — macOS-green is a trap

macOS-green proves **nothing** about `#[cfg(target_os = "linux")]` /
`windows` code. Recent agent failures that only showed up on CI Linux:

- Shadowing a crate-level constant with a local `const` of a different type
  (e.g. `LOGITECH_VENDOR_ID: u16` next to `use crate::LOGITECH_VENDOR_ID`
  which is `u32`) — E0255 / E0308, **only compiles on Linux**.
- Importing a name that only exists on another OS, or redefining one that
  master already exports from `lib.rs`.

When the diff touches any of:

- `crates/openlogi-hook/src/linux.rs` / `windows.rs`
- `crates/openlogi-inject/src/inject/linux.rs` / `windows.rs`
- `crates/openlogi-hid/src/channel/transport.rs` (has `#[cfg]` branches)
- any `#[cfg(target_os = …)]` block, in any crate

you MUST either:

1. Cross-check with devenv when available:
   `devenv tasks run openlogi:check-windows` (and any linux check the repo has), or
2. Manually re-read every changed cfg-gated file against **current master** for:
   - name collisions with existing `pub use` / `pub const` items
   - type mismatches (`u16` vs `u32`, `Option` arity, new enum fields)
   - call sites that gained args on master (e.g. `with_runtime`, `build_device_list`,
     `dispatch_action`) but the PR still uses the old signature

Do not claim "cross-platform green" without CI (or a local cross-lint) having
actually run those targets. `RUSTFLAGS=-D warnings` is global in CI — plain
warnings fail there too.
