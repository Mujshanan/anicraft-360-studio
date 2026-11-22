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
