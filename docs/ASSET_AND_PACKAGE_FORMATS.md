# AniCraft 360 Studio — Open Asset and Package Formats

**Status:** Accepted architecture decision  
**Scope:** Artwork interchange, managed project assets, Studio metadata, and portable character-project exchange  
**Decision:** Use established creative file formats for artwork. Use versioned JSON for Studio-owned metadata. Keep `.anicraft360` only as an optional ZIP-based project exchange package.

## 1. Goals

- Keep character artwork editable in established creative applications.
- Avoid inventing a proprietary artwork format or requiring a proprietary asset container for ordinary editing.
- Preserve stable Studio identity and character-specific relationships independently of artwork file internals.
- Support both portable, project-managed assets and explicitly linked external files.
- Make import/export behavior, compatibility limits, and lossy conversions visible to the user.
- Keep AniCraft responsible for its native document, animation, rendering, persistence, and host export formats.

This is an architecture decision, not a claim that every importer/exporter is already implemented.

## 2. Artwork formats

| Format | Role in Studio | Import | Export | Notes |
|---|---|---|---|---|
| SVG (`.svg`) | Primary editable vector artwork | Planned | Planned | Preserve vector paths, groups, transforms, and stable element identifiers where possible. |
| Photoshop document (`.psd`) | Layered raster/paint artwork | Planned, compatibility-dependent | Planned, compatibility-dependent | Preserve supported layers, groups, masks, and metadata. Advanced application-specific features may not round-trip. |
| Blender project (`.blend`) | Optional 3D source/reference project | Planned, optional integration | Planned, optional integration | For 3D modeling, posing, and reference/turnaround generation; not a replacement for 2D artwork or AniCraft rigging. |
| PNG (`.png`) | Raster artwork, previews, textures, and interchange | Planned | Planned | Supports transparency; flattened raster data does not preserve editable source layers. |

Additional standard formats may be added when there is a concrete workflow need and a verified implementation path. A format being listed here does not mean its importer/exporter exists today.

### 2.1 Format selection guidance

- Prefer SVG for scalable vector character components and symbols.
- Prefer PSD when layered raster painting, texture, or compositing data is important.
- Use BLEND for optional 3D source scenes and reference generation workflows.
- Use PNG for flattened raster artwork, previews, textures, and compatibility exchange.
- Do not convert source artwork to a flattened format silently.

### 2.2 Round-trip and compatibility behavior

Importers and exporters must declare the features they preserve. If a file contains unsupported features, Studio must report them before or during import/export and identify any lossy conversion. It must not claim a lossless round-trip unless that has been verified for the relevant feature set.

The original source file should remain recoverable when an operation creates a converted or flattened derivative. Export should never silently overwrite the source file.

## 3. Asset storage modes

Studio supports two explicit storage modes:

### 3.1 Managed project assets — default

On import, Studio copies the selected source asset into the character project's managed asset area. The project then references that managed copy.

Benefits:
- The character project is portable and self-contained.
- Moving the project does not break references to the imported artwork.
- External edits cannot silently change a project asset.
- Studio can maintain version history and variation relationships against a stable project-owned file.

The import flow should preserve the source file and record its original filename, detected format, import time, and available provenance information. Managed copies are not permission to destructively rewrite the original source.

### 3.2 Linked external assets — opt-in

The user may choose to reference an external file without copying it.

Requirements:
- Display that the asset is externally linked.
- Detect missing or moved files and provide relink workflow.
- Do not silently substitute another file with the same name.
- Make portability warnings visible when packaging or sharing a project.
- Record the resolved path/URI and a content fingerprint where supported; do not treat a path alone as stable identity.

Changing a linked source must be an explicit user action with a review of the resulting asset changes.

## 4. Studio metadata

Studio-specific semantics are stored separately from the artwork, using versioned UTF-8 JSON documents. Metadata references artwork through stable Studio asset IDs and project-relative managed paths or explicit external-link records.

The metadata model may be split into logical documents such as:

- `character.json` — character identity, project settings, and top-level structure.
- `assets.json` — asset IDs, types, format, artwork references, and provenance.
- `variations.json` — variation identity, parent/origin relationship, and independent version history.
- `rig.json` — Studio-owned rig organization and Smart Bone authoring references.
- `switches.json` — Switch Groups, stable option IDs, defaults, and artwork/symbol references.
- `turnarounds.json` — authored views, turn-angle ordering, landmarks, and correspondence.

These are logical responsibilities; the implementation may choose a different file partition as long as schemas and ownership remain explicit.

Metadata includes, where relevant:
- Permanent asset and project IDs.
- Parent/origin IDs for derived variations.
- Style and asset classifications.
- Artwork references and content fingerprints.
- Anchors and landmarks.
- Rigging metadata and canonical host references.
- Smart Bone/controller references.
- Switch Group/state references.
- Turnaround views and landmark correspondence.
- Dependency and compatibility information.

AniCraft remains the owner of native document identity, symbol primitives, timeline, rendering, undo/redo, persistence infrastructure, and host export. Studio metadata must not become a parallel AniCraft document model or VFS.

## 5. Asset identity and variations

Keep these concepts distinct:

1. **Library asset:** a reusable source definition.
2. **Variation:** an independently identified derivative of a source asset.
3. **Project instance:** a use of an asset or variation within a character.

A variation receives a new permanent asset ID. The source remains unchanged. The variation retains its parent/origin relationship but has independent artwork and version history. Compatible metadata may be copied and then validated.

Reimporting or updating an existing variation preserves its identity; it must not create duplicate variations automatically. Changes to the original do not propagate to variations. The user may compare a variation with its source and explicitly incorporate selected changes.

When editing an existing variation, Studio must make instance impact explicit: update all linked instances, update selected instances, or create another variation. No silent project-wide modification is allowed.

Smart Bone and Switch Group metadata copied into a variation must be variation-specific references or explicitly shared references; it must not accidentally retain hidden dependencies on the source variation.

## 6. Optional project exchange package

The `.anicraft360` extension is reserved for an optional portable project exchange package. It is not the native artwork format and is not required for ordinary editing.

The package is a ZIP-based container with a versioned manifest, Studio JSON metadata, and the files required by the selected packaging mode. A typical layout is:

```text
Character.anicraft360
├── manifest.json
├── Studio/
│   ├── character.json
│   ├── assets.json
│   ├── variations.json
│   ├── rig.json
│   ├── switches.json
│   └── turnarounds.json
├── Artwork/
│   ├── Head.svg
│   ├── Eyes.psd
│   └── Body.png
└── References/
    └── Character.blend
```

The exact internal directory layout is an implementation detail governed by the manifest schema; the example is illustrative.

### 6.1 Manifest responsibilities

The manifest identifies:
- Package format and schema version.
- Studio project identity and metadata entry points.
- Included assets, stable IDs, media types, and relative paths.
- Variation parent/origin relationships.
- Dependencies and external links not embedded in the package.
- Required capabilities and compatibility constraints.
- Optional content hashes for integrity verification.

### 6.2 Packaging and extraction rules

- Managed assets should be included by default.
- Linked external assets must be listed as external dependencies and clearly reported; they must not be silently omitted or silently embedded.
- The user may choose to include linked files when creating a portable package, subject to permissions and compatibility.
- Import validates the manifest, paths, hashes when present, schema versions, IDs, and dependency references before applying project changes.
- Reject path traversal, duplicate/conflicting package paths, malformed manifests, and unsupported major schema versions.
- Unknown metadata must not be silently reinterpreted.
- Schema migrations must be explicit and testable.
- Import should stage and validate the package before committing changes to the open project.

## 7. Import and export contract

| File/package | Import role | Export role |
|---|---|---|
| `.svg` | Vector artwork or asset | Editable vector artwork |
| `.psd` | Layered raster artwork | Layered raster artwork within supported feature limits |
| `.blend` | Optional 3D source/reference | Optional 3D source/reference when integration exists |
| `.png` | Raster artwork, texture, or preview | Raster artwork, texture, or preview |
| `.json` | Studio metadata and schema documents | Studio metadata and schema documents |
| `.anicraft360` | Portable Studio project exchange package | Portable Studio project exchange package |

The import UI must let the user choose whether artwork is added to an existing character, imported as reusable Library assets, or used to create a new character project. Importing artwork alone must not silently create a complete character project.

Export must explain whether the result is editable source artwork, a flattened/raster derivative, Studio metadata, or a complete project package. AniCraft-native document and animation formats remain governed by AniCraft's own verified import/export capabilities; this specification does not assert support for any unverified host format.

## 8. Security, reliability, and validation

- Treat imported files and package manifests as untrusted input.
- Validate file type from content as well as extension where feasible.
- Apply resource limits to archive extraction and parsing.
- Prevent archive path traversal and unsafe external-reference resolution.
- Do not execute scripts, embedded code, or external automation from imported assets.
- Preserve stable IDs across save/load, package round-trip, rename, and reorder.
- Detect missing artwork, broken metadata references, hash mismatches, and unsupported format features.
- Keep import transactional: a failed validation must not leave a partially modified character project.

## 9. Non-goals

- No proprietary replacement for SVG, PSD, BLEND, or PNG artwork.
- No mandatory proprietary asset container for individual files.
- No second document model, VFS, renderer, timeline, undo system, or native animation format.
- No claim that a listed format is implemented until its importer/exporter and compatibility tests exist.

## 10. Acceptance criteria

The architecture is considered correctly implemented when:
- Individual artwork remains in standard creative formats.
- Studio metadata is versioned JSON with stable IDs and explicit references.
- Managed project copies are the default; linked external files are opt-in and diagnosable.
- Variations have independent permanent identities and histories.
- The optional `.anicraft360` package can carry metadata and selected artwork/dependencies without changing the underlying artwork formats.
- Package import validates before modifying the project and handles unknown schema versions explicitly.
- Import/export surfaces compatibility and lossy-conversion information to the user.
- AniCraft continues to own its native document and host-level workflows.
