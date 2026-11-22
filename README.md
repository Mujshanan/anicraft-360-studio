# AniCraft 360 Studio

Native character-authoring workspace for AniCraft, implemented as an embeddable Rust/egui component.

## Current implementation

The initial workspace shell provides:
- AniCraft-style compact top bar and contextual menus.
- Character hierarchy panel.
- Central Stage region delegated to the host renderer.
- Contextual Properties inspector.
- Bottom turn-angle control and registered-view rail.
- Explicit empty, unavailable, and host-renderer-pending states.
- Typed host commands and a versioned snapshot boundary.
- Mock-host unit tests for basic contract behavior.

## Architecture

This crate intentionally depends on `egui`, not AniCraft's application or document crates. The host mounts `StudioUi::show(&mut egui::Ui, &mut host)` inside its own dockable panel and implements `StudioHost` to:
1. provide a read-only `StudioSnapshot`;
2. execute `StudioCommand` through AniCraft's command/undo system;
3. render the real AniCraft Stage in the provided UI region.

The UI owns transient panel/filter/diagnostics state. AniCraft remains the owner of documents, symbols, bones, viewport transforms, rendering, persistence, undo/redo, assets, and timeline.

## Important plugin limitation

AniCraft's current WebAssembly plugin ABI supports commands, frame scripts, exporters, and easing functions. It does not expose arbitrary native egui panel registration or an embedded UI lifecycle. Therefore this crate is **plugin-ready as a native UI component**, but it is not currently installable through the existing `.wasm` plugin manager. A future host-side native extension point is required before it can appear as a dockable panel in the running application.

This repository contains the Studio UI and feature logic. Any required native panel registration, host adapter, or document-extension support belongs in the AniCraft host repository and must be implemented as a separate, explicitly reviewed integration change.

## Development

```sh
cargo test
cargo check
```

The crate uses egui 0.36 to match the current AniCraft host dependency. It is designed to be embedded in AniCraft's existing workspace, not to create a second standalone animation editor.
