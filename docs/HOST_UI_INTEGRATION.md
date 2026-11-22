# Host UI integration contract (draft)

**Status:** implementation boundary proposal; not a host API and not an authorization to modify AniCraft.

## 1. Purpose

This contract defines the minimum seam between the AniCraft 360 Studio panel library and the AniCraft host. It keeps Studio UI code reusable while the host remains authoritative for application lifecycle and document operations.

## 2. UI crate contract

The anicraft-360-ui crate:
- targets the host's egui version;
- receives immutable StudioUiModel projections;
- receives mutable StudioUiState;
- emits typed StudioUiAction intents;
- draws inside a host-provided egui::Ui and inherits the host's active style.

It must not access AniCraft private modules or execute commands directly.

## 3. Host adapter responsibilities

The host adapter must:
1. Construct the current model projection from the active document and Studio extension data.
2. Supply capability flags based on verified APIs, not plugin presence or version strings.
3. Retain transient UI state for the workspace lifetime.
4. Place panel functions in the existing AniCraft docking/workspace system.
5. Validate each action against current document identity, stable object identity, permissions, and capability state.
6. Dispatch accepted persistent operations through host commands/transactions so undo/redo remains host-owned.
7. Return updated model projections and operation status on subsequent frames.
8. Keep document mutation, persistence, file I/O, rendering, and evaluation outside the UI crate.

## 4. Plugin rollout constraint

The audited AniCraft plugin system is a sandboxed WebAssembly ABI for commands, frame scripts, exporters, and easing functions. It does not currently expose arbitrary native egui panel registration. A plugin-first UI rollout therefore requires a host-owned extension point or a native bridge. That host change is separate from this Studio repository.

## 5. Native inclusion

When AniCraft provides the extension point, it can depend on this crate directly and call the same panel functions from its workspace panels. No UI rewrite is required. The host adapter remains the only host-specific layer.

## 6. Semantic guardrails

- The View Rail and Turn Ruler select registered views only until a verified interpolation capability exists.
- Smart Bone authoring references canonical AniCraft bone IDs and Forma Deform ControllerIds.
- Smart Controller evaluation remains owned by Forma Deform.
- Forma Deform pin evaluation remains independent; the UI does not combine outputs or evaluators.
- Preview results remain transient and clearly labeled.
