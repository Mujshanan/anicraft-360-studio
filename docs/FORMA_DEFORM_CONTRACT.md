# AniCraft 360 Studio ↔ Forma Deform Integration Contract

**Status:** Proposed cross-plugin contract, aligned to current repository capabilities  
**Contract version:** 0.1.0  
**Owners:** AniCraft 360 Studio (character semantics) and Forma Deform (controller/deformation semantics)  
**Host:** AniCraft

This document defines the boundary between the two sibling plugins. It is a schema and interaction proposal, not evidence that a runtime bridge or host persistence API already exists.

## 1. Ownership

| Concern | Owner | Contract behavior |
|---|---|---|
| Character, part, symbol and instance organization | 360 Studio | Publishes stable character/part references |
| View sets, turn angles, landmarks, correspondence | 360 Studio | Owns schemas and validation |
| Smart Bone authoring/action semantics | 360 Studio | References host bone IDs and Forma Controller IDs |
| Controller definitions, input normalization, mapping, easing | Forma Deform | Owns canonical controller model and evaluation |
| Deformation bindings, target mapping, corrective deformation | Forma Deform | Owns target bindings and deformation semantics |
| Bone identity, transforms, symbols, document, timeline | AniCraft | Authoritative host source |
| Persistence, undo/redo, command execution, rendering, evaluation lifecycle | AniCraft | Plugins use supported host APIs only |

Neither plugin may create a competing host document model, timeline, renderer, VFS, undo stack, bone engine, or general-purpose deformation runtime.

## 2. Identity rules

All cross-plugin references use stable IDs. Display names, array positions, hierarchy paths, and current selection are not identity.

### ID namespaces

| ID | Issuer | Example shape | Meaning |
|---|---|---|---|
| `character_id` | 360 Studio | `char_<opaque>` | Character record |
| `part_id` | 360 Studio | `part_<opaque>` | Character part/group record |
| `view_set_id` | 360 Studio | `vset_<opaque>` | Multi-view set |
| `view_id` | 360 Studio | `view_<opaque>` | Authored view |
| `landmark_id` | 360 Studio | `lm_<opaque>` | Semantic landmark |
| `smart_bone_id` | 360 Studio | `sb_<opaque>` | Smart Bone authoring record |
| `switch_group_id` / `option_id` | 360 Studio | opaque strings | Switch Group and option |
| `bone_id` | AniCraft | decimal numeric ID serialized as a string | Canonical host bone |
| `element_id` | AniCraft | decimal numeric ID serialized as a string | Host element/symbol instance |
| `controller_id` | Forma Deform | opaque stable string | Canonical controller definition |
| `binding_id` | Forma Deform | opaque stable string | Controller-to-target binding |
| `target_id` | Host adapter / Forma target contract | typed stable reference | Deformation target, e.g. a specific warp pin |

ID strings are opaque to consumers. Host numeric IDs must be parsed and validated by the adapter before invoking host commands. A host ID is never an array index.

### Typed host references

A target reference must include its target kind and enough host identity to resolve it unambiguously. For the currently audited warp-pin path:

```json
{
  "kind": "anicraft.warp_pin",
  "element_id": "1234",
  "pin_id": "7"
}
```

The pin ID is the specific pin returned by `warp.info`; the containing element alone is not a target identity. Coordinate values for this target are stage-space unless an operation explicitly says otherwise.

## 3. Smart Bone reference schema

360 Studio owns the authoring record. Forma Deform owns the referenced controller definition.

Illustrative schema (not a currently persisted host document shape):

```json
{
  "schema": "anicraft.360.smart_bone",
  "schema_version": 1,
  "smart_bone_id": "sb_example",
  "name": "Head turn",
  "source": {
    "kind": "anicraft.bone",
    "bone_id": "42",
    "angle_channel": "constrained_keyed_angle_degrees"
  },
  "action": {
    "kind": "character_action",
    "view_set_id": "vset_main",
    "affected_part_ids": ["part_head"]
  },
  "controller_ref": {
    "provider": "forma-deform",
    "controller_id": "ctrl_example"
  },
  "status": "draft"
}
```

Rules:

- `bone_id` is the canonical AniCraft bone ID.
- The angle channel is explicit. The audited `bone.info` result separates constrained keyed angle from spring swing. If a future controller uses the sprung angle, that must be a separately named channel and tested adapter behavior.
- `controller_id` references a Forma Deform controller; Studio does not embed a copy of its ranges, easing, mapping, graph, or outputs.
- A Smart Bone action is not itself a deformation binding.
- The host document extension, not either plugin's private filesystem, must own durable storage.

## 4. Forma Deform controller reference

360 Studio stores only the minimum reference needed to identify a controller and present its status:

```json
{
  "provider": "forma-deform",
  "contract_version": "0.1.0",
  "controller_id": "ctrl_example",
  "display_name_hint": "Head rotation",
  "required_capabilities": ["controller.inspect", "controller.preview"]
}
```

`display_name_hint` is presentation only; the controller ID is authoritative. The referenced controller may be missing, incompatible, or unsupported. Studio must surface that state rather than silently recreating it.

## 5. Handoff contract

The desired user flow is:

1. Studio selects a Smart Bone or character action.
2. Studio resolves its host bone and controller references.
3. Studio requests Forma Deform to inspect/edit the referenced controller.
4. Forma Deform owns controller configuration and deformation target bindings.
5. Forma Deform returns a result/status summary; Studio refreshes its reference status.

The handoff context should be versioned and contain references, not copied mutable models:

```json
{
  "protocol": "anicraft.plugin-handoff",
  "protocol_version": 1,
  "source_plugin": "anicraft-360-studio",
  "target_plugin": "forma-deform",
  "context": {
    "character_id": "char_example",
    "smart_bone_id": "sb_example",
    "bone_id": "42",
    "controller_id": "ctrl_example"
  },
  "intent": "inspect_or_edit_controller"
}
```

This is a proposed payload. The repositories do not yet establish a cross-plugin deep-link API. Until a host-supported handoff exists, the UI may offer a documented manual open/focus action or show the integration as unavailable. Do not invent a URL scheme or claim automatic navigation.

## 6. Preview request and response

The first integration target is deterministic, on-demand preview. Preview must be separate from apply/bake and must not mutate authored document data.

Illustrative request:

```json
{
  "protocol": "forma-deform.preview",
  "protocol_version": 1,
  "request_id": "req_example",
  "controller_id": "ctrl_example",
  "input": {
    "kind": "bone_angle_degrees",
    "bone_id": "42",
    "frame": 12,
    "angle_channel": "constrained_keyed_angle_degrees",
    "value": 18.5
  },
  "targets": [
    {
      "kind": "anicraft.warp_pin",
      "element_id": "1234",
      "pin_id": "7"
    }
  ],
  "mode": "read_only"
}
```

Illustrative response:

```json
{
  "protocol": "forma-deform.preview-result",
  "protocol_version": 1,
  "request_id": "req_example",
  "status": "previewed",
  "input": {
    "value": 18.5,
    "unit": "degrees",
    "source": "anicraft.bone"
  },
  "outputs": [
    {
      "target": {
        "kind": "anicraft.warp_pin",
        "element_id": "1234",
        "pin_id": "7"
      },
      "value": { "x": 108.0, "y": 46.0 },
      "unit": "stage_space_position"
    }
  ],
  "diagnostics": []
}
```

The output values above are illustrative only. The exact request/response transport and final output schema require agreement with Forma Deform's adapter. No plugin-to-plugin RPC currently exists in the inspected contracts.

Preview requirements:

- Inputs are an explicit snapshot, including frame and angle channel.
- Same model and same explicit inputs produce the same result.
- The response is read-only and does not write keyframes, pin positions, or document metadata.
- Missing/stale source or target identities produce structured diagnostics.
- Preview results are invalidated or marked stale when source, target, controller, mapping, or frame changes.
- The UI distinguishes “previewed” from “applied” and “persisted”.

## 7. Apply and mutation boundary

An apply/bake operation is not a preview. Any operation that changes AniCraft artwork must be issued through a host command/transaction.

For the currently audited pin command:

- `warp.info` is read-only and returns pin IDs, stage-space current positions, and artwork-space rest positions.
- `warp.pin_move` is mutating and keys the pin at the requested frame.
- Absolute `x/y` are stage-space coordinates; `dx/dy` are stage-space deltas.
- The command participates in the host document undo transaction.
- A target must resolve to the specific numeric element and pin IDs.
- A command-dialog preview may use host rollback-on-cancel semantics only after adapter tests verify the exact path.

Do not use a mutating pin command as if it were a pure preview API. Do not combine Smart Controller outputs and Form Deform pin outputs implicitly. Any future composition must define ordering, coordinate space, additive/override semantics, and undo behavior explicitly.

## 8. Current host capability baseline

Based on the current Forma Deform SDK audit:

| Capability | Current status | Integration consequence |
|---|---|---|
| Query bone angle with `bone.info` | Available on demand | Preview can request an explicit frame snapshot |
| Distinguish keyed angle and spring swing | Available in query result | Channel must be explicit |
| Inspect warp pins with `warp.info` | Available on demand | Stable pin target can be resolved |
| Move a warp pin with `warp.pin_move` | Mutating command | Apply/preview transaction semantics must be tested |
| Whole-document plugin element inventory | Not established; element query is active-layer scoped | Do not promise arbitrary document-wide discovery |
| Symbol instance displayed-frame setter | Not established | Do not promise visual symbol-frame application |
| Arbitrary plugin-owned document metadata | Not available in inspected plugin contract | Durable Smart Bone/ViewSet/Binding persistence needs a host extension |
| Continuous per-frame controller evaluator | Not available in inspected plugin contract | No live playback claim |
| Cross-plugin deep link/RPC | Not established | Handoff is a proposal until host transport exists |

This baseline is intentionally conservative. Re-audit the AniCraft host and Forma Deform adapter before changing a capability to supported.

## 9. Persistence contract

The long-term design requires an AniCraft-owned, versioned document extension. The exact host implementation is outside both plugin repositories.

Candidate document extension envelope:

```json
{
  "extension": "anicraft.360-studio",
  "schema_version": 1,
  "characters": [],
  "view_sets": [],
  "smart_bones": [],
  "switch_groups": []
}
```

Forma Deform's controller and binding records remain Forma-owned under a separate extension namespace. Cross-plugin references use stable IDs only.

Requirements:

- Host-controlled save/load and document lifecycle.
- Schema version and migration path.
- Validation of references on load.
- Stable identity across rename, reorder, duplicate, and save/load.
- One coherent undo operation for each user-visible document mutation where supported.
- Unknown fields/namespaces preserved by the host migration path where possible.
- No plugin writes directly into undocumented host internals.

Until this extension exists, the UI must treat authored records as drafts/session-only and clearly disclose that they are not persisted.

## 10. Versioning and compatibility

- Contract versions use `MAJOR.MINOR.PATCH`.
- Additive optional fields increment MINOR.
- Bug fixes and clarifications increment PATCH.
- Breaking field meaning, ID semantics, units, coordinate spaces, or required behavior increment MAJOR.
- Every request/response carries a protocol name and integer protocol version.
- Unknown optional fields are ignored; unknown required capabilities cause an explicit unsupported result.
- Units and coordinate spaces are mandatory wherever values are numeric.
- Never infer units from field names or UI context.

## 11. Diagnostics

Use stable machine-readable codes with human-readable messages. Initial proposed codes:

| Code | Meaning |
|---|---|
| `STUDIO_REF_MISSING_CONTROLLER` | Referenced Forma controller does not exist |
| `STUDIO_REF_MISSING_BONE` | Host bone ID cannot be resolved |
| `STUDIO_REF_MISSING_TARGET` | Deformation target cannot be resolved |
| `STUDIO_UNSUPPORTED_HANDOFF` | Host has no supported cross-plugin handoff |
| `STUDIO_UNSUPPORTED_PERSISTENCE` | Host document extension is unavailable |
| `STUDIO_UNSUPPORTED_LIVE_EVALUATION` | Continuous evaluator is unavailable |
| `FORMA_INPUT_CHANNEL_UNSUPPORTED` | Requested bone input channel is not exposed |
| `FORMA_TARGET_KIND_UNSUPPORTED` | Target type is not supported by the adapter |
| `FORMA_PREVIEW_INVALID_INPUT` | Preview input is invalid or non-finite |
| `FORMA_PREVIEW_STALE_REFERENCE` | A referenced source or target is stale |

Codes are proposals and must be reconciled with Forma Deform's diagnostic registry before implementation.

## 12. Conformance tests

### Identity and schema
- IDs remain stable across rename and reorder.
- Host IDs are never treated as array indices.
- Missing and stale controller/bone/target references fail with structured diagnostics.
- Unknown optional fields are tolerated.
- Unsupported required capabilities fail explicitly.

### Smart Bone semantics
- Studio stores a controller reference, not a duplicate mapping.
- Keyed angle and spring swing are never silently conflated.
- Smart Bone action metadata remains distinct from Forma deformation bindings.

### Preview and mutation
- Preview is deterministic for identical explicit inputs.
- Preview does not mutate document state or create keyframes.
- Target resolution identifies a specific pin.
- Apply uses host transaction/undo semantics.
- Preview and apply have distinct UI states and commands.

### Persistence and lifecycle
- Draft state is never presented as persisted.
- Host-backed persistence tests cover save/load and migration before enabling “Saved”.
- Live playback remains disabled until a host evaluator integration test proves deterministic evaluation and cache invalidation.

## 13. Implementation sequence

1. **Contract alignment:** agree on ID formats, angle channel, target identity, diagnostics, and capability vocabulary.
2. **Studio UX shell:** hierarchy, stage context, view rail, turn ruler, and contextual Properties.
3. **Studio domain schemas:** ViewSet, View, Landmark, SmartBoneDefinition, SwitchGroup, and validation.
4. **Forma handoff:** implement only through a verified host mechanism; until then expose an honest unavailable/manual state.
5. **Preview adapter:** agree and test the request/response contract against Forma's existing deterministic core and AniCraft query commands.
6. **Persistence:** implement a separately reviewed AniCraft document extension before enabling durable authored records.
7. **Live evaluation:** separate host project and milestone; no implied support from the v1 command plugin.

