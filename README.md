# AniCraft 360 Studio

**AniCraft 360 Studio** is a native AniCraft character-authoring workspace/plugin for modular character construction, symbol rigging, Smart Bones, Switch Groups, reusable character assets, and multi-view/360° character workflows.

It is not a second animation application, renderer, timeline, document model, or deformation engine.

## Native UI implementation

The reusable Rust UI library is in `crates/anicraft-360-ui`. The persistent, host-independent character domain model is in `crates/anicraft-360-model`. It targets egui 0.36 to match AniCraft and inherits the host's active visuals, spacing, typography, and interaction conventions.

The crate provides:
- A composed workspace entry point with Character Hierarchy, host-rendered Stage slot, Properties Inspector, Turn Ruler, and View Rail.
- Individual dockable panel functions for Character Hierarchy, View Rail, Turn Ruler, Properties, Asset Library, and Diagnostics.
- Read-only host projections, transient UI state, and typed semantic action intents.

AniCraft remains responsible for the actual Stage renderer, timeline, document model, selection authority, VFS/assets, persistence, undo/redo, command execution, and docking. The Studio crate creates no application window and no competing docking system.

**Plugin constraint:** AniCraft's currently audited WASM plugin ABI does not expose arbitrary native egui panels. A host-owned native panel extension point or bridge is required before this UI can be installed as a plugin. The same crate can then be linked into AniCraft as a built-in workspace component. See [the host UI integration contract](docs/HOST_UI_INTEGRATION.md).

## Character data model\n\nM1 introduces a versioned `CharacterDocument` model for character parts, authored view sets and turn angles, landmark correspondence, Smart Bone authoring references, Switch Groups, and host asset references. The model validates stable IDs and cross-references and supports validated JSON round trips. It is an extension payload, not a replacement project format; AniCraft still owns persistence, undo/redo, and host object resolution. See [the Character Data Model contract](docs/CHARACTER_DATA_MODEL.md).\n\n## Product responsibilities

- Modular symbol and character-part organization.
- Character rig setup and Smart Bone authoring workflows.
- Switch Groups for discrete artwork/state alternatives.
- Character asset library and reusable symbol/rig templates.
- Multi-view character sets, turn-angle organization, landmarks, correspondence, and turn validation.
- View-specific corrective setup, using Forma Deform for deformation behavior.

## Native host integration

The companion AniCraft fork integration mounts the Character, Smart Bone, and Switch Group panels in AniCraft's existing dock system. It reads the active document, Stage selection, armatures and Library, and dispatches edits through native commands (`bone.add`, `bone.set_limits`, `library.new_folder`, `library.move`, `element.swap_symbol`). AniCraft's central Stage and timeline remain authoritative; Studio creates no duplicate renderer or timeline. Smart Controller binding/evaluation is still a separate Forma Deform integration step.

## Relationship to Forma Deform

- **360 Studio** owns character semantics, Smart Bone authoring, Switch Groups, asset organization, views, turn angles, landmarks, and view-specific authoring.
- **Forma Deform** owns Smart Controller definitions/evaluation, target bindings, deformation algorithms, and corrective deformation.
- **AniCraft** owns shared host infrastructure: bones, document identity, timeline, rendering, persistence, undo/redo, command execution, evaluation lifecycle, and export.

Smart Bones reference canonical controller IDs; they do not duplicate the controller engine. Smart Controller binding evaluation and Forma Deform pin evaluation remain independent. No implicit output composition is introduced.

## Build and test

From the repository root:

```sh
cargo test -p anicraft-360-ui
cargo check -p anicraft-360-ui
```

See [DEVELOPMENT.md](DEVELOPMENT.md), [the UI specification](docs/UI_SPECIFICATION.md), and [the host UI integration contract](docs/HOST_UI_INTEGRATION.md), and [the Character Data Model](docs/CHARACTER_DATA_MODEL.md) for architecture, behavior, and integration constraints.
