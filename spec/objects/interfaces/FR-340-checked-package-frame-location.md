---
id: FR-340
title: "Checked-package frame body"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/FR-013
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-208
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-340: Checked-package frame body

## Description

When a `quire.checked-semantic-graph/v2` node has `node_tag: "state"` and
`semantic_form: "frame"`, the producer SHALL encode its `body` as exactly
the business-model frame `model-complete.md`'s Frames rule and
[FR-013](../../functional/foundation/FR-013-validate-operation-frames.md)
already state: three named members, `modifies`, `creates` and `deletes`.
A `modifies` entry names a relationship by its node key, or a field by the
node key of the declaration that declares it plus the field's name; a
`creates` or `deletes` entry names an object type or process by its node
key. This wire encoding is the V2 carrier of that existing model.

## Properties

- **semantic_type** (`NodeRef`) — the `model`/`object_type` node declaring
  the operation whose frame this is: the `context` of the
  `state`/`operation_anchor` node that binds the frame
  ([FR-342](FR-342-checked-package-operation-anchor-body.md)).
- **occurrences** — every occurrence has role `generated`: a frame is
  derived from its operation's authored `Modifies:`, `Creates:` and
  `Deletes:` lines, never declared on its own.
- **modifies** (array of `FrameModifiesEntry`) — every field or relationship
  the operation may write. An entry is one of:
  - `{kind: "relationship", declaration}`: `declaration` is the node key of
    a declared `relation`/`relationship` node;
  - `{kind: "field", declaration, name, scope}`: `declaration` is the node
    key of the `model`/`object_type` or `model`/`record_value_type` node that
    declares, or inherits, the field, and `name` is the field's name. This
    is the same pair a `field` operation member carries
    ([FR-322](FR-322-checked-package-artifact.md) "Model-owned members").
    `scope` is exactly `receiver` or `every-object`, the entry's
    [FR-013](../../functional/foundation/FR-013-validate-operation-frames.md)
    scope: `receiver` grants the write on the operation's receiver only,
    `every-object` on every object whose effective type has the field.
- **creates** (array of `NodeRef`) — every object type or process the
  operation may create an instance of. Each entry is the node key of a
  declared `model`/`object_type` or `model`/`process` node.
- **deletes** (array of `NodeRef`) — every object type or process the
  operation may delete an instance of. Each entry is the node key of a
  declared `model`/`object_type` or `model`/`process` node, using the same
  closed meaning as `creates`. Such an entry is type-wide.
- **deletes_scoped** (array of `FrameDeletesScopedEntry`) — every scoped
  [FR-013](../../functional/foundation/FR-013-validate-operation-frames.md)
  `deletes` entry: `{kind: "receiver"}` for `deletes self`, or
  `{kind: "parameter", name}` for `deletes p`, `name` being the reference
  parameter's name. The member is required and may be empty; a `parameter`
  entry whose name is not a reference parameter of the operation refuses
  `missing_declaration`/`missing-name`. Entries are sorted with `receiver`
  first, then `parameter` entries by name as UTF-8 bytes, with no duplicate;
  a misordered array refuses `invalid_semantic_graph`.
- Every entry's node key (a `modifies` entry's `declaration`, a `creates`
  or `deletes` entry itself) equals one of the frame node's own
  `dependencies` ([FR-322](FR-322-checked-package-artifact.md): "References
  name a compatible declared target by node key"); an entry naming no
  dependency of the frame node, or no node of the package at all, refuses
  `missing_declaration`/`missing-name`. An entry naming a declared
  dependency of any other meaning refuses `invalid_model_binding`/
  `malformed-declaration`: a relationship entry naming a node that is not
  `relation`/`relationship`; a field entry whose `declaration` is not
  `model`/`object_type` or `model`/`record_value_type`; a `creates` or
  `deletes` entry naming a node that is not `model`/`object_type` or
  `model`/`process`. A `value` literal, an `expression`, and every other
  `model` form (`value_type`, `variant_type`, `event_type`,
  `state_machine`, `persistence_interface`, `namespace`, `model_import` and
  the five systems forms) has no frame meaning at all.

## Field entries

A field entry's `name` resolves among the fields of its declaring node, by
the member resolution
[FR-322](FR-322-checked-package-artifact.md)'s "Model-owned members" gives a
`field` member: for a `model`/`object_type` node, step 2 looks up the
declaration the node's `ModelOwner` names and step 3 matches `name` against the exposed effective
field members of that object type, own and inherited; for a
`model`/`record_value_type` node, `name` matches the fields its record
declaration declares. Operation members are not candidates. The field
entry refuses as follows, and a refusal is that entry's meaning-join
defect:

- a declaring node whose owner names no selected declaration of its kind,
  or that is not its own preimage, was refused earlier by FR-322's
  derived-shape key stage (`missing_declaration`/`missing-selection` or
  `invalid_package`/`stale-node-key`), so it reaches no field entry;
- no field of that name: `missing_declaration`/`missing-name`;
- two or more exposed fields of that name:
  `ambiguous_declaration`/`ambiguous-name`.

No `model` node stands for a field, an operation or a clause member on its
own. `ModelNode`'s `semantic_form` names FR-208's business type kinds
individually (`object_type`, `value_type`, `variant_type`,
`record_value_type`, `event_type`, `state_machine`, `process`,
`persistence_interface`, `namespace`), `model_import`, and FR-208's five
systems meanings (`systems_part`, `systems_port`, `systems_interface`,
`systems_connection` and `systems_allocation`, one per
`quire.meaning.systems.<x>/v1` value FR-208 lists and
[FR-152](../../functional/type-model/FR-152-bind-systems-model-structures.md)
binds). A member is named everywhere by its declaring node and its name.
A relationship is encoded exactly one way, `relation`/`relationship`, and a
`modifies` entry names a relationship only through that one node tag and
form. None of the five systems forms is an admitted declaring node for any
entry, so no systems meaning is a frame-entry target.

A field of an object type whose own type names a record value type, and a
field declared on that record value type itself, are separate field
entries: one names the object type's node, the other the record value
type's node, and `modifies` admits both in the same frame body.
`(Account, "address")`, a field of `Account` typed `Address`, and
`(Address, "street")`, a field of `Address`, are one such pair.

A `modifies` entry naming a relationship grants every link of that
relationship, observed from each end: a write is covered by the grant
whether it is read through the end whose object type declares the
relationship or through the relationship's other end, because the entry
names one node key that stands for the relationship as a whole.
[interface_004](interface_004-checked-package.md)'s Declaration section
already fixes this node key's own shape: a `relation` node is not among the
node tags that carry `declaration: {qualified_name}`, so a relationship's
wire encoding names no end as its own, and there is no qualified name for a
reader to resolve as the declaring end apart from the other.

## Outputs

Reading an admitted frame node yields exactly:

- the declaring object type of the operation (`semantic_type`);
- the set of declared fields and relationships the operation may write
  (`modifies`, membership-checked against every declared dependency's
  meaning and, for a field, resolved to one field);
- the set of declared object types and processes the operation may create
  (`creates`, membership-checked the same way);
- the set of declared object types and processes the operation may delete
  (`deletes`, membership-checked the same way);
- the scoped deletes the operation may perform (`deletes_scoped`): the
  receiver, or the argument bound to a named reference parameter;
- nothing else. A frame body with all four members empty is a valid
  output meaning the operation modifies, creates and deletes nothing.

A frame body missing any of the four stated members fails the schema, so
an admitted frame body's Outputs are never partial.

## Wire encoding

A frame node's `body` is exactly `{"term": "frame", "modifies": [...],
"creates": [...], "deletes": [...], "deletes_scoped": [...]}`; all four
members are required, and each may independently be the empty array. `modifies` is an array of
`FrameModifiesEntry` (`{"kind": "field", "declaration", "name", "scope"}` or
`{"kind": "relationship", "declaration"}`), and `creates` and `deletes` are
each an array of `NodeRef`, and `deletes_scoped` an array of
`FrameDeletesScopedEntry`; no member holds a duplicate entry. This shape
is scoped to validate only as the `body` of a `state`/`frame` node: a
`value`, `expression`, or any other node's `body` continues to validate
against the ordinary `SemanticTerm` union, and a `state`/`frame` node's
`body` validates only against this shape, never against `SemanticTerm`. A
frame body holds no application, so no clause operation is ever the body
root of a frame node ([FR-341](FR-341-checked-package-state-clause-body.md)
"Node forms and body roots"). The schema also requires every occurrence of
a `state`/`frame` node to have role `generated`.

A field entry's `scope` enters the frame node's identity, so changing an
entry from `every-object` to `receiver` changes the node id and the
`package_id`. A field entry with no `scope`, or a `scope` other than
`receiver` or `every-object`, fails the schema. Two field entries with the
same `declaration` and `name` and different scopes refuse
`invalid_model_binding`/`conflicting-binding`, naming both entries.

## Canonical ordering and reader precedence

Each entry has an order key: for a `modifies` entry, its `declaration`
digest (compared as unsigned bytes) and then its `name` as UTF-8 bytes,
with a relationship entry's name empty; for a `creates` or `deletes` entry,
its digest. Each of `modifies`, `creates` and `deletes` is independently
sorted ascending by that key, following the ascending-digest ordering
[FR-322](FR-322-checked-package-artifact.md) already uses for the
dependency lists of application-node preimages. A member array that is not
in this order refuses `invalid_semantic_graph`, the same code
[FR-322-AC-10](FR-322-checked-package-artifact.md) uses for every other
noncanonical member/term order.

A frame node whose `semantic_type` names no node of the package refuses
`missing_declaration`/`missing-name`, and one naming a node that is not
`model`/`object_type` refuses `invalid_model_binding`/
`malformed-declaration`, with that `semantic_type` as the locus. An entry
reference outside the graph, or naming no declared dependency of the frame
node, refuses `missing_declaration`/`missing-name`. An entry reference
naming a declared dependency of a meaning the entry cannot carry refuses
`invalid_model_binding`/`malformed-declaration`. These are exactly
`model-complete.md`'s Frames row causes. The locus of an entry's refusal is
its declaring node key. A field entry that passes those two checks then
resolves its name ("Field entries" above), and a resolution refusal is that
entry's defect.

When one frame node carries more than one defect at once, the reader
reports exactly one diagnostic: the one this fixed order, and only this
order, decides:

1. Any meaning-join defect (a `semantic_type` defect, or an entry's
   `missing-name`, `missing-selection`, `ambiguous-name` or
   `malformed-declaration`, in any of the three members) outranks any
   canonical-order defect.
2. Ties break by position, in the frame node's own fixed order:
   `semantic_type`, then `modifies`, then `creates`, then `deletes`.
3. Remaining ties, among multiple meaning-join defects in the same
   member, break by the ascending order key of the offending entry, the
   same order rule the member array itself must satisfy.

This precedence rule fixes exactly one diagnostic for a frame node carrying
several defects, regardless of which member happens to carry which defect
or where in its array an entry sits.

When a package publishes more than one `state`/`frame` node, the reader
checks them in ascending `node_id` digest order and reports the first one
carrying any defect, using that frame node's own precedence rule above.
The frame step runs before the state step of
[FR-341](FR-341-checked-package-state-clause-body.md) and
[FR-342](FR-342-checked-package-operation-anchor-body.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-340-AC-1 | `modifies`, `creates` and `deletes` may each independently be the empty array; an all-empty frame body and the published fixture's fully-populated frame body are both admitted with no defect. | Test (TC-233) |
| FR-340-AC-2 | Every relationship entry of `modifies` names a declared dependency of the frame node that is a `relation`/`relationship` node; every field entry's `declaration` names a declared dependency that is a `model`/`object_type` or `model`/`record_value_type` node; every `creates`/`deletes` entry names a declared dependency that is a `model`/`object_type` or `model`/`process` node. | Test (TC-233) |
| FR-340-AC-3 | An entry naming no node of the package, or no declared dependency of the frame node, refuses `missing_declaration`/`missing-name`; an entry naming a declared dependency of any other meaning -- a relationship entry naming an object type or a literal, a field entry on a process, a relationship or a systems node, a `creates` entry naming a relationship, and every `model`/`*_type`, `model`/`namespace`, `model`/`state_machine` or `model`/`persistence_interface` form an entry cannot carry -- refuses `invalid_model_binding`/`malformed-declaration`, with the entry's declaring node key as the locus. | Test (TC-233) |
| FR-340-AC-4 | Each of `modifies`, `creates` and `deletes` is duplicate-free and sorted ascending by entry order key (declaring node digest, then field name as UTF-8 bytes); a duplicated entry fails the schema, and a misordered array, including two field entries of one declaring node in descending name order, refuses `invalid_semantic_graph`. | Test (TC-233) |
| FR-340-AC-5 | The frame body shape validates only as a `state`/`frame` node's `body`; a `value` node's `body` carrying this shape, and a `state`/`frame` node's `body` carrying an ordinary `SemanticTerm` shape, both fail the published schema. | Test (TC-233) |
| FR-340-AC-6 | When one frame node carries a meaning-join defect and a canonical-order defect at once, the reported diagnostic is the meaning-join refusal the precedence rule selects, with its exact code, cause and locus, regardless of which member carries which defect; two field entries of one declaring node tie on the digest and break by field name; the same frame node reports that same selected diagnostic regardless of collection or scan order. | Test (TC-233) |
| FR-340-AC-7 | A frame body missing any of `modifies`, `creates` or `deletes` fails the schema; every published fixture's frame node is admitted with no defect by the same reader rule `read_package_semantics` applies. | Test (TC-233) |
| FR-340-AC-8 | `ModelNode.semantic_form` names each of FR-208's five systems meanings -- `systems_part`, `systems_port`, `systems_interface`, `systems_connection` and `systems_allocation`, one per `quire.meaning.systems.<x>/v1` value -- and none of the five is admitted as a field entry's declaring node or as a `creates` or `deletes` entry. | Test (TC-233) |
| FR-340-AC-9 | When a package publishes more than one `state`/`frame` node, the reader reports the first defect found while checking frame nodes in ascending `node_id` digest order; a lower-digest frame node's own canonical-order defect is reported even when a higher-digest frame node carries a meaning-join defect that would otherwise outrank it. | Test (TC-233) |
| FR-340-AC-10 | A field entry on an object type for a field typed with a record value type, and a field entry on that record value type's node for one of its own fields, are distinct entries; a frame body naming both is admitted with no defect. | Test (TC-233) |
| FR-340-AC-11 | A `modifies` entry naming a relationship is one node key, and a `relation`/`relationship` node carries no `declaration`; that single key is the whole grant, covering every link of the relationship observed from each end. | Test (TC-233) |
| FR-340-AC-12 | A frame node's `semantic_type` is the `model`/`object_type` node declaring its operation, and a `semantic_type` naming any other node refuses `invalid_model_binding`/`malformed-declaration` before any entry defect. | Test (TC-233) |
| FR-340-AC-13 | A field entry's name resolves among the exposed effective fields of its declaring object type, own and inherited: `(Order, "total")` and `(Sub, "total")` resolve to `Order`'s field; an undeclared name and an operation's name refuse `missing_declaration`/`missing-name`, a name two supertypes both expose refuses `ambiguous_declaration`/`ambiguous-name`, and a declaring node keyed under an unselected domain package version refuses `missing_declaration`/`missing-selection`. | Test (TC-280) |
| FR-340-AC-14 | A frame occurrence with a role other than `generated` fails the schema. | Test (TC-233) |
| FR-340-AC-15 | No `model` node stands for a field, an operation or a clause member on its own: `ModelNode.semantic_form` names only FR-208's business type kinds, `model_import` and FR-208's five systems meanings. | Test (TC-233) |
| FR-340-AC-16 | A field entry carries `scope` exactly `receiver` or `every-object`; an entry without it or with another value fails the schema, two entries for one field with different scopes refuse `invalid_model_binding`/`conflicting-binding`, and changing an entry's scope changes the frame node id and the `package_id`. | Test (TC-326) |
| FR-340-AC-17 | A frame body carries `deletes_scoped`, possibly empty; `deletes self` encodes as `{kind: "receiver"}` and `deletes p` as `{kind: "parameter", name: "p"}`; a body without the member fails the schema, a `parameter` entry naming no reference parameter refuses `missing_declaration`/`missing-name`, and a misordered array refuses `invalid_semantic_graph`. | Test (TC-326) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`
  property this artifact specifies the `state`/`frame` body of, including
  the reference-resolution, model-owned member resolution and
  canonical-order rules this artifact reuses.
- [FR-342](FR-342-checked-package-operation-anchor-body.md) binds each frame
  to its operation and requires the frame's `semantic_type` to equal the
  operation's context.
- [FR-013](../../functional/foundation/FR-013-validate-operation-frames.md)
  owns the runtime enforcement of an authored frame against observed
  populations; this artifact owns only the wire shape the frame is read
  from.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns
  "Effect frames" as a declared-artifact model feature (`Modifies:`,
  `Creates:`, `Deletes:` lines lowering to `Operation.frame`); this
  artifact is the checked-package V2 carrier of that same feature.
- `proposals/quire-v1/definitions/model-complete.md`'s Frames row is the
  business-model rule this artifact's `modifies`/`creates`/`deletes` shape
  and refusal causes carry unchanged onto the V2 wire.
- [FR-208](../foundation/FR-208-quire-meaning-vocabulary.md) fixes which
  business-model meanings (field, relationship, object type, process) are
  eligible frame-entry targets.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires this artifact's refusals to carry a stable catalogued cause and
  an exact available locus.
- [interface_004](interface_004-checked-package.md) publishes the schema
  this artifact's shape is normative against, under
  [`proposals/checked-package-v2`](../../../proposals/checked-package-v2/).

## Status

Accepted complete-V1 boundary value contract for the `state`/`frame` node
body. Implementation and qualification remain tracked separately.

## References

- Receiver scope design: quire-spec-language ADR-021 (POR-3, RU-1, QS-4).
- Scoped `deletes`: quire-spec-language SR-1140 FND-004.
- Tickets: Linear STD-134; paired QSL-368.
