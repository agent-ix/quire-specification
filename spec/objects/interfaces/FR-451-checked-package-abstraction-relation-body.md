---
id: FR-451
title: "Checked-package abstraction relation body"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-353
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-450
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-451: Checked-package abstraction relation body

## Description

When a checked package carries an
[FR-450](../../functional/ir-proof/FR-450-spell-abstraction-relation-declaration.md)
abstraction relation declaration, the producer SHALL emit it as one
`quire.checked-semantic-graph/v2` node with `node_tag: "correspondence"` and
`semantic_form: "abstraction_relation"`, whose body carries every binding
with its key and value, and the reader SHALL refuse a node whose joins below
do not hold with the first refusal in the order below. The node is the only
carrier of the relation: a generator reads the relation from it and from
nothing else.

## Properties

- **body** — `{term: "abstraction_relation", objects, populations, frames}`,
  all three members required, each possibly empty:
  - `objects` — one entry per object binding:
    `{type: NodeRef, rust_type: RustPath, fields: [{name: Identifier,
    rust_field: RustField}]}`;
  - `populations` — one entry per population binding:
    `{population: NodeRef, collection: RustPath}`;
  - `frames` — one entry per operation binding:
    `{context: NodeRef, operation: Identifier, function: RustPath,
    receiver: RustReceiver, parameters: [{name: Identifier,
    rust_parameter: RustIdentifier}]}`.
- **RustPath** — an array of one or more `RustIdentifier` strings, the
  path's segments in order. **RustField** is a `RustIdentifier` or a
  tuple-field index string. **RustReceiver** is `"self"` or a
  `RustIdentifier`. Each string has FR-450's "Rust spellings" syntax.
- **Keys.** An object entry's key is its `type` target, a `model`/
  `object_type` node, whose node id is the object type's AD-006
  effective-declaration key carried as its `ModelOwner` (FR-322
  "Model-owned members"). A population entry's `population` value is the
  `node_id` of its target `relation`/`population` node, not the population's
  declaration key directly, its member type, a collection node, or a scalar
  text node. The target is the FR-322 "Population references" node: its
  `owner` is `{kind: "model", identity, node}`, where `identity` is the
  selected domain package identity and `node` is the population declaration's
  IR identity; its `node_tag`/`semantic_form` are `relation`/`population`, its
  body is `{term: "aggregate", members: []}`, its `semantic_type` is its own
  `node_id`, and its `dependencies` are empty. Its `node_id` is the
  structural id of FR-322, `{package, path, ordinal}`, whose `path` is the
  population's declaration path. It therefore identifies the declaration
  through its `ModelOwner`, while the enclosing abstraction relation's
  `dependencies` includes this target node id. The P7/P8 cases in
  [model-owner-form-vectors.json](../../../proposals/checked-package-v2/model-owner-form-vectors.json)
  are two concrete cases: `acme/runs/counters` targets P7 and
  `acme/runs/archive` targets P8. Both use the same node shape and package
  identity, but their declaration paths produce distinct owner nodes and
  target ids. A
  frame entry's key is the
  operation key: its `context` target, a `model`/`object_type` node, and
  its `operation` identifier, the same pair a `state`/`operation_anchor`
  node binds
  ([FR-342](FR-342-checked-package-operation-anchor-body.md)). The FR-013
  frame and FR-012 anchor identities of that operation are the `state`/
  `frame` and `state`/`operation_anchor` nodes FR-342 joins to the same
  pair; the entry carries neither, and binds an operation whether or not
  the package holds those nodes.
- **semantic_type** — the node's own `node_id`.
- **dependencies** — exactly the unique, node-id-ascending set of every
  `type`, `population` and `context` target in the body.
- **occurrences** — every occurrence has role `declaration`, at the
  `abstraction-decl` it was produced from.
- **node_id** — the JCS SHA-256 of `{version:
  "quire.abstraction-relation-node/v1", body}`. Every binding is therefore
  part of the node id, and through the `identity_projection` of the
  package id (FR-322): changing any binding changes the package id, so a
  rebinding is a new relation revision that never reinterprets a contract
  generated from the earlier package (FR-353-AC-5).

## Canonical order

`objects` is sorted ascending by its `type` node id, `populations` by its
`population` node id and `frames` by its `context` node id and then its
`operation` as UTF-8 bytes, each node id compared in ascending node-id
order. `fields`
and `parameters` are sorted ascending by `name` as UTF-8 bytes. A member
array out of this order refuses `invalid_semantic_graph`, as FR-322-AC-10
does for every other noncanonical order.

## Reader order

The abstraction step runs after the state step of
[FR-341](FR-341-checked-package-state-clause-body.md), over every
`correspondence`/`abstraction_relation` node in ascending node-id
order. For one node, the checks run in this order and the first failing
check is its defect:

1. **Identity.** The node's `semantic_type` is its own `node_id` and
   its `dependencies` are exactly the body's targets, else
   `invalid_semantic_graph`.
2. **Order.** The canonical order above holds, else
   `invalid_semantic_graph`.
3. **Targets**, entry by entry in body order (`objects`, `populations`,
   `frames`): a target that is not a declared dependency refuses
   `missing_declaration`/`missing-name`; a `type` or `context` target that
   is not `model`/`object_type`, or a `population` target that is not
   `relation`/`population`, refuses `invalid_model_binding`/
   `malformed-declaration`; a target whose `ModelOwner` names no selected
   declaration of its kind was refused earlier by FR-322's
   owner join (`missing_declaration`/`missing-selection`). The locus is the target.
4. **Members**, entry by entry in the same order: two `fields` entries of
   one object entry with equal `name` refuse `invalid_model_binding`/
   `conflicting-binding` naming both entries and the name; otherwise, a
   `fields` name that is not an effective field member of the object type; a frame `operation`
   that does not resolve as FR-342 "Operation resolution" resolves an
   anchor's operation, with that section's refusals; a `parameters` list
   that is not exactly the operation's declared parameters; two
   `rust_parameter` values that are equal, or one equal to the `receiver`;
   or a Rust string not of its syntax: `invalid_model_binding`/
   `malformed-declaration` at the entry, unless FR-342's resolution gives
   its own code.

Once every node's own checks hold, the reader checks package-wide key
uniqueness over all `correspondence`/`abstraction_relation` nodes, in
ascending node-id order and body order: the second entry to claim an
object, population or operation key refuses `invalid_model_binding`/
`conflicting-binding`, naming both entries and the key (FR-353-AC-4).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-451-AC-1 | A package produced from FR-450-AC-1's declaration holds exactly one `correspondence`/`abstraction_relation` node whose body validates against the published schema and yields the same three keys and values the source declared; a body missing `frames`, a `RustPath` that is an empty array, a frame entry without `receiver`, and an occurrence of role `anchor` each fail the schema. | Test (TC-412) |
| FR-451-AC-2 | A frame entry keyed by (`ConfigVersion`, `attemptUpdate`) is admitted in a package that holds no `state`/`frame` node for that operation; in a package that holds one, the entry's pair equals that frame's `operation_anchor` pair. | Test (TC-412) |
| FR-451-AC-3 | A node whose `dependencies` omit a body target, or whose `objects` are out of node-id order refuses `invalid_semantic_graph`. | Test (TC-412) |
| FR-451-AC-4 | A `type` target naming a `relation`/`population` node and a `population` target naming an object type refuse `invalid_model_binding`/`malformed-declaration` at the target; a target missing from `dependencies` refuses `missing_declaration`/`missing-name`; a frame `operation` the context only inherits refuses `invalid_model_binding`/`malformed-declaration`; a `parameters` list missing a declared parameter refuses `invalid_model_binding`/`malformed-declaration` at the entry. | Test (TC-412) |
| FR-451-AC-5 | Two nodes that bind one object key refuse `invalid_model_binding`/`conflicting-binding` naming both entries and the key, at the second in ascending node-id order. | Test (TC-412) |
| FR-451-AC-6 | Recompiling an unchanged unit gives the same node id; changing one field's `rust_field` changes the compiled package. | Test (TC-412) |
| FR-451-AC-7 | An object entry holding two `fields` entries named `version`, bound to different Rust fields, refuses `invalid_model_binding`/`conflicting-binding` naming both entries and `version`. | Test (TC-412) |
| FR-451-AC-8 | The `population` value of an abstraction entry is the `node_id` of the FR-322 population declaration node: the P7 `counters` case and P8 `archive` case each admit the exact `relation`/`population` node, owner, empty aggregate body, self `semantic_type` and empty dependencies in `model-owner-form-vectors.json`; the enclosing relation depends on that node id. The two targets remain distinct although they share package identity and node shape. A member-type or text-literal node used in place of either target refuses `invalid_model_binding`/`malformed-declaration` at the target. | Test (TC-412) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the graph, node ids,
  model-owned member resolution, the `identity_projection` and the reader's
  refusal order.
- [FR-353](../../functional/ir-proof/FR-353-declare-abstraction-relation.md)
  owns the relation's meaning and refusals.
- [FR-450](../../functional/ir-proof/FR-450-spell-abstraction-relation-declaration.md)
  owns the source form and its static checks.
- [FR-342](FR-342-checked-package-operation-anchor-body.md) owns the
  operation resolution the frame entries share.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires each refusal to carry a stable catalogued cause and an exact
  locus.

## References

- quire-spec-language ADR-017 §3 AR-2 and AR-4, §6 Q-2 and Q-7.
- Specification tickets STD-116, STD-121.

## Status

Proposed complete-V1 boundary contract for the abstraction relation node.
