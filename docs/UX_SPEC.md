# AniCraft 360 Studio — UX Specification

**Status:** Proposed implementation baseline  
**Target:** Native AniCraft workspace/plugin; usable first as a plugin and later as an integrated workspace  
**Related contract:** [Forma Deform Integration Contract](./FORMA_DEFORM_CONTRACT.md)

## 1. Product experience

AniCraft 360 Studio is the character-authoring workspace. It answers: **how is this character assembled, controlled, and authored across its turn views?**

The workspace must feel like a focused AniCraft editing mode, not a second animation application. It reuses AniCraft's stage, selection, timeline, document, rendering, command, asset, persistence, and undo systems.

### UX principles

1. **Stage first:** character artwork remains the primary focus.
2. **Character context, not a second document model:** hierarchy is an authoring view over host symbols and instances.
3. **Progressive disclosure:** show controls for the current selection and active authoring task.
4. **Stable identity:** renaming or reordering a part/view must not break references.
5. **Explicit operations:** create, capture, bind, validate, preview, and apply are distinct actions.
6. **Honest capability states:** distinguish local draft, previewed, host-persisted, and runtime-evaluated data.
7. **Shared host behavior:** selection, shortcuts, timeline, undo/redo, and asset browsing follow AniCraft conventions.
8. **Forma Deform is a peer tool:** launch or focus its editor for deformation work; do not recreate its controller inspector or deformation tools.

## 2. Workspace information architecture

The workspace consists of six coordinated regions:

- **Character hierarchy:** character parts, host symbols/instances, rig groups, Smart Bones, and Switch Groups.
- **Stage:** active artwork/view, host selection, rig context, landmarks, silhouette guides, and optional correspondence overlays.
- **View rail:** ordered authored views with turn angles and validation states.
- **Turn ruler:** numeric angle and direction; selects an authored view or requests a supported preview state.
- **Properties:** contextual inspectors for the selected character concept.
- **Asset Library:** reusable character parts, symbols, turn templates, and references through AniCraft's asset services.

AniCraft's timeline remains the only timeline. The host document/layer model remains authoritative.

## 3. Desktop layout

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ AniCraft chrome · document · workspace tools · undo / redo                  │
├──────────────────┬──────────────────────────────────────┬───────────────────┤
│ CHARACTER        │ STAGE                                │ PROPERTIES        │
│ HIERARCHY        │                                      │                   │
│                  │ Active character view                 │ Selection context │
│ Character        │                                      │ Smart Bone        │
│  Parts           │ Artwork / rig / landmarks             │ Switch Group      │
│  Rig groups      │ Correspondence / silhouette guides   │ View registration │
│  Smart Bones     │                                      │ Validation        │
│  Switch Groups   │                                      │                   │
├──────────────────┴──────────────────────────────────────┴───────────────────┤
│ VIEW RAIL · FRONT · 3/4 · SIDE · 3/4 · BACK · + VIEW                         │
│ TURN RULER · direction · angle · active view                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ Existing AniCraft timeline / layers / playhead                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Responsive/docking behavior

- Hierarchy and Properties are dockable using host conventions.
- The Stage receives the largest share of the workspace.
- The View Rail can collapse to thumbnails or a compact strip.
- On narrow layouts, Properties may collapse into a host panel/tab.
- Routine authoring must not require a blocking modal.
- Do not introduce a replacement timeline, scene tree, or asset browser backend.

## 4. Navigation and selection model

- Selecting a hierarchy row selects or focuses its corresponding AniCraft symbol/instance through the host selection API.
- Selecting artwork on the Stage updates the hierarchy and contextual Properties.
- Selecting a view changes the active character-view context; it must not silently change the host playhead.
- Selecting a Smart Bone opens the 360 Studio authoring inspector. Its controller reference may open Forma Deform.
- Selecting a Switch Group exposes its options and current/default state.
- Selection is transient UI state. Authored character data is persistent document data only when a supported host extension exists.

## 5. Core workflows

### 5.1 Character setup and modular parts

1. Create or select a character root using host-supported symbol/document primitives.
2. Organize existing symbols and instances into character-specific groups.
3. Assign character roles and stable 360 Studio IDs to the authoring records.
4. Inspect host identity separately from Studio identity.
5. Reorder or rename groups without changing their IDs.
6. Use AniCraft commands for host document changes so undo/redo remains coherent.

The hierarchy is an organizational lens, not a duplicate document tree. A host object must never be fabricated when the host API cannot create or identify it.

### 5.2 Smart Bone authoring

1. Select a canonical AniCraft bone.
2. Create a Smart Bone definition and assign a stable Studio ID.
3. Define its character action and affected view/part relationships.
4. Choose or create a Forma Deform Controller definition.
5. Store a reference to the canonical Forma Deform Controller ID; do not copy its mapping, range, easing, or evaluation model.
6. Validate source-bone and controller references.
7. Open Forma Deform to edit controller mapping and deformation bindings.
8. Preview only through a supported on-demand capability.
9. Persist the Smart Bone authoring record only through the approved host extension boundary.

A Smart Bone is a character-authoring concept. It is not a second Smart Controller implementation.

### 5.3 Switch Groups

- A group contains mutually exclusive artwork/symbol alternatives.
- Each group and option has a stable ID.
- The group declares a default option and, where supported, a current option.
- Option order and display name are mutable; IDs are not.
- Host symbol visibility/selection changes use AniCraft commands and undo.
- View-specific option associations are Studio metadata, not a new rendering system.

### 5.4 Multi-view and turn authoring

1. Create a View Set and choose turn direction/convention.
2. Register authored views with stable IDs and explicit angles.
3. Associate each view with host artwork/symbol references.
4. Mark landmarks with stable IDs and positions in the documented coordinate space.
5. Link corresponding landmarks across views.
6. Run validation for missing views, duplicate angles, missing/duplicate landmarks, and inconsistent correspondence.
7. Add view-specific corrective intent and reference Forma Deform targets when deformation is required.
8. Generate turn sheets through AniCraft rendering/export facilities.

The turn ruler selects authored view context. It must not imply interpolated artwork or live deformation if no evaluator is available.

## 6. Contextual Properties

### Character / Part
- Display name and Studio ID
- Host symbol/instance reference
- Parent/group membership
- View participation
- Validation and missing-reference state

### Smart Bone
- Name and Studio ID
- Canonical host bone ID and display name
- Action definition and trigger/view range
- Forma Deform Controller ID and link status
- Affected parts/targets
- Validation diagnostics
- Open in Forma Deform action

### Switch Group
- Group ID and name
- Option list with stable IDs
- Default/current option
- Host symbol references
- View associations
- Missing/duplicate option diagnostics

### View
- View ID and label
- Turn angle and direction convention
- Host artwork reference
- Landmark list and correspondence status
- Validation state

### Landmark
- Landmark ID and semantic label
- Position and coordinate-space label
- Correspondence links
- Source view
- Validation state

## 7. Forma Deform handoff

The Studio UI should present a clear boundary:

- **Configure character action** in 360 Studio.
- **Configure controller mapping and deformation targets** in Forma Deform.
- **Return to 360 Studio** to review character/view context and validation.

The handoff carries stable IDs and a selection/context hint only. It must not pass mutable array indices or depend on current selection as identity.

If a deep link or host action is unavailable, show a disabled/unavailable state with a clear explanation rather than a fake button. The two tools may initially be separate plugin commands; the UX must still preserve the same ownership model.

Forma Deform owns its own target inspector, controller editor, mapping, deformation diagnostics, and preview values. 360 Studio may summarize integration status but must not duplicate those controls.

## 8. State, persistence, and undo

| State/data | Owner | Lifetime |
|---|---|---|
| Hover, active tool, panel layout, selection | Studio UI / host | Transient |
| View sets, turn angles, landmarks, correspondence | 360 Studio schema | Persistent only via host extension |
| Smart Bone/action authoring | 360 Studio schema | Persistent only via host extension |
| Switch Group policy/options | 360 Studio schema + host symbol refs | Persistent only via host extension |
| Controller definitions/mapping | Forma Deform | Forma-owned |
| Deformation target bindings | Forma Deform | Forma-owned |
| Bones, symbols, document, timeline, rendering | AniCraft | Host-owned |
| Undo/redo and command execution | AniCraft | Host-owned |

Do not introduce a standalone project file, plugin-owned filesystem store, or private undo stack. Until the host persistence extension exists, authored records must be labeled as drafts/session-only and must not be described as saved.

A user-visible operation that mutates the AniCraft document must use a host command/transaction. Multi-step edits should be one coherent undo operation where the host supports it.

## 9. Capability and status language

Use explicit labels:

- **Draft** — local authoring state; not persisted.
- **Needs attention** — invalid or unresolved references.
- **Ready to preview** — inputs and target references are valid for an available preview path.
- **Previewed** — an explicit deterministic preview completed; no authored data was changed.
- **Persisted** — only after verified host-backed document storage succeeds.
- **Live** — only after a verified host evaluation hook exists.
- **Unsupported** — the current host/plugin contract cannot perform this action.

Never use “Saved”, “Live”, “Auto-updating”, or “Playback enabled” for unverified capabilities.

## 10. Accessibility and visual language

- Every icon-only tool has a tooltip and accessible name.
- Status is conveyed with text/icon as well as color.
- Selected, hovered, disabled, invalid, and focused states are visually distinct.
- Keyboard navigation covers hierarchy, view rail, and inspector controls.
- Angle values include units and turn direction.
- Coordinate values identify their coordinate space.
- Landmark overlays remain legible against light and dark artwork.
- Provide overlay toggles for landmarks, correspondence, silhouette, rig, and corrective references.

## 11. Acceptance criteria

- The workspace reuses AniCraft stage, selection, timeline, document, asset, and undo infrastructure.
- No second timeline, renderer, scene tree, VFS, document model, or deformation engine is introduced.
- Character parts, views, landmarks, Smart Bones, and Switch Groups use stable IDs.
- Reordering/renaming does not invalidate references.
- Selecting a Smart Bone exposes a Forma Deform Controller reference and an honest handoff state.
- Controller mapping and deformation settings are not duplicated in 360 Studio.
- Missing/unsupported host capabilities are visible and actionable.
- No persistence/live-evaluation claim is made before the corresponding host contract is implemented and tested.
- View ordering, correspondence, Switch Group exclusivity, and Smart Bone references have validation coverage.
