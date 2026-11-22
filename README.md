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

## UX and integration specifications

- [UX Specification](docs/UX_SPEC.md) — workspace layout, navigation, character workflows, contextual inspectors, status language, accessibility, and acceptance criteria.
- [Forma Deform Integration Contract](docs/FORMA_DEFORM_CONTRACT.md) — stable IDs, Smart Bone/controller references, handoff and preview payload proposals, mutation boundaries, capability baseline, versioning, diagnostics, and conformance tests.
- [DEVELOPMENT.md](DEVELOPMENT.md) — ownership boundaries, feature allocation, and staged roadmap.

These specifications distinguish proposed cross-plugin behavior from currently verified host capabilities. In particular, durable plugin-owned document persistence, cross-plugin handoff, and continuous playback evaluation must not be presented as available until implemented and verified in AniCraft.

## Current status

This repository is an architecture/workspace proposal. Features listed here are planned responsibilities, not claims that the plugin or its UI is already implemented. The UX and integration documents establish the implementation baseline; they do not imply that the host bridge or persistence extension already exists.
