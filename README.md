# AniCraft 360 Studio

**AniCraft 360 Studio** is a proposed native AniCraft workspace/plugin for modular character construction, symbol rigging, Smart Bones, Switch Groups, reusable character assets, and multi-view/360° character workflows.

It organizes how a character is assembled, controlled, and represented across views. It is not a second animation application, renderer, timeline, or deformation engine.

## Product responsibilities

- Modular symbol and character-part organization.
- Character rig setup and Smart Bone authoring workflows.
- Switch Groups for discrete artwork/state alternatives.
- Character asset library and reusable symbol/rig templates.
- Multi-view character sets: front, side, three-quarter, back, and intermediate turn views.
- Turn-angle and view/keyform organization.
- View correspondence, landmarks, silhouette/proportion checks, and turn-sheet workflows.
- View-specific corrective setup, using Forma Deform for deformation behavior.

## Relationship to Forma Deform

The two plugins have distinct responsibilities:

- **360 Studio owns character semantics and authoring:** symbols, Smart Bones/actions, Switch Groups, asset organization, views, turn angles, and view-specific authoring.
- **Forma Deform owns deformation behavior:** Smart Controller definitions and mapping, target bindings, deformation algorithms, corrective deformation evaluation, and deformation preview.
- **AniCraft owns shared host infrastructure:** bones, document identity, timeline, rendering, persistence, undo/redo, command execution, evaluation scheduling, and export.

Smart Bones in 360 Studio should reference canonical controller IDs. They do not create a second Smart Controller engine. Smart Controller binding evaluation and Form Deform pin evaluation remain independent; no implicit output composition is introduced.

## UI proposal

360 Studio should be a native AniCraft workspace built from existing dockable panels and the existing stage/timeline.

1. **Character workspace / stage:** the current selected view, with overlays for landmarks, silhouette, rig controls, symbol bounds, and view-specific guides.
2. **View rail:** thumbnails for front, front three-quarter, side, back three-quarter, and back views, plus authored intermediate views. A turn-angle ruler selects the active view/transition position.
3. **Character hierarchy panel:** symbols, body parts, nested symbol instances, rig groups, and Switch Groups. This is a character-specific organization view, not a replacement for AniCraft's layer/document model.
4. **Properties panel:** selected symbol, Smart Bone/action mapping, Switch Group states, view angle, correspondence landmarks, and validation.
5. **Asset Library panel:** reusable symbols, character parts, turn templates, rig presets, and preview thumbnails. It should integrate with AniCraft's host asset/library services rather than implement a second VFS.
6. **Timeline:** AniCraft's existing timeline remains authoritative. 360 Studio adds context-specific authoring affordances but does not create a separate animation timeline.
7. **Forma Deform integration:** a selected corrective target can open or invoke Forma Deform's binding/deformation editor. 360 Studio stores the view-specific intent and target references; Forma Deform evaluates deformation.

### Conceptual layout

```text
┌────────────────────────────────────────────────────────────────────────────┐
│ AniCraft menus / Character workspace / tool context                         │
├──────────────┬───────────────────────────────────────────┬─────────────────┤
│ Character    │ Stage                                     │ Properties      │
│ hierarchy    │                                           │                 │
│              │       active character view                │ Symbol          │
│ Symbols      │       rig / landmarks / silhouette        │ Smart Bone      │
│ Body parts   │       correspondence guides                │ Switch Group    │
│ Rig groups   │                                           │ View angle      │
│ Switches     │                                           │ Validation      │
├──────────────┼───────────────────────────────────────────┼─────────────────┤
│ View rail    │ Turn-angle ruler / view correspondence     │ Asset Library   │
│ Front        │ ●────●────●────●────●                       │ Symbols         │
│ 3/4          │ F    3/4  Side  3/4  Back                   │ Parts/templates │
│ Side         │                                           │ Rig presets     │
├──────────────┴───────────────────────────────────────────┴─────────────────┤
│ Existing AniCraft timeline / layers / playhead                              │
└────────────────────────────────────────────────────────────────────────────┘
```

The layout is a proposal, not an implementation or a request to add new host-level docking infrastructure.

## Current status

This repository contains the architecture proposal and the initial `anicraft-360-ui` host-embedded egui panel crate. The hierarchy, view rail, properties/validation, and asset-library panels are implemented as UI components; live AniCraft host integration and persistent character features remain planned.

See [DEVELOPMENT.md](DEVELOPMENT.md) for the feature matrix, code-reuse audit, ownership boundaries, UI behavior, and staged roadmap.
