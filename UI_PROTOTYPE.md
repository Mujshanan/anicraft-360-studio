# AniCraft 360 Studio UI prototype

This repository currently contains a standalone **egui desktop prototype** for the proposed 360 Studio workspace.

## Run

Requires the Rust toolchain used by AniCraft (Rust 1.95+):

```sh
cargo run
```

## What is interactive

- Select character parts, Smart Bone and Switch Group rows in the hierarchy.
- Switch between the five authored turn views.
- Adjust the turn-angle control.
- Toggle rig, landmark and silhouette overlays.
- Play/pause the illustrative timeline strip.
- Inspect contextual properties.

The character drawing and timeline are sample UI data. They are not connected to an AniCraft document, host timeline, persistence, undo stack, or Forma Deform.

## Integration boundary

AniCraft's current plug-in ABI runs one-shot WebAssembly commands; it does not expose a persistent dockable panel API. This prototype deliberately uses the same Rust/egui UI technology as AniCraft and follows its graphite/cobalt visual conventions, but it is not yet an installable AniCraft plug-in. The next integration step requires a host-side workspace/panel extension point (or a built-in workspace contribution in AniCraft), plus host-owned document state and command bindings.

See [UX_SPEC.md](docs/UX_SPEC.md) and [FORMA_DEFORM_CONTRACT.md](docs/FORMA_DEFORM_CONTRACT.md) for the intended product and integration contracts.

## In-process AniCraft integration (new)

The optional `anicraft-host` feature adds an embeddable panel adapter in `src/anicraft_host.rs`. It accepts AniCraft's existing `&mut anicraft_engine::Session`; it does not create or load a parallel document.

The host adapter reads:
- `Session::doc`, `scene`, `frame`, `layer`, `frame_count()` and `active_layer()` for document and timeline state;
- `Session::selection` and `Session::hit_test()` for the actual selection;
- `Session::display_list()` plus `anicraft_render::render()` for the real evaluated stage.

Timeline navigation and keyframe creation call `Session::execute()`, so they use AniCraft's command registry and undo model. The adapter is exposed as a library module when the feature is enabled.

### Host-side wiring still required

This adapter must be compiled into AniCraft's desktop shell (the host owns the live Session). Add this repository as a dependency of the AniCraft app, enable `anicraft-host`, then call `anicraft_360_studio::anicraft_host::show(ui, &mut app.s)` from the chosen workspace/panel draw location. AniCraft's current panel registry does not yet have a third-party extension hook, so this is a host integration change, not something a separately launched Studio process can perform. The standalone `cargo run` binary remains a visual sandbox; it does not magically attach to an already-running AniCraft instance.

Build the adapter feature with `cargo check --features anicraft-host` after the host and Studio are on compatible AniCraft revisions. The upstream engine/render crates are Git dependencies and the full feature build has not yet been validated in this environment.
