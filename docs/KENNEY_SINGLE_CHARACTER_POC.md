# GameShipKit Single-Character PoC

**Status:** source selected and PoC scope updated; an actual character export, artwork inspection, AniCraft import, and host integration are not yet complete.

## Goal

Validate one modular character in AniCraft 360 Studio using a ready-made, consistently generated multi-view character. Keep this deliberately small: one character, four discrete views, one expression Switch Group, and a simple native head/neck setup only if the exported artwork supports it.

## Selected source

- **Tool:** [GameShipKit — Cartoon Character Maker](https://gameshipkit.com/tools/cartoon-character-maker/)
- **Source facts:** builds a character from body, hair, eyes, eyebrows, outfit, face extras, accessories and colour choices; generates matching front, right, left and back views; supplies six matching expressions (neutral, happy, angry, sad, surprised, shy).
- **Export limits:** free tier provides 512 px PNG exports, with a limited number of downloads per day. 1024/2048 px exports, the all-views-and-expressions ZIP, expression sheet, turnaround sheet, and SVG vector export are Pro features.
- **License:** GameShipKit states that generated characters may be used in free and commercial projects without attribution; do not resell the generated characters as an asset pack. Verify the current [license terms](https://gameshipkit.com/license/) when preparing a distributable build.
- **Important limitation:** the back view has no face and is one image for all expressions. These are four discrete rendered views, not a continuous 3D turn or an animation rig.

Use the free 512 px PNG exports for the initial prototype. Do not purchase Pro solely for this PoC. Since free exports are individual images, collect the required images in a consistent naming scheme and verify their alignment before import.

## PoC character deliverable

Create exactly one character in the GameShipKit maker. Keep all character-part and colour selections unchanged while exporting each view/expression combination.

Required view set:

- Front
- Right
- Left
- Back

Required expression set:

- `neutral`
- `happy`
- `angry`
- `sad`
- `surprised`
- `shy`

The back view is a single shared image, not six facial variants. The expected minimum export set is therefore 19 PNGs: six front expressions, six right expressions, six left expressions, and one back view.

The maker's images are generated from the same component selections and are intended to line up. Verify actual pixel dimensions, canvas alignment, and view consistency before importing. Do not imply that the outputs are editable layered SVG; the SVG export is Pro-only. Do not claim that individual PNG exports preserve separate editable body-part layers.

## Studio representation

Use the existing M1 model concepts; do not invent a second character schema:

- `CharacterPart`: character root and imported assembled view/expression artwork as supported by the host import path.
- `ViewSet` / `CharacterView`: four explicit views (front, right, left, back), with deterministic discrete selection.
- `SwitchGroup` / `SwitchOption`: one expression group with six options and an explicit default.
- `AssetReference`: references to imported AniCraft-managed assets, with source/export metadata retained in the PoC manifest.

For the initial PNG-only import, treat each exported view/expression as a flattened image. Do not promise independent head, eye, mouth, or body-part editing. If later work requires cut-out parts, that is a separate artwork-preparation task and must not be represented as provided by this source.

AniCraft remains authoritative for symbol IDs, bones, Library assets, rendering, persistence, and undo/redo. 360 Studio owns character/view/Switch Group semantics. Forma Deform is not required merely to switch discrete view or expression artwork.

## Export naming convention

Use stable names:

- `character_front_neutral.png` through `character_front_shy.png`
- `character_right_neutral.png` through `character_right_shy.png`
- `character_left_neutral.png` through `character_left_shy.png`
- `character_back.png`

Retain the original downloaded filenames in the manifest if they differ.

## Import and provenance rules

- Record the tool URL, creator/provider, license URL, retrieval date, selected character settings/preset if available, export tier, original filenames, and any post-processing.
- Keep original exported PNGs unchanged; put any resized, trimmed, or prepared images in a separate derived-artwork folder.
- Verify that the exported character is consistent across views before importing.
- Do not commit the full generator's source or unrelated assets.
- Do not claim an SVG/layered source unless the actual SVG export is obtained and inspected.

## Acceptance criteria

- [ ] One character is generated with fixed part and colour selections.
- [ ] Four views are available: front, right, left, back.
- [ ] Six expressions are available for front, right and left; one shared back view is used.
- [ ] Required 19 PNGs are collected, named, and verified for dimensions/alignment.
- [ ] Studio view selection changes only the active authored view; no fake interpolation.
- [ ] Expression switching selects the corresponding authored image for the active view.
- [ ] The back view is not incorrectly treated as having facial expressions.
- [ ] No claim of editable layers, independent mouth/eye symbols, SVG, or rigging is made from flattened PNG exports.
- [ ] Save/reopen and undo/redo are verified through AniCraft before claiming persistence.
- [ ] Source and license provenance are retained.
- [ ] The PoC does not require Forma Deform, a separate timeline, or a second renderer.

## Explicit non-goals

- No continuous 360-degree rotation or automatic view synthesis.
- No claim of 3D rotation or intermediate view interpolation.
- No lip-sync, phoneme system, audio, or mouth deformation engine.
- No promise of separate editable body/head/eye/mouth layers from PNG exports.
- No full-body rig or animation demo.
- No purchase of Pro solely for this PoC.
- No general asset-library expansion or second renderer/timeline.
- No merge or host-repository changes implied by this PoC.
