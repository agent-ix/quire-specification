---
id: TC-412
title: "Checked-package abstraction relation node"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-451
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-353
    type: verifies
---

# TC-412: Checked-package abstraction relation node

## Description

Read `correspondence`/`abstraction_relation` nodes against the published
schema and the FR-451 reader order, and confirm that the node carries every
binding, keys a frame binding by its operation key. For the population-target vectors, use the P7/P8 population node
identity vectors from `proposals/checked-package-v2/model-owner-form-vectors.json`
as the relation entries' targets.

## Test Procedure

Produce the checked package of TC-410's AD-01 and read its relation node.
Build each adverse vector by editing that package. Read each package with the strict reader
through the real local Rust boundary and compare its result, or its first
refusal's code, cause and locus, with the vector.

## Vectors

| Vector | Package | Expected result |
| --- | --- | --- |
| AN-01 | AD-01 | One relation node; reading it yields AD-01's three keys and values; the node's `semantic_type` is its own id and its `dependencies` are the three body targets in node-id order |
| AN-02 | AD-01 with no state clause naming `attemptUpdate`, so no `state`/`frame` node exists for it | Admitted; the frame entry is keyed (`ConfigVersion`, `attemptUpdate`) |
| AN-03 | AD-01 with a clause naming `attemptUpdate` | Admitted; the frame entry's pair equals the `operation_anchor` pair of that operation |
| AN-04 | Body without `frames`; a `RustPath` that is `[]`; a frame entry without `receiver`; an occurrence of role `anchor` | Each fails the schema |
| AN-06 | `dependencies` missing the population target | `invalid_semantic_graph` |
| AN-07 | Two `objects` entries out of node-id order | `invalid_semantic_graph` |
| AN-08 | `type` target is the `relation`/`population` node; `population` target is the object type | `invalid_model_binding`/`malformed-declaration` at the target |
| AN-09 | Frame `operation` is one `ConfigVersion` only inherits | `invalid_model_binding`/`malformed-declaration` at the entry |
| AN-10 | `parameters` is `[]` | `invalid_model_binding`/`malformed-declaration` at the entry |
| AN-11 | A second relation node binding the `ConfigVersion` object key | `invalid_model_binding`/`conflicting-binding` at the second node in ascending node-id order, naming both entries and the key |
| AN-12 | AD-01 compiled twice; then AD-01 with `version to "v2"` | The two unchanged compiles give equal node ids; the edited one gives a different relation node |
| AN-13 | The `ConfigVersion` object entry with two `fields` entries named `version`, bound to `"version"` and `"rev"` | `invalid_model_binding`/`conflicting-binding` at the entry, naming both entries and `version` |
| AN-14 | Population entry targeting P7, the `acme/runs/counters` vector | Admitted; `population` is P7's node id; the target has the vector's `relation`/`population` tag/form, `ModelOwner`, empty aggregate body, self `semantic_type` and empty dependencies; the relation depends on P7 |
| AN-15 | Population entry targeting P8, the `acme/runs/archive` vector | Admitted with the corresponding P8 node id; the target has the vector's same node shape, and the relation depends on P8; P7 and P8 remain distinct despite their shared package identity and node shape |
| AN-16 | A population entry targeting a member-type node or scalar text node instead of a `relation`/`population` node | `invalid_model_binding`/`malformed-declaration` at the target |

## Expected Results

Each vector gives exactly its listed result. A frame entry never carries a
frame or anchor node, and its pair resolves as an `operation_anchor` pair
does. Every binding change yields a different relation node. P7 and P8 are
population declaration-node identities; they are not member-type or
collection identities.
