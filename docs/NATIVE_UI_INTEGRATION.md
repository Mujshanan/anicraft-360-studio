# Native UI integration boundary (v1)

**Status:** initial implementation contract; not yet implemented by the AniCraft host.

## Purpose

Allow AniCraft 360 Studio to render as a native dockable workspace component while remaining independently developed in this repository.

## Ownership

- **Studio:** character hierarchy presentation, view rail, turn-angle inspection UI, Smart Bone authoring UI, Switch Group authoring UI, character-specific diagnostics presentation.
- **AniCraft:** document identity and storage, symbol/bone identity, viewport and Stage renderer, selection authority, command execution, undo/redo, timeline, VFS/assets, notifications, workspace docking and preferences.
- **Forma Deform:** Smart Controller definitions/evaluation and deformation behavior. Studio stores references and presents supported capabilities; it does not implement controller or deformation math.

## Contract surface

The Rust crate exposes:
- `StudioHost::snapshot() -> StudioSnapshot`
- `StudioHost::dispatch(StudioCommand) -> Result<(), String>`
- `StudioHost::show_stage(&mut egui::Ui)`
- `STUDIO_HOST_API_MAJOR`

The host adapter must translate its native document/session state into the snapshot and route persistent commands through the host's command/undo path. The Stage callback must use AniCraft's renderer and viewport transforms; Studio must not draw a competing document renderer.

## Versioning and compatibility

- The host checks `STUDIO_HOST_API_MAJOR` before mounting the panel.
- Breaking contract changes increment the major version.
- Additive snapshot fields and commands require an explicit minor-version policy before host adoption.
- Unknown commands must fail explicitly; no silent fallback or simulated success.
- Snapshot data is read-only and must describe one coherent document/session revision.
- The host should provide stable IDs, not display names, as command targets.

## Current limitation

The host's existing WASM plugin runtime is sandboxed and does not permit native UI imports. The current plugin kinds are command, frame-script, exporter, and ease. This contract is for a future native in-process extension point or direct built-in integration; it does not claim compatibility with the current WASM plugin ABI.

## Deferred until host integration

- Persistent document extension schema and migration.
- Real symbol/bone selection and command adapters.
- Dockable panel registration and workspace persistence.
- Host viewport fit/pan/zoom controls.
- Asset browser/import operations.
- Forma Deform controller discovery and preview.
- Undo/redo and save/load integration tests.
