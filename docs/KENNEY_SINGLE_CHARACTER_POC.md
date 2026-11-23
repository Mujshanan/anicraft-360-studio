# Kenney Single-Character PoC

**Status:** scoped PoC; asset download, source inspection, artwork preparation, and host integration are not yet complete.

## Goal

Validate one real, modular character in AniCraft 360 Studio instead of using rectangle placeholders. Keep this deliberately small: one character, one head-turn set, one mouth Switch Group, and one simple head/neck rig.

## Selected source

- **Pack:** [Kenney — Shape Characters](https://kenney-assets.itch.io/shape-characters)
- **Source facts:** 100+ sprites; separate facial features and hands; spritesheets and separate sprites.
- **License:** CC0 1.0 Universal.
- **Selection reason:** separate facial features make it the best fit for testing interchangeable facial artwork.
- **Not included by the source:** a guaranteed 360-degree head turnaround, a rig, or a ready-made mouth-animation controller. Do not claim otherwise.

The pack is free/name-your-price. Keep the source pack outside the repository until the exact chosen character and required source files have been inspected. Do not commit the entire pack.

## PoC character deliverable

Prepare exactly one assembled character from the Kenney source:

1. A replaceable head symbol.
2. Separate eye artwork.
3. Separate mouth artwork.
4. A simple neck/head pivot setup using AniCraft's native rig primitives.
5. Four authored head-view artworks at cardinal angles:
   - Front: 0°
   - Right profile: 90°
   - Back: 180°
   - Left profile: 270°

These are discrete authored views. The 360 Studio turn ruler must select the nearest registered view; it must not imply continuous 3D rotation or generated intermediate artwork. Do not add three-quarter or intermediate views to this PoC.

## Mouth Switch Group

Create one Studio-owned Switch Group named **Mouth** with four mutually exclusive options:

| Option ID | Label | Intended artwork |
|---|---|---|
| `mouth-neutral` | Neutral / closed | Resting mouth |
| `mouth-open` | Open | Open mouth |
| `mouth-smile` | Smile | Smiling mouth |
| `mouth-surprised` | Surprised | Round / surprised mouth |

Use separate Kenney facial-feature artwork when a matching source sprite exists. If the pack does not contain all four suitable shapes, derive only the missing mouth shapes from the selected Kenney vector artwork; document each derivative and retain the untouched source. Do not introduce a second asset provider or generate replacement character artwork.

The Mouth Switch Group is discrete artwork selection, not a deformation or phoneme evaluator. Keep its options independent of the head-turn view set. If mouth visibility differs by view, use view-specific option artwork/references rather than silently applying a front-facing mouth to the back of the head.

## Data and ownership mapping

Use the existing M1 model concepts; do not invent a second character schema:

- `CharacterPart`: character root, head, neck, eyes, mouth, and other necessary assembled parts.
- `ViewSet` / `CharacterView`: the four explicit cardinal head views and their angles.
- `LandmarkTrack`: only a small set of useful correspondence landmarks (head center, eye line, nose/mouth center, chin, neck pivot) where they can be identified reliably across the applicable views.
- `SwitchGroup` / `SwitchOption`: the four mouth states, with one explicit default.
- `AssetReference`: references to AniCraft-managed assets, with source/derived role metadata kept in the PoC manifest.

AniCraft remains authoritative for symbol IDs, bones, Library assets, rendering, persistence, and undo/redo. 360 Studio owns the character/view/Switch Group semantics. Forma Deform is not required for this PoC and must not be introduced merely to switch mouth artwork or select a discrete view.

## Import and provenance rules

- Preserve the downloaded Kenney archive and original vector/raster files outside generated outputs.
- Import only the selected character's required source elements into the AniCraft Library.
- Keep source and prepared/derived artwork distinguishable.
- Record the source pack URL, author (Kenney), CC0 1.0 license, retrieval date, original filenames, and any derived-file relationship.
- Do not add the whole source pack to the application repository.
- Do not commit artwork until the selected source files and their layer/part suitability have been inspected.

## Acceptance criteria

- [ ] One Kenney character is assembled from separately editable parts.
- [ ] Head, neck, eyes, and mouth are independently selectable/editable.
- [ ] Four authored cardinal head views are registered at 0°, 90°, 180°, and 270°.
- [ ] Turn-ruler selection changes the active authored view only; no fake interpolation.
- [ ] Mouth Switch Group has four options and a deterministic default.
- [ ] Switching mouth options changes only the mouth artwork and preserves head/eye identity.
- [ ] Head/neck pivot uses AniCraft-native rig primitives; no second bone engine.
- [ ] Save/reopen and undo/redo are verified through AniCraft before claiming persistence.
- [ ] Asset provenance and CC0 license metadata are retained.
- [ ] The PoC does not require Forma Deform, new asset providers, a separate timeline, or a second renderer.

## Explicit non-goals

- No full 360-degree continuous rotation or automatic view synthesis.
- No lip-sync, phoneme system, audio, or mouth deformation engine.
- No full-body rig or animation demo.
- No general asset-library expansion or asset repository.
- No merge or host-repository changes implied by this PoC.
