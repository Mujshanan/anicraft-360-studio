# AniCraft 360 Studio — UI Specification

**Document status:** Proposed specification  
**Version:** 1.0  
**Scope:** User interface and interaction behavior for the AniCraft 360 Studio workspace  
**Parent architecture:** [DEVELOPMENT.md](../DEVELOPMENT.md)  
**Implementation status:** Not implemented; this document defines requirements, not delivered functionality.

---

## 1. Purpose and authority

This document specifies the user-facing workspace for AniCraft 360 Studio: its application shell, panels, tools, navigation, interaction rules, UI states, accessibility expectations, and acceptance criteria.

It is subordinate to `DEVELOPMENT.md`:

- `DEVELOPMENT.md` is authoritative for product scope, system ownership, integration boundaries, and milestones.
- This document is authoritative for UI behavior and presentation.
- If a UI requirement conflicts with an accepted architecture boundary, the architecture boundary wins and this document must be revised.

AniCraft 360 Studio is a specialized character-authoring workspace hosted by AniCraft. It is not a separate animation application and must not recreate host infrastructure.

### 1.1 Product questions the UI must answer

The workspace should make it clear to the artist:

1. Which character and character document are being edited?
2. Which character part, symbol, bone, or authoring object is selected?
3. Which authored view and turn angle are active?
4. Which views are missing, incomplete, or invalid?
5. Which Smart Bone definitions and Switch Groups affect the character?
6. Which data is authored, preview-only, generated, or supplied by another system?
7. Which optional integrations are available, unavailable, or reporting errors?

### 1.2 Design principles

- **Host-native:** use AniCraft's established window, panel, command, theme, selection, and timeline conventions wherever available.
- **Character-first:** organize tools around character structure and authored views.
- **Contextual:** show properties and actions relevant to the current selection and mode.
- **Non-destructive:** preserve original artwork and distinguish previews from authored data.
- **Explicit:** never imply interpolation, playback, deformation, or integration capabilities that are not implemented and verified.
- **Traceable:** identify the owner and status of externally supplied data.
- **Progressive disclosure:** keep common authoring actions visible and advanced settings contextual.
- **Consistent:** use the host's visual language rather than introducing an unrelated design system.

---

## 2. Scope and exclusions

### 2.1 In scope

- AniCraft 360 Studio workspace shell and panel arrangement.
- Character hierarchy and selection.
- Stage/viewport context and authoring overlays.
- Multi-view rail and view registration.
- Turn ruler and angle navigation.
- Contextual Properties Inspector.
- Character Asset Library integration.
- Smart Bone authoring UI and controller references.
- Switch Group authoring UI.
- View correspondence and landmark authoring.
- Validation and diagnostics presentation.
- UI state, document state, empty states, loading states, and error states.
- Keyboard, pointer, and accessibility behavior.

### 2.2 Out of scope

The UI must not implement or duplicate:

- AniCraft document storage, VFS, rendering, timeline, undo/redo, command execution, or general bone infrastructure.
- Forma Deform controller evaluation or deformation algorithms.
- A second animation timeline or playback engine.
- A second project/document format.
- Automatic destructive editing of artwork.
- An unverified continuous 360-degree interpolation experience.

---

## 3. Workspace information architecture

### 3.1 Workspace entry

AniCraft 360 Studio is opened as a native AniCraft workspace or plugin window. It must use the host's document context and active document.

The workspace header should identify:

- Workspace name: **AniCraft 360 Studio**.
- Active character/document name, when available.
- Active view name and angle, when available.
- Integration status when a required capability is unavailable.

Do not create a separate project-opening flow if AniCraft already owns document opening and switching.

### 3.2 Primary regions

The default desktop arrangement is:

| Region | Default placement | Responsibility |
|---|---|---|
| Workspace header | Top | Workspace identity and contextual status |
| Character Hierarchy | Left | Character parts, symbols, bones, Smart Bones, Switch Groups |
| Stage | Center | Active artwork, selection, guides, and authoring overlays |
| View Rail | Along stage edge or below stage | Authored views and their status |
| Turn Ruler | Near View Rail | Numeric angle navigation and discrete view selection |
| Properties Inspector | Right | Contextual editing and object details |
| Asset Library | Dockable panel or tab | Character assets and reusable templates |
| Host Timeline | Host-owned location | Existing AniCraft timeline only |
| Diagnostics | Dockable panel or tab | Validation, integration, and authoring issues |

The host may constrain exact docking positions. The UI must adapt to supported AniCraft panel APIs rather than creating a competing docking framework.

### 3.3 Panel behavior

Every panel must define:

- A stable panel identity.
- A visible title and accessible name.
- A clear empty state.
- A loading state when data is asynchronous.
- An error state with a recoverable action where possible.
- Selection/context behavior.
- Whether its state is transient UI state or persistent workspace preference.

Panel visibility, width, docking position, and active tab are workspace preferences if the host supports saving them. They are not character document data.

### 3.4 Minimum viable workspace

The first usable workspace should include:

1. Character Hierarchy.
2. Stage with selection context.
3. View Rail.
4. Properties Inspector.
5. Diagnostics access.
6. Host timeline integration without duplicating the timeline.

The Asset Library, Smart Bone Editor, and Switch Group Editor may initially appear as contextual tabs or dockable panels. Do not block the shell on advanced engine integrations.

---

## 4. Character Hierarchy

### 4.1 Purpose

The hierarchy presents the character's authored organization using AniCraft's existing symbols, instances, bones, and other host primitives, together with 360 Studio's character-specific groupings.

### 4.2 Node categories

The UI may display these node categories when the underlying host or extension data provides them:

- Character root.
- Character part/group.
- Symbol or symbol instance.
- Bone.
- Smart Bone definition.
- Switch Group.
- Switch Group option.
- View set and authored view.
- Landmark set or correspondence group, where useful.

Each node must have a stable underlying identity. Display names are labels, not identifiers.

### 4.3 Required interactions

- Single-click selects a node and updates contextual properties.
- Double-click performs the node's primary edit/open action, if one is defined.
- Expand/collapse changes only hierarchy UI state.
- Rename uses the host command/document APIs appropriate to the object owner.
- Drag-and-drop reordering is allowed only where the data model defines a valid ordering operation.
- Dragging a host symbol or bone into a 360 Studio grouping must create a reference or host-supported relationship, not duplicate the underlying object.
- Context menus expose only actions valid for the selected node and current capability state.
- Multi-selection must be visibly indicated and must not silently change the active view.

### 4.4 Selection synchronization

- Selecting a hierarchy node selects or focuses its corresponding host object when supported.
- Selecting an object on the Stage highlights its hierarchy node when a mapping exists.
- If the selected object is outside the current character context, the UI must not fabricate a hierarchy mapping.
- Selection synchronization must not create recursive selection loops.
- The active view and selected object are separate state dimensions.

### 4.5 Node status

Use text and iconography in addition to color to communicate states such as:

- Valid.
- Missing reference.
- Unavailable integration.
- Warning.
- Error.
- Preview-only.
- Generated/uncommitted.

Do not rely on color alone.

---

## 5. Stage / Viewport

### 5.1 Purpose

The Stage is the primary visual authoring area. It displays the active authored view using AniCraft's renderer and selection infrastructure.

### 5.2 Responsibilities

360 Studio may add character-specific overlays, including:

- Active view label and turn angle.
- View registration guides.
- Landmark markers and correspondence guides.
- Silhouette/reference overlays.
- View-specific correction references.
- Selection emphasis for relevant character objects.
- Validation annotations.
- Preview-state indicators.

The Stage must not implement a separate renderer, transform system, or deformation engine.

### 5.3 Navigation

Use AniCraft's existing viewport navigation conventions wherever possible:

- Pan.
- Zoom.
- Fit artwork/content.
- Reset view.
- Select.
- Contextual tool activation.

Do not introduce conflicting shortcuts for host-owned viewport operations.

### 5.4 Selection and overlays

- Overlay visibility is transient UI state unless the host has an established persistent tool-preference mechanism.
- Selecting a landmark highlights the same landmark in the active view and the Properties Inspector.
- Selecting a correspondence item highlights all available corresponding landmarks without changing their authored positions.
- Missing correspondence must be indicated explicitly.
- Preview overlays must be visually distinguishable from authored artwork.
- Hiding an overlay must never delete its underlying data.

### 5.5 View status banner

When useful, show a compact status banner containing the active view name, angle, and status. Avoid covering important artwork. The banner must distinguish an authored view from a generated preview.

### 5.6 Unsupported operations

If the host cannot provide a required operation, disable the corresponding control and explain why. Never simulate a successful edit with a visual-only state change.

---

## 6. Multi-view Rail

### 6.1 Purpose

The View Rail presents the registered views in deterministic turn-angle order and provides direct navigation between them.

### 6.2 View item content

Each view item should display:

- Thumbnail when available.
- View name.
- Authored turn angle.
- Status indicator.
- Optional short warning count.

The item must distinguish:

- Authored artwork.
- Generated or assisted artwork.
- Missing/unregistered view.
- Invalid or incomplete view.

### 6.3 Required interactions

- Click a view item to make it the active view.
- Keyboard navigation moves between items in the displayed order.
- A dedicated command creates/registers a view when supported by the model.
- Rename and remove actions must use owner-approved document commands.
- Reordering is controlled by the angle model, not arbitrary visual drag order, unless a separate explicit custom-order property is defined.
- A missing view placeholder may be shown only when the ViewSet model explicitly represents an expected but absent view.

### 6.4 Ordering

The rail must use the canonical turn-angle ordering defined by the data model. It must not independently sort angles using display strings or locale-dependent text.

The UI must respect the project's declared turn direction, zero-angle convention, and wraparound rules.

### 6.5 Empty state

When no views are registered, show a concise explanation and a primary action to create the first view set or register the first view, depending on the available model.

---

## 7. Turn Ruler

### 7.1 Purpose

The Turn Ruler provides precise angle inspection and navigation across registered views.

### 7.2 Required controls

- Numeric angle display.
- Numeric angle input when editing/navigation is supported.
- Discrete view markers for registered views.
- Current active-view indicator.
- Turn direction and angle convention indicator, where needed.
- Previous/next registered-view actions.

### 7.3 Interaction semantics

The Turn Ruler must support two explicitly distinct operations:

1. **Select registered view:** choose an authored view at its registered angle.
2. **Inspect/request angle:** enter or scrub an angle for inspection only when the host and relevant integration provide a verified capability.

If continuous interpolation is unavailable, scrubbing must not visually imply that the artwork is continuously interpolating. The UI should snap to registered views or show an explicit “no evaluated view at this angle” state.

### 7.4 Numeric input

- Validate values against the canonical angle convention.
- Show normalized values consistently.
- Reject invalid or non-finite values.
- Do not silently reinterpret a value when turn direction changes.
- Preserve the distinction between changing the active inspection angle and modifying a registered view's stored angle.

### 7.5 Accessibility

All view markers must be keyboard reachable and have accessible labels containing the view name and angle.

---

## 8. Properties Inspector

### 8.1 Purpose

The Inspector exposes properties and commands for the current selection. It must not become a generic second document editor.

### 8.2 Contexts

The Inspector should provide contextual sections for:

- Character/character root.
- Character part/group.
- Symbol or symbol instance.
- Bone reference.
- Smart Bone definition.
- Switch Group.
- Switch Group option.
- ViewSet.
- Authored view.
- Landmark.
- Correspondence group.
- Validation issue.

### 8.3 Common behavior

- Show the selected object's name and type.
- Group related properties into labeled sections.
- Distinguish editable fields from read-only host properties.
- Validate edits before committing them.
- Use host commands and document transactions for persistent edits.
- Provide inline validation messages near invalid fields.
- Do not silently discard invalid or unsaved edits.
- Avoid displaying internal IDs as primary user-facing labels; expose them in advanced/debug details when useful.

### 8.4 No selection

When nothing is selected, show workspace-level properties and concise guidance. Do not show stale properties from the previous selection.

### 8.5 Multiple selection

Show only common editable properties or clearly labeled batch operations. Do not present a single object's values as if they applied to all selected objects.

---

## 9. Character Asset Library

### 9.1 Ownership

AniCraft owns storage, VFS, file access, indexing, persistence, and general asset infrastructure. 360 Studio owns character-specific organization, metadata, templates, and authoring workflows.

### 9.2 Library categories

The UI may organize supported assets into categories such as:

- Character parts.
- Symbols.
- Character templates.
- Turn/view templates.
- Smart Bone authoring presets.
- Switch Group presets.
- Character configurations.

A category must not imply that 360 Studio owns a separate asset file format.

### 9.3 Required interactions

- Browse and search.
- Filter by supported metadata.
- Preview an asset where the host supports preview.
- Import or insert using AniCraft asset commands.
- Reuse/link an asset where supported.
- Inspect source and compatibility information.
- Show unavailable or incompatible assets without silently omitting them.

### 9.4 Import safety

- Confirm destructive replacement or overwrite operations through host-standard confirmation.
- Preserve source assets.
- Show import errors and actionable details.
- Do not claim an asset has been imported until the host confirms completion.

---

## 10. Smart Bone Editor

### 10.1 Ownership model

The UI must present three distinct concepts:

| Concept | Owner | UI responsibility |
|---|---|---|
| Host bone and its identity/transforms | AniCraft | Select and reference the canonical bone |
| Smart Bone authoring definition | 360 Studio | Create and edit character-specific authoring data |
| Smart Controller and evaluation | Forma Deform | Discover capability, select/reference controller, request supported preview |

A Smart Bone definition is not a second Smart Controller implementation.

### 10.2 Editor sections

Where supported, the editor should expose:

- Display name and stable definition identity (advanced details).
- Canonical AniCraft bone reference.
- Optional Forma Deform ControllerId reference.
- Character-specific action or view relationship.
- Trigger range and associated view information, as defined by the model.
- Affected symbols/targets as references.
- Validation and integration status.
- Preview controls only when a verified preview capability exists.

### 10.3 Controller reference states

Show explicit states:

- Not linked.
- Linked and valid.
- Linked but unresolved.
- Controller capability unavailable.
- Incompatible controller/API version.
- Preview unavailable.
- Preview request failed.

The UI must not invent a controller or silently substitute another controller when a reference is invalid.

### 10.4 Preview

- Preview is an explicit action.
- Display the source and status of preview results.
- Keep preview output separate from authored document data.
- Do not claim live playback unless the host provides a verified evaluation hook.
- Do not combine Smart Controller outputs with pin-deformation outputs implicitly.

---

## 11. Switch Group Editor

### 11.1 Purpose

The editor manages named groups of mutually exclusive artwork or symbol alternatives using host-owned symbols and persistence.

### 11.2 Required fields

- Group name.
- Stable group identity.
- Options with stable option identities.
- Display names.
- Default option.
- Current selected option.
- Optional view or character-state associations where the data model supports them.

### 11.3 Required interactions

- Create and rename a group.
- Add an existing symbol/artwork reference as an option.
- Remove an option without deleting its source asset.
- Choose a default option.
- Choose the current option.
- Reorder options if order is a supported presentation property.
- Validate that the default and current options refer to existing members.

### 11.4 Exclusivity

The UI must make mutual exclusivity clear. It must not allow multiple active options in a group unless the underlying model explicitly changes the group semantics.

### 11.5 Empty and invalid states

- Empty group: explain that options must be added.
- Missing option reference: show an unresolved item and preserve its identity for repair.
- Missing default: show a validation issue and require an explicit resolution where a default is mandatory.

---

## 12. Multi-view Registration and Correspondence

### 12.1 View registration

The registration UI should allow the user to:

- Create or select a ViewSet.
- Define or inspect turn direction and angle convention.
- Register an existing artwork/symbol as a view.
- Assign the canonical angle.
- Name the view.
- Inspect duplicate or conflicting angles.
- Remove a registration without deleting the underlying artwork.

The exact rules for duplicate angles, multiple artwork alternatives, and angle normalization must be defined by the character data model, not by UI-only logic.

### 12.2 Landmark authoring

A landmark editor should support:

- Creating a landmark with a stable identity.
- Naming or categorizing it.
- Placing or editing its position in the active view using host-supported coordinate transforms.
- Selecting an existing landmark.
- Showing its correspondence in other registered views.
- Marking optional or unavailable correspondence where the model supports it.

The UI must display the coordinate space and units where ambiguity is possible. It must not calculate or persist coordinates using guessed viewport transforms.

### 12.3 Correspondence view

The UI should provide a way to inspect a landmark across registered views. At minimum:

- Show the active landmark identity.
- List views where the landmark is present.
- Identify missing or duplicate references.
- Allow navigation to a corresponding view.
- Avoid moving or editing other views as a side effect of navigation.

### 12.4 Validation presentation

Validation messages should identify:

- Affected ViewSet/view/landmark.
- Problem category.
- Severity.
- Short explanation.
- Available repair action, if one exists.

A validation warning must not automatically modify the document.

---

## 13. Diagnostics and status communication

### 13.1 Diagnostic categories

The Diagnostics panel should group issues by:

- Character data/model validation.
- Missing host references.
- Missing or invalid Forma Deform references.
- View registration and angle conflicts.
- Landmark correspondence.
- Switch Group consistency.
- Unsupported capabilities.
- Preview/evaluation failures.

### 13.2 Severity

Use a consistent severity model:

- **Info:** useful state or non-blocking note.
- **Warning:** potentially incomplete or inconsistent authoring.
- **Error:** invalid data or failed operation.
- **Unavailable:** required optional capability is not present.

Every severity must include a text label or accessible equivalent; color alone is insufficient.

### 13.3 Navigation

Selecting a diagnostic should focus the relevant panel, object, or view where possible. Navigation must not alter authored data.

### 13.4 Empty state

When no issues exist, show a positive, non-intrusive “No issues detected” state. Do not imply that unimplemented validators have run.

---

## 14. UI state and document state

### 14.1 Transient UI state

Examples:

- Current selection.
- Hover.
- Active tool.
- Open context menu.
- Active panel/tab.
- Expanded hierarchy nodes.
- Overlay visibility.
- Current zoom/pan.
- Current inspection angle when not committed to the model.
- Preview display state.

These values must not be written into character document data unless explicitly promoted to a documented persistent preference or authored property.

### 14.2 Persistent authored data

Examples:

- ViewSets and registered views.
- Turn-angle definitions.
- Landmark identities and positions.
- Landmark correspondence.
- Smart Bone authoring definitions.
- Switch Group definitions and options.
- Character template references and character-specific metadata.

Persistent data must use the versioned AniCraft-owned document extension boundary defined by the architecture. 360 Studio must not create a separate project file or persistence system.

### 14.3 Workspace preferences

Panel arrangement, panel visibility, and other workspace preferences may be persisted through host-supported workspace settings. They must remain separate from character-authored data.

### 14.4 Undo and redo

Every persistent authoring operation must use AniCraft's command/undo system. UI-only changes must not create document undo entries unless host conventions explicitly require it.

---

## 15. Interaction conventions

### 15.1 Pointer

- Single-click selects or activates.
- Double-click invokes the defined primary edit/open action.
- Right-click opens a context menu appropriate to the target.
- Drag handles must be visually identifiable.
- Drag-and-drop must show a valid/invalid drop indication.
- Destructive actions require host-standard confirmation.

### 15.2 Keyboard

Use AniCraft's existing shortcut conventions first. 360 Studio-specific shortcuts must:

- Avoid collisions with host commands.
- Be discoverable in menus/tooltips.
- Work when focus is in the appropriate panel.
- Respect text-entry controls.
- Be documented in the application shortcut reference.

The exact shortcut assignments are implementation decisions and must be checked against the host shortcut registry before adoption.

### 15.3 Focus

- Keyboard focus must be visible.
- Opening a panel should establish predictable focus.
- Closing a modal or popover should return focus to its invoking control when possible.
- Panel navigation must not steal focus from active text entry.

### 15.4 Confirmation and recovery

Use confirmation for destructive or difficult-to-reverse operations. Prefer reversible host commands and undo over unnecessary confirmation dialogs.

---

## 16. Visual design system

### 16.1 Host-first styling

Use AniCraft theme tokens and component primitives if available. Do not introduce an independent color palette, typography system, or control library without a demonstrated host gap.

### 16.2 Visual hierarchy

The interface should prioritize:

1. Active artwork and selection.
2. Active view and turn angle.
3. Character structure.
4. Contextual properties.
5. Validation and integration status.

### 16.3 Color and status

- Use host semantic colors for success, warning, error, and disabled states.
- Do not communicate status through color alone.
- Keep overlays visually distinct from original artwork.
- Ensure selection and landmark markers remain distinguishable against light and dark artwork.

### 16.4 Typography and spacing

Use host typography, spacing, border, elevation, and corner-radius tokens. Avoid hard-coded visual values where host design tokens exist.

### 16.5 Icons

Use the host icon system. Icons must have accessible names or accompanying labels when their meaning is not universally clear.

### 16.6 Density

Support the information density expected of a desktop creative application. Avoid oversized controls that unnecessarily reduce Stage space. Frequently used controls should remain discoverable.

---

## 17. Responsive and window behavior

AniCraft 360 Studio is primarily a desktop authoring workspace.

The layout must adapt to constrained window sizes by:

- Allowing supported panels to collapse or become tabs.
- Preserving access to the Stage and active-view navigation.
- Moving secondary controls into overflow menus when needed.
- Avoiding overlapping panels and inaccessible controls.
- Preserving selection and authored data when panels are resized or hidden.

This requirement does not imply a separate mobile UI. Mobile/tablet support requires a separate product decision and host capability audit.

---

## 18. Accessibility

The UI should:

- Provide keyboard access to all primary workflows.
- Show visible keyboard focus.
- Use accessible names for icon-only controls.
- Expose status and validation text to assistive technologies.
- Avoid color-only communication.
- Maintain readable contrast under host light and dark themes.
- Support text scaling where the host supports it.
- Provide non-drag alternatives for important drag-and-drop operations.
- Ensure tooltips do not contain the only explanation of a critical control.

---

## 19. Loading, empty, error, and unavailable states

Every data-driven panel must define these states:

| State | Expected behavior |
|---|---|
| Loading | Show progress without presenting stale data as current |
| Empty | Explain what is absent and provide a valid next action |
| Error | Explain what failed and offer retry/recovery where possible |
| Unavailable | Identify the missing host/plugin capability |
| Invalid reference | Preserve the reference and expose a repair path |
| Read-only | Explain why editing is unavailable |
| Preview-only | Clearly label non-authored output |
| Unsaved/pending | Reflect host transaction or operation status accurately |

Do not show a success state until the host or integration confirms the operation.

---

## 20. UI architecture and implementation constraints

### 20.1 Host integration

Before implementation, inspect and document the actual AniCraft APIs for:

- Panel registration and docking.
- Theme and design tokens.
- Selection and active document context.
- Symbol and bone references.
- Viewport overlays and tools.
- Commands and undo/redo.
- Asset browsing/import.
- Document extension storage.
- Diagnostics and notifications.
- Timeline access.

Use existing host APIs where available. Record missing capabilities rather than creating shadow implementations.

### 20.2 Component boundaries

UI components should be separated by responsibility:

- Workspace shell and panel composition.
- Character hierarchy presentation.
- Stage overlays and tools.
- View Rail and Turn Ruler.
- Properties Inspector.
- Asset Library presentation.
- Smart Bone authoring forms.
- Switch Group editor.
- Landmark/correspondence tools.
- Diagnostics presentation.

These are logical UI boundaries, not a mandate to create one package or module per component.

### 20.3 State ownership

- Host document state remains host-owned.
- 360 Studio authored extension state is accessed through its versioned document boundary.
- Forma Deform state is not copied into a parallel 360 Studio store.
- UI state is managed by the workspace and must not mutate document state accidentally.
- Preview results are transient unless explicitly committed through a documented authoring command.

### 20.4 Capability-driven UI

The UI must query supported host/plugin capabilities through documented interfaces. It must not infer support from plugin presence or version strings alone.

Controls requiring unavailable capabilities must be disabled or hidden according to host conventions, with a clear explanation when the missing capability affects the current workflow.

---

## 21. Testing and acceptance criteria

### 21.1 Workspace shell

- Workspace opens in an AniCraft document context.
- No separate document or timeline is created.
- Panels can be shown/hidden and resized using host-supported behavior.
- Empty, loading, and error states are present.
- Closing/reopening the workspace does not corrupt document data.

### 21.2 Hierarchy and selection

- Host objects are referenced rather than duplicated.
- Hierarchy selection and Stage selection synchronize where mappings exist.
- Invalid references are visible and recoverable.
- Reordering is allowed only for supported relationships.
- Multi-selection does not misrepresent object properties.

### 21.3 View Rail and Turn Ruler

- Views display in canonical angle order.
- Selecting a view activates the correct authored view.
- Angle labels follow the model's canonical convention.
- Invalid, missing, and generated views are distinguishable.
- Scrubbing does not imply interpolation when none exists.
- Keyboard navigation works.

### 21.4 Properties Inspector

- Inspector context follows selection.
- No-selection state does not show stale properties.
- Invalid edits are reported.
- Persistent changes use host commands and undo/redo.
- Multi-selection displays only valid common properties.

### 21.5 Smart Bone Editor

- Smart Bone authoring data references canonical host bone identities.
- Controller references use canonical Forma Deform ControllerIds.
- Missing/unavailable controller capability is represented accurately.
- No duplicate controller evaluator is introduced.
- Preview is clearly separated from authored data.

### 21.6 Switch Group Editor

- Option identities remain stable across rename and reorder.
- The default/current option must reference a group member.
- Mutual exclusivity is enforced according to the model.
- Removing an option does not delete its source asset.
- Undo/redo restores the prior group state.

### 21.7 Multi-view and landmarks

- Landmark identities remain stable across views.
- Missing and duplicate correspondence is reported.
- Navigation between corresponding landmarks does not modify positions.
- Coordinate conversion uses verified host transforms.
- Validation never performs automatic destructive repair.

### 21.8 Integration and lifecycle

- Workspace remains usable when optional Forma Deform capabilities are unavailable.
- Invalid plugin references remain visible.
- Preview failures are reported without corrupting authored data.
- Controller and deformation outputs remain independent.
- No live-playback claim is made without a verified host evaluator.

### 21.9 Accessibility and visual regression

- Primary workflows are keyboard accessible.
- Focus indicators are visible.
- Status is not color-only.
- Light and dark host themes are supported where provided.
- Panel resizing and constrained layouts do not hide essential controls.

---

## 22. Implementation phases

The following UI work maps to the architecture milestones in `DEVELOPMENT.md`. It does not replace or reorder those milestones.

| UI phase | Related milestone | Deliverable | Exit condition |
|---|---|---|---|
| UI-0 — Host UI audit | M0 | Host UI capability map | Existing APIs and gaps documented |
| UI-1 — Interaction and data contract | M1 | UI state map, object contexts, wireframes, error states | UI behavior maps to versioned data contracts |
| UI-2 — Workspace shell | M2 | Native panels and workspace composition | Shell works with test data and host context |
| UI-3 — Character organization | M3 | Hierarchy and Switch Group UI | Host references, persistence, and undo verified |
| UI-4 — Smart Bone authoring | M4 | Smart Bone editor | Canonical references and unavailable states verified |
| UI-5 — Multi-view authoring | M5 | View Rail, Turn Ruler, registration and landmarks | Deterministic navigation and validation verified |
| UI-6 — Corrective integration | M6 | Corrective-reference UI and diagnostics | Independent evaluator contracts preserved |
| UI-7 — Assisted workflows | M7 | Preview and analysis presentation | Non-destructive and opt-in behavior verified |
| UI-8 — Playback presentation | M8 | Verified playback/scrub controls | Host evaluation lifecycle and invalidation verified |

### 22.1 UI-0 gate

Do not implement the workspace against assumed host APIs. The audit must include source references, supported signatures, limitations, and test evidence.

### 22.2 UI-1 gate

Do not finalize detailed component behavior before the character data model and integration contracts are sufficiently specified.

### 22.3 UI-2 gate

The first workspace shell may use mock/test data, but mock data must be clearly separated from production document state.

---

## 23. Open decisions

The following decisions require host/API inspection or explicit product approval. They must not be silently assumed during implementation.

| ID | Decision | Required evidence/owner |
|---|---|---|
| UI-OD-01 | Exact panel docking and persistence APIs | AniCraft host audit |
| UI-OD-02 | Stage overlay/tool extension APIs | AniCraft host audit |
| UI-OD-03 | Canonical turn-angle convention and wraparound | 360 Studio data-model specification |
| UI-OD-04 | Duplicate-angle and multiple-artwork-per-view policy | 360 Studio data-model specification |
| UI-OD-05 | Landmark coordinate space and conversion API | AniCraft host audit + data model |
| UI-OD-06 | Exact Smart Bone action/trigger fields | 360 Studio + Forma Deform contract |
| UI-OD-07 | Forma Deform capability discovery and preview API | Forma Deform public contract |
| UI-OD-08 | Character Asset Library host API and metadata support | AniCraft host audit |
| UI-OD-09 | Keyboard shortcut assignments | AniCraft shortcut registry |
| UI-OD-10 | Mobile/tablet workspace support | Separate product decision |

Until resolved, implementation must use only behavior supported by verified contracts and clearly mark unavailable functionality.

---

## 24. Change control

- This document is a UI specification, not evidence that any UI feature exists.
- Any change that affects ownership, persistence, evaluation semantics, or system boundaries must be reconciled with `DEVELOPMENT.md`.
- Host API assumptions must be backed by source references or integration tests.
- New UI behavior must include relevant state, error, accessibility, and acceptance requirements.
- Do not merge implementation work or change the AniCraft host architecture solely because a UI control has been proposed.

**End of specification.**
