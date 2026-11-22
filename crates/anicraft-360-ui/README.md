# AniCraft 360 Studio UI

The anicraft-360-ui crate is the native egui panel library for AniCraft 360 Studio.

## Host visual compatibility

The crate targets egui 0.36, matching the audited AniCraft host dependency. It inherits the active egui style and visuals supplied by the host rather than defining a second theme, font system, spacing scale, icon library, or docking manager. The host should apply its current theme before invoking the panel functions.

## Panels provided

- Workspace identity and integration status header
- Character hierarchy
- Authored View Rail
- Discrete Turn Ruler (registered-view navigation only)
- Contextual Properties Inspector
- Character Asset Library
- Diagnostics

## Integration boundary

The host supplies a read-only StudioUiModel projection and retains StudioUiState as transient workspace state. The UI appends typed StudioUiAction intents. The host validates these against the active document, capability contract, and command registry before dispatch.

This crate does not create an application window or implement docking. It does not create a second stage, renderer, timeline, document model, VFS, persistence layer, or undo system. It performs no file I/O and implements neither Smart Controller evaluation nor Forma Deform pin deformation.

## Plugin-to-native rollout

The UI component library is intentionally independent of the host's private Rust modules. The first integration step is a host-provided panel extension point that can call these egui functions with a host-owned context. AniCraft's currently audited WASM plugin ABI is command-oriented and does not yet expose arbitrary native egui panels; therefore this crate is not installable as a UI plugin until that host capability exists.

Once the host exposes the panel extension point, the same crate can be linked into AniCraft as an in-process workspace component. The panel implementation and typed action boundary remain unchanged; only the host adapter is supplied by AniCraft.

## Build and test

Run: cargo test -p anicraft-360-ui
