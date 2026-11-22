# anicraft-360-ui

Host-embedded egui panels for AniCraft 360 Studio.

This crate is intentionally a UI component library, not a desktop application.
AniCraft owns the main window, docking/floating behavior, stage, timeline,
document model, rendering, VFS, persistence, undo, and command execution.

## Panels

- Character hierarchy
- Authored-view rail
- Contextual properties and validation
- Character asset library

## Integration model

The host supplies a read-only StudioUiModel projection and retains
StudioUiState as transient panel state. Panel functions append typed
StudioUiAction values when the user interacts. A host adapter validates
those intents and maps them to the corresponding studio.* CommandSpec
operations.

No panel performs file I/O, mutates persistent document data, evaluates
Smart Controllers, evaluates Forma Deform pins, or creates native windows.

The host should place each panel in its existing docking system. The existing
AniCraft stage and timeline remain authoritative and are not recreated here.

## Current limitation

The projection and action types are an initial plugin-side contract. They must
be reconciled with AniCraft's actual host APIs before live integration.
