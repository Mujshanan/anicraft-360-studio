# Host UI integration contract (draft)

**Status:** implementation boundary proposal; not a host API and not an authorization to modify AniCraft.

## 1. Purpose

This contract defines the minimum seam between the AniCraft 360 Studio panel library and the AniCraft host. It keeps Studio UI code reusable while the host remains authoritative for application lifecycle and document operations.

## 2. UI crate contract

The `anicraft-360-ui` crate:

- targets the host's egui version (currently 0.36);
- receives immutable `StudioUiModel` projections;
- receives mutable `StudioUiState` containing transient UI state only;
- emits typed `StudioUiAction` intents;
- draws inside a host-provided `egui::Ui` and inherits the host's active style;
- exposes individual dockable panel functions and a composed `show_workspace` entry point.

The host may mount the composed workspace in one dock tab, or mount the individual panels in separate existing AniCraft docks. The Stage callback passed to `show_workspace` must draw AniCraft's actual Stage; the crate contains no competing renderer.

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

The audited AniCraft plugin system is a sandboxed WebAssembly ABI for commands, frame scripts, exporters, and easing functions. It does not currently expose arbitrary native egui panel registration. A plugin-first UI rollout therefore requires a host-owned extension point or a native bridge. The Studio crate is ready to be embedded behind such an extension point, but it is not installable through the existing WASM plugin ABI alone.

## 5. Native inclusion

When AniCraft provides the extension point, it can depend on this crate directly and call the same panel functions from its workspace panels. No UI rewrite is required. The host adapter remains the only host-specific layer. The same crate can also be linked directly as a built-in workspace component.

## 6. Semantic guardrails

- The View Rail and Turn Ruler select registered views only until a verified interpolation capability exists.
- Smart Bone authoring references canonical AniCraft bone IDs and Forma Deform ControllerIds.
- Smart Controller evaluation remains owned by Forma Deform.
- Forma Deform pin evaluation remains independent; the UI does not combine outputs or evaluators.
- Preview results remain transient and clearly labeled.
- AniCraft's existing timeline remains the only timeline.

## 7. Native host adapter (implementation increment)

The companion AniCraft integration branch `feat/anicraft-360-studio-native-integration` mounts three Studio panels in the host's existing dock system:

- **360 Studio** projects the active scene's visible symbols, shapes, groups, and native selection into the Character Hierarchy. Selecting a hierarchy item updates AniCraft's actual `Session.selection` and the Stage reflects it on the next frame.
- **Smart Bones** projects armatures through the existing `bone.info` command. Creating a Smart Bone authors an AniCraft armature bone through `bone.add`; joint limits use `bone.set_limits`. All edits therefore participate in the host's command validation, undo history, and live Stage rendering.
- **Switch Groups** use AniCraft Library folders named under `Switch Groups`. A group's symbol assets are its discrete options; adding options uses `library.move`, and activating one on the current Stage selection uses `element.swap_symbol`.

The host's central Stage and bottom timeline remain the only Stage and timeline. Studio panels are docked beside Properties and read the same active `Session`, document, selection, playhead, and Library. There is no duplicate renderer or document state.

### Current boundary

The Smart Bone panel currently authors native armature bones and limits. It does **not** create or evaluate Forma Deform Smart Controllers; the `ControllerId` field is reserved as a reference boundary and remains empty until the Forma Deform binding API is integrated. Pin deformation and controller evaluation remain independent.

Switch Groups currently provide folder-backed option organization and symbol swapping. They do not introduce a second timeline, continuous interpolation, or custom evaluator.

The host integration depends on a pinned commit of the Studio UI crate. Update that pin only after the Studio branch has a reviewed commit, and keep both pull requests draft until the cross-repository API is reviewed together.
