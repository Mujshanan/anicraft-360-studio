# AniCraft 360 Studio — Development and Architecture

**Project role:** native AniCraft character-authoring workspace/plugin.  
**Status:** native egui 0.36 panel library and first versioned character-domain model implemented on feature branches; host persistence adapter and complete native integration remain in progress.  
**Scope:** modular character/symbol rigging, Smart Bones, Switch Groups, asset library workflows, and multi-view/360° character authoring.

## 1. Product definition

AniCraft 360 Studio answers: **how is a character assembled, controlled, organized, and represented across its authored views?**

It owns character-specific authoring concepts:
- Modular character parts and symbol relationships.
- Smart Bone authoring and mapping to controller identities.
- Switch Groups and discrete artwork alternatives.
- Character asset-library organization and reusable character templates.
- Multi-view character sets, turn angles, view correspondence, landmarks, and turn validation.
- View-specific corrective intent and references to deformation targets.

It does not own general animation infrastructure or deformation algorithms. AniCraft remains the host application; Forma Deform remains the deformation tool.

## 2. Accepted ownership boundaries

| System | Owns | Must not own |
|---|---|---|
| AniCraft | Document, stable IDs, symbols/instances as host primitives, bones, timeline, rendering, VFS/assets infrastructure, persistence, undo, commands, evaluation lifecycle, export | Character-specific 360 authoring policy |
| 360 Studio | Character organization, Smart Bone authoring workflow, Switch Groups, view sets, turn-angle model, view correspondence, landmarks, turn validation, character templates | General renderer, timeline, undo, VFS, duplicate bone engine, deformation solver |
| Forma Deform | Smart Controller definitions/mapping/preview, bindings, deformation math, corrective deformation, deformation diagnostics | Symbol library, Switch Groups, character-view semantics, second document model |

Smart Bones in 360 Studio reference canonical ControllerIds and define character-specific actions/relationships. Forma Deform owns the Smart Controller engine and controller evaluation semantics. This separation prevents two competing implementations of controllers.

## 3. Feature allocation matrix

**Priority:** P0 = foundation; P1 = core workflow; P2 = advanced; P3 = later extension.  
**Disposition:** Own = implement here; Consume = use host/plugin capability; Adapter = bridge; Research = investigate before commitment.

| ID | Feature | Priority | 360 Studio disposition | Boundary |
|---|---|---:|---|---|
| S01 | Multi-view character workspace | P0 | Own | Character view organization and active-view authoring |
| S02 | Turn-angle model and turn direction | P0 | Own | Store angle/view semantics; AniCraft supplies timeline time |
| S03 | View/keyform registration | P0 | Own | Link authored artwork/poses to view angles |
| S04 | View correspondence | P1 | Own | Landmark correspondence between authored views |
| S05 | Silhouette continuity checks | P2 | Own | Analyze authored view sequence; no renderer duplication |
| S06 | Landmark and anchor management | P0 | Own | Stable character-view landmark IDs; expose target references to Forma Deform |
| S07 | Intermediate-view generation assistance | P2 | Own workflow / Consume deformation | Request Forma Deform evaluation; do not implement a second deformer |
| S08 | Volume and proportion consistency checks | P2 | Own | Character/view validation and artist-facing diagnostics |
| S09 | View-specific rig configuration | P1 | Own configuration / Consume bones | AniCraft owns bone representation and transforms |
| S10 | View-specific corrective shapes | P1 | Own authoring references / Consume Forma Deform | Store view-specific intent; Forma Deform owns deformation |
| S11 | Turn-sheet generation | P1 | Own workflow / Consume export | Use AniCraft rendering/export |
| S12 | Turn validation | P1 | Own | Missing views, landmark mismatch, discontinuities, proportion warnings |
| S13 | Automatic view/mesh preparation assistance | P3 | Own workflow / Consume algorithms | Use AniCraft asset data and Forma Deform mesh utilities |
| S14 | Character-view library and turn templates | P2 | Own character templates / Consume host library | AniCraft owns VFS and persistence |
| C01 | Canonical bone identity | P0 | Consume | AniCraft stable bone IDs |
| C02 | Bone transforms and coordinate spaces | P0 | Consume | AniCraft transform conventions |
| C03 | Transform constraints and inheritance | P1 | Consume | AniCraft rig/constraint system |
| C06 | Pose-set and keyform interpolation | P1 | Consume / Adapter | Reuse Forma Deform mapping where appropriate; view data remains 360-owned |
| C07 | Corrective poses / blend shapes | P1 | Consume | Forma Deform owns deformation evaluation |
| C08 | Smart Bone action mapping | P0 | Own authoring / Adapter | Smart Bone action references canonical ControllerIds |
| C09 | Slider/joystick controls | P2 | Consume | Forma Deform owns controller models |
| C10 | Multiple target bindings | P1 | Consume | Forma Deform binding system |
| C11 | Deterministic controller dependency evaluation | P1 | Consume | Forma Deform evaluates; AniCraft schedules |
| C12 | Deformation target abstraction | P1 | Consume / Adapter | Forma Deform target contract |
| C13 | Pin/vector/mesh deformation | P1 | Consume | Forma Deform and AniCraft Asset Warp |
| C14 | Deformation diagnostics | P1 | Consume / Present | Forma Deform validates deformation; 360 Studio presents view-context issues |
| C15 | Pure, repeatable evaluation | P0 | Require | Both plugin-facing calculations must be deterministic; host owns lifecycle |
| E07 | Pose capture, mirroring and libraries | P2 | Own character/view presets | Do not duplicate general deformation preset logic |
| E08 | Character-turn mapping | P0 | Own | Core 360 Studio responsibility |
| E09 | Mesh tracing and automatic mesh generation | P3 | Research / Adapter | No second geometry or asset system |
| E11 | Reusable deformation presets | P2 | Consume | Forma Deform owns deformation presets; 360 Studio stores references |

## 4. UI proposal and interaction model

The workspace should feel like a specialized AniCraft workspace, not a separate app.

### 4.1 Main regions

- **Character hierarchy:** symbols, body parts, nested symbol instances, rig groups, Smart Bones, and Switch Groups.
- **Stage:** active view, selection, rig controls, landmarks, silhouette guides, and correction overlays.
- **View rail:** authored views ordered by turn angle. Missing or unvalidated views receive clear status indicators.
- **Turn ruler:** numeric angle and view position. Scrubbing this control selects/requests a view state; it must not pretend that a live interpolation engine exists until implemented.
- **Properties:** contextual inspectors for symbols, Smart Bones, Switch Groups, view registration, correspondence, and validation.
- **Asset Library:** reusable symbols, parts, turn templates, rig presets, and character configurations.
- **Timeline:** AniCraft's existing timeline/playhead only.

### 4.2 Smart Bone workflow

1. Select a canonical AniCraft bone.
2. Create or select a Smart Bone definition in 360 Studio.
3. Author the action/view relationship and its trigger range.
4. Reference a canonical ControllerId from Forma Deform; do not duplicate controller math.
5. Associate affected symbols or view-specific targets.
6. Preview through supported on-demand capabilities.
7. Persist the authored definition through a host-owned document extension.
8. Live playback is enabled only after AniCraft provides a verified evaluation hook.

### 4.3 Switch Group workflow

- Create a named group of mutually exclusive symbol/artwork alternatives.
- Assign stable option IDs and a default state.
- Associate options with relevant views or character states where useful.
- Expose current option in the Properties panel and character hierarchy.
- Keep rendering, symbol instance identity, document storage, and undo in AniCraft.

### 4.4 Multi-view workflow

1. Create a character view set and define turn direction/angle convention.
2. Register front, three-quarter, side, back-three-quarter, and back views.
3. Mark corresponding landmarks across views.
4. Review silhouette and proportion consistency.
5. Add view-specific corrections and reference Forma Deform targets when deformation is needed.
6. Generate turn sheets through AniCraft export facilities.

### 4.5 UI state and persistence

Transient selection, hover, active tool, and preview overlays remain UI state. Authored view sets, Smart Bone definitions, Switch Group configuration, landmark correspondence, and asset references are persistent document data and require a versioned AniCraft-owned extension boundary. Do not create a second project file format or VFS.

## 5. Code-reuse and integration matrix

| Capability | Source/owner | Reuse mode in 360 Studio | Decision |
|---|---|---|---|
| Bone IDs, transforms, hierarchy | AniCraft | Host API | Consume; do not fork |
| Symbol instances and nested timelines | AniCraft | Host API | Consume; do not recreate |
| Timeline, rendering, export, undo | AniCraft | Host API | Consume; no duplicate implementation |
| Smart Controller model/mapping | Forma Deform | Stable ControllerId contract | Consume; 360 Studio owns Smart Bone authoring only |
| Pin/vector/mesh deformation | Forma Deform / AniCraft Asset Warp | Target and evaluation contracts | Consume; no second deformation engine |
| Switch Group state model | AniCraft primitives + 360 Studio policy | Character-specific model using host symbols | Implement only the missing character-level grouping policy |
| Multi-view registration and turn-angle mapping | 360 Studio | Native implementation | Own |
| Landmark correspondence and view validation | 360 Studio | Native implementation | Own |
| Pose mirroring utilities | Fanimate (reference) | Algorithmic reference; narrow port possible | Verify source license, dependency graph and behavior |
| Character-turn data | LibreToon (reference) | Data-model and algorithmic reference | Compare schema; do not copy full rig/document system |
| Multi-axis keyform interpolation | Aether Canvas (reference) | Consume Forma Deform if adopted there | Do not fork into 360 Studio; license not confirmed |
| RBF / ARAP / advanced mesh deformation | OSATURA / OpenToonz | Consume through Forma Deform if selected | No direct implementation in 360 Studio |

## 6. External repository reuse notes

These are research references, not dependencies or approved code imports.

| Repository | Relevant material | Potential use | License caution |
|---|---|---|---|
| [Fanimate](https://github.com/toonittmedia/fanimate) | Smart Bone actions, pose capture/mirroring, stable bone references | Study Smart Bone/action workflow and pose operations | Repository metadata reported Apache-2.0; verify exact file notices |
| [LibreToon](https://github.com/alvillafm/LibreToon) | Character-turn data, vector rigs, weight interpolation | Study view/turn data representation and correspondence | Repository metadata reported MIT; verify current license |
| [Aether Canvas](https://github.com/syuumaimikan/Aether-Canvas) | Parameter/keyform grid, additive corrective shapes, drivers | Reference for view-keyform data and interpolation; prefer consuming Forma Deform implementation | License not confirmed; no code copying |
| [OpenBone](https://github.com/UXCentury/open-bone) | Constraints, transforms, pose solver | Reference only if AniCraft rig API has a verified gap | Repository metadata reported MIT |
| [OSATURA](https://github.com/chiuta/osatura) | Rigging, mesh prep, soft selection, RBF warp | Indirectly through Forma Deform; not a 360 Studio dependency | MPL-2.0 core; other components AGPL-3.0-or-later |
| [OpenToonz](https://github.com/opentoonz/opentoonz) | Plastic/ARAP deformation | Indirectly through Forma Deform if benchmarked and selected | BSD-3-Clause project code; third-party terms vary |

**Reuse policy:** prefer AniCraft host APIs and the sibling Forma Deform contract. Do not vendor a complete animation framework, document model, renderer, timeline, or asset library. Before copying external code, verify exact file-level license, notices, dependencies, tests, and whether the behavior duplicates host functionality.

## 7. Milestones

| Milestone | Scope | Exit criteria |
|---|---|---|
| M0 — Host contract map | Symbol, bone, timeline, VFS, undo, command, extension storage, evaluator APIs | Supported and missing capabilities documented with source references |
| M1 — Character model | `anicraft-360-model`: CharacterDocument, CharacterPart, ViewSet, CharacterView, LandmarkTrack, SmartBoneDefinition, SwitchGroup and AssetReference | Domain model, stable typed IDs, validation and JSON round-trip implemented; host persistence adapter/migrations remain |
| M2 — Workspace shell | Hierarchy, stage context, view rail, properties, asset panel | Native AniCraft panels; no duplicate timeline or VFS |
| M3 — Symbol and Switch Groups | Organize character parts and discrete alternatives | Stable option identity, persistence and undo tests |
| M4 — Smart Bone authoring | Bind canonical bones/actions to ControllerIds | Forma Deform owns controller semantics; preview limitations explicit |
| M5 — Multi-view authoring | Register views, landmarks, correspondence and turn sheets | Deterministic view ordering and validation |
| M6 — Corrective integration | View-specific targets and Forma Deform requests | Independent evaluator contracts; no implicit composition |
| M7 — Assisted generation | Intermediate views, mesh preparation, silhouette/proportion analysis | Benchmarked and opt-in; artist retains control |
| M8 — Playback integration | Live view/controller evaluation only after host hook exists | Deterministic scrub/playback, cache invalidation and undo semantics verified |

## 8. Testing requirements

- Schema validation and migration for view sets, landmarks, Smart Bones, Switch Groups, and asset references.
- Stable identity tests across reorder, rename, duplicate, and save/load.
- Turn-angle ordering and interpolation boundary tests.
- Correspondence tests for missing, duplicate, or inconsistent landmarks.
- Switch Group exclusivity/default-state tests.
- Smart Bone mapping tests proving controller definitions are referenced rather than duplicated.
- Integration tests for AniCraft undo/redo, persistence, symbols, and host timeline.
- UI tests for view switching, selection, missing-view diagnostics, and asset-library navigation.
- Deterministic preview tests; no live-playback claim before a verified host evaluator exists.


## 9. Current UI implementation status

The Studio repository contains the host-independent native panel crate at `crates/anicraft-360-ui`. It targets egui 0.36, matching the audited AniCraft host dependency, and inherits the active egui style/visuals instead of defining a competing theme or docking system.

Implemented panel components on the UI implementation branch:
- Workspace identity and integration status header.
- Character hierarchy projection and selection intents.
- Authored View Rail with explicit authored/generated/preview and validation states.
- Discrete Turn Ruler navigation between registered views only.
- Contextual Properties Inspector with canonical controller/target references.
- Character Asset Library presentation and capability-gated open actions.
- Diagnostics presentation and host-navigation intents.

The UI crate consumes immutable host/domain projections, stores only transient UI state, and emits typed semantic actions. The host remains responsible for validating and executing commands, persistence, undo/redo, stage rendering, timeline, VFS, and evaluation.

**Plugin constraint:** the audited AniCraft WASM plugin ABI does not expose arbitrary native egui panel registration. The panel crate is therefore not yet installable as a UI plugin. A host-owned panel extension point or native bridge is required for the plugin stage; the same crate can then be linked into AniCraft for native inclusion. See [HOST_UI_INTEGRATION.md](docs/HOST_UI_INTEGRATION.md).

This implementation status does not imply that the host adapter, character schemas, persistent authoring commands, live stage overlays, or playback integration exist.

## 10. Non-goals and change control

- No separate animation timeline, renderer, undo system, document format, VFS, or general-purpose bone engine.
- No independent Smart Controller implementation.
- No deformation solver duplication.
- No automatic destructive edits to authored views.
- No implicit composition of Smart Controller and Form Deform pin outputs.
- No PR merge or AniCraft host change is authorized by this plan.

This document defines ownership and a proposed roadmap. It does not mean listed features are implemented.


## 10. M1 implementation status

The `anicraft-360-model` workspace crate implements schema version 1 of the persistent Studio-owned character extension. It defines typed identities for Studio entities and distinct wrappers for AniCraft and Forma Deform references.

Implemented:
- Character aggregate and modular part hierarchy.
- View sets, explicit turn direction, zero-angle view and discrete authored views.
- Stable landmark tracks with normalized per-view positions.
- Smart Bone authoring metadata referencing canonical host bone, ControllerId and deformation target identities.
- Switch Groups with explicit defaults, option identity and optional view association.
- Host asset references.
- Validation for duplicate/empty identities, parent references/cycles, view references, angle range/uniqueness, landmark coordinates, Smart Bone ranges, Switch Group defaults and asset metadata.
- Validated JSON serialization/deserialization; unsupported schema versions fail closed.
- Unit tests for round trips and key invalid-data cases.

Not yet implemented:
- AniCraft-owned persistence slot and versioned extension registration.
- Host commands/transactions for create, update and delete.
- Undo/redo and save/reopen integration tests.
- Schema migration framework beyond the current version gate.
- Projection adapter from `CharacterDocument` into `StudioUiModel`.

See [docs/CHARACTER_DATA_MODEL.md](docs/CHARACTER_DATA_MODEL.md). The domain crate intentionally does not resolve external host IDs; the AniCraft adapter must verify symbol, bone, asset, controller and deformation-target references against their authoritative owners.
