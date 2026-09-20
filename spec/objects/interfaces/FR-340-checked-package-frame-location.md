---
id: FR-340
title: "Checked-package frame body"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
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
already state: three named members, `modifies`, `creates` and `deletes`,
each naming declarations of the package by node key. This wire encoding is
the V2 carrier of that existing model.

## Properties

- **modifies** (array of `NodeRef`) — every field or relationship the
  operation may write. Each entry is the node key of a declared
  `relation`/`relationship` node or a declared `model`/`field_declaration`
  node of the package's semantic graph.
- **creates** (array of `NodeRef`) — every object type or process the
  operation may create an instance of. Each entry is the node key of a
  declared `model`/`object_type` or `model`/`process` node.
- **deletes** (array of `NodeRef`) — every object type or process the
  operation may delete an instance of. Each entry is the node key of a
  declared `model`/`object_type` or `model`/`process` node, using the same
  closed meaning as `creates`.
- Every entry of every member equals one of the frame node's own
  `dependencies` ([FR-322](FR-322-checked-package-artifact.md): "References
  name a compatible declared target by node key"); an entry naming no
  dependency of the frame node, or no node of the package at all, refuses
  `missing_declaration`/`missing-name`. An entry naming a declared
  dependency of any other meaning refuses `invalid_model_binding`/
  `malformed-declaration`: a `model`/`operation_declaration` or
  `model`/`clause_member_declaration` node; a `model`/`value_type`,
  `model`/`variant_type`, `model`/`record_value_type`, `model`/`event_type`,
  `model`/`state_machine`, `model`/`persistence_interface` or
  `model`/`namespace` node; or a `value`, `expression`, or any node outside
  the two forms each member admits. A `value` literal has no frame meaning
  at all.

`ModelNode`'s `semantic_form` names each of FR-208's business-model
meanings individually: `field_declaration`, `operation_declaration` and
`clause_member_declaration` for the three declaration kinds a `model` node
other than a relationship carries, and `object_type`, `value_type`,
`variant_type`, `record_value_type`, `event_type`, `state_machine`,
`process`, `persistence_interface` and `namespace` for the nine business
type kinds ([FR-208](../foundation/FR-208-quire-meaning-vocabulary.md)'s
tenth, `population`, wires separately as `relation`/`population`). Frame
entry eligibility is checked directly against an entry's own
`semantic_form`: `modifies` admits `field_declaration`; `creates` and
`deletes` admit `object_type` and `process`; every other `model` form is
refused. A relationship is encoded exactly one way, `relation`/
`relationship`, and a `modifies` entry names a relationship only through
that one node tag and form.

A field declared on an object type whose own type names a record value
type, and a field declared on that record value type itself, are each a
separate `model`/`field_declaration` node: one is a declaration of the
object type, scoped to it; the other is a declaration of the record type,
shared by every field of that type wherever it occurs. `modifies` admits
both node keys in the same frame body, each independently eligible under
the rule above, because they are distinct declaration references naming
different declarations. `Account.address`, a field of `Account` typed
`Address`, and `Address.street`, a field of `Address`, are one such pair.

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

`ModelNode`'s `semantic_form` also names each of FR-208's five systems
meanings individually: `systems_part`, `systems_port`, `systems_interface`,
`systems_connection` and `systems_allocation`, one per
`quire.meaning.systems.<x>/v1` value FR-208 lists and
[FR-152](../../functional/type-model/FR-152-bind-systems-model-structures.md)
binds (Part, Port, Interface, Connection, Allocation). None of the five is a
`field_declaration`, `object_type` or `process`, so frame entry eligibility
above already refuses every one of them the same way it refuses every other
non-admitted `model` form; no systems meaning is a frame-entry target.

## Outputs

Reading an admitted frame node yields exactly:

- the set of declared fields and relationships the operation may write
  (`modifies`, membership-checked against every declared dependency's
  meaning);
- the set of declared object types and processes the operation may create
  (`creates`, membership-checked the same way);
- the set of declared object types and processes the operation may delete
  (`deletes`, membership-checked the same way);
- nothing else. A frame body with all three members empty is a valid
  output meaning the operation modifies, creates and deletes nothing.

A frame body missing any of the three stated members fails the schema, so
an admitted frame body's Outputs are never partial.

## Wire encoding

A frame node's `body` is exactly `{"term": "frame", "modifies": [...],
"creates": [...], "deletes": [...]}`; all three members are required, and
each may independently be the empty array. `modifies`, `creates` and
`deletes` are each an array of `NodeRef` with no duplicate entry. This
shape is scoped to validate only as the `body` of a `state`/`frame` node:
a `value`, `expression`, or any other node's `body` continues to validate
against the ordinary `SemanticTerm` union, and a `state`/`frame` node's
`body` validates only against this shape, never against `SemanticTerm`.

## Canonical ordering and reader precedence

Each of `modifies`, `creates` and `deletes` is independently sorted
ascending by entry `NodeRef.digest` (compared as unsigned bytes), following
the ascending-digest ordering [FR-322](FR-322-checked-package-artifact.md)
already uses for the dependency lists of application-node preimages. A
member array that is not in this order refuses `invalid_semantic_graph`,
the same code [FR-322-AC-10](FR-322-checked-package-artifact.md) uses for
every other noncanonical member/term order.

A reference outside the graph, or naming no declared dependency of the
frame node, refuses `missing_declaration`/`missing-name`. A reference
naming a declared dependency of a meaning the member cannot carry refuses
`invalid_model_binding`/`malformed-declaration`. These are exactly
`model-complete.md`'s Frames row causes. A reference naming no node of the
package at all and a reference naming a real node that is not a declared
dependency of the frame node both refuse `missing_declaration`/
`missing-name`, the same cause.

When one frame body carries more than one defect at once, the reader
reports exactly one diagnostic: the one this fixed order, and only this
order, decides:

1. Any meaning-join defect (a `missing-name` or `malformed-declaration`
   entry, in any of the three members) outranks any canonical-order
   defect.
2. Ties break by member, in the frame body's own fixed member order:
   `modifies`, then `creates`, then `deletes`.
3. Remaining ties, among multiple meaning-join defects in the same
   member, break by the ascending digest of the offending entry, the same
   order rule the member array itself must satisfy.

This precedence rule fixes exactly one diagnostic for a frame body carrying
several defects, regardless of which member happens to carry which defect
or where in its array an entry sits.

When a package publishes more than one `state`/`frame` node, the reader
checks them in ascending `node_id` digest order and reports the first one
carrying any defect, using that frame node's own precedence rule above.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-340-AC-1 | `modifies`, `creates` and `deletes` may each independently be the empty array; an all-empty frame body and the published fixture's fully-populated frame body are both admitted with no defect. | Test (TC-233) |
| FR-340-AC-2 | Every `modifies` entry names a declared dependency of the frame node that is a `relation`/`relationship` or `model`/`field_declaration` node; every `creates`/`deletes` entry names a declared dependency that is a `model`/`object_type` or `model`/`process` node. | Test (TC-233) |
| FR-340-AC-3 | An entry naming no node of the package, or no declared dependency of the frame node, refuses `missing_declaration`/`missing-name`; an entry naming a declared dependency of any other meaning — including `model`/`operation_declaration`, `model`/`clause_member_declaration`, and every other `model`/`*_type`, `model`/`namespace`, `model`/`state_machine` or `model`/`persistence_interface` form — refuses `invalid_model_binding`/`malformed-declaration`. | Test (TC-233) |
| FR-340-AC-4 | Each of `modifies`, `creates` and `deletes` is duplicate-free and sorted ascending by entry digest; a duplicated entry fails the schema and a misordered array refuses `invalid_semantic_graph`. | Test (TC-233) |
| FR-340-AC-5 | The frame body shape validates only as a `state`/`frame` node's `body`; a `value` node's `body` carrying this shape, and a `state`/`frame` node's `body` carrying an ordinary `SemanticTerm` shape, both fail the published schema. | Test (TC-233) |
| FR-340-AC-6 | When one frame body carries a meaning-join defect and a canonical-order defect at once, the reported diagnostic is the meaning-join refusal the precedence rule selects, with its exact code, cause and locus, regardless of which member carries which defect; the same frame body reports that same selected diagnostic regardless of collection or scan order. | Test (TC-233) |
| FR-340-AC-7 | A frame body missing any of `modifies`, `creates` or `deletes` fails the schema; every published fixture's frame node is admitted with no defect by the same reader rule `read_package_semantics` applies. | Test (TC-233) |
| FR-340-AC-8 | `ModelNode.semantic_form` names each of FR-208's five systems meanings -- `systems_part`, `systems_port`, `systems_interface`, `systems_connection` and `systems_allocation`, one per `quire.meaning.systems.<x>/v1` value -- and none of the five is admitted into `modifies`, `creates` or `deletes`. | Test (TC-233) |
| FR-340-AC-9 | When a package publishes more than one `state`/`frame` node, the reader reports the first defect found while checking frame nodes in ascending `node_id` digest order; a lower-digest frame node's own canonical-order defect is reported even when a higher-digest frame node carries a meaning-join defect that would otherwise outrank it. | Test (TC-233) |
| FR-340-AC-10 | A `modifies` entry naming a field declared on an object type whose own type names a record value type, and a `modifies` entry naming a field declared on that record value type itself, are distinct declaration references; a frame body naming both is admitted with no defect. | Test (TC-233) |
| FR-340-AC-11 | A `modifies` entry naming a relationship is one node key, and a `relation`/`relationship` node carries no `declaration`; that single key is the whole grant, covering every link of the relationship observed from each end. | Test (TC-233) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`
  property this artifact specifies the `state`/`frame` body of, including
  the reference-resolution and canonical-order rules this artifact reuses.
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
