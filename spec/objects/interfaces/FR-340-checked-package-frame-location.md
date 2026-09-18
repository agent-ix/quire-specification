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
each naming declarations of the package rather than access paths into a
runtime value. This wire encoding is the V2 carrier of that existing model;
it does not define a second, competing frame shape.

## Properties

- **modifies** (array of `NodeRef`) — every field or relationship the
  operation may write. Each entry is the node key of a declared
  `relation`/`relationship` node or a declared `model`/`model_declaration`
  node of the package's semantic graph.
- **creates** (array of `NodeRef`) — every object type or process the
  operation may create an instance of. Each entry is the node key of a
  declared `model`/`model_type` node.
- **deletes** (array of `NodeRef`) — every object type or process the
  operation may delete an instance of. Each entry is the node key of a
  declared `model`/`model_type` node, using the same closed meaning as
  `creates`.
- Every entry of every member equals one of the frame node's own
  `dependencies` ([FR-322](FR-322-checked-package-artifact.md): "References
  name a compatible declared target by node key"); an entry naming no
  dependency of the frame node, or no node of the package at all, is
  refused. An entry naming a declared dependency of any other meaning —
  including a `value`, `expression` or any node outside the four listed
  above — is refused. This is the rule that makes a location-based frame
  body, such as one rooting a field selection at a literal, unrepresentable:
  a literal has no frame meaning.

Mapping `modifies`/`creates`/`deletes` targets onto the existing `model`/
`relation` node tags and forms above (`model_declaration` for a field,
`model_type` for an object type or a process, `relation`/`relationship` for
a relationship) is this artifact's own wiring of FR-208's closed business-
model meanings onto the checked-semantic-graph's existing closed node-tag
and semantic-form vocabulary. FR-208 and `model-complete.md` fix which
meanings are eligible; they do not name checked-semantic-graph node tags.
No new node tag or semantic form is introduced to carry them.

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
every other noncanonical member/term order; canonical order is not a
distinct new refusal category.

A reference outside the graph, or naming no declared dependency of the
frame node, refuses `missing_declaration`/`missing-name`. A reference
naming a declared dependency of a meaning the member cannot carry refuses
`invalid_model_binding`/`malformed-declaration`. These are exactly
`model-complete.md`'s Frames row causes; no new cause is invented for the
meaning-join check.

When one frame body carries more than one defect at once, the reader
reports exactly one diagnostic, decided in this fixed order, never by
which member array an implementation happens to scan first or by an
entry's position inside its array:

1. Any meaning-join defect (a `missing-name` or `malformed-declaration`
   entry, in any of the three members) outranks any canonical-order
   defect.
2. Ties break by member: `modifies` before `creates` before `deletes`.
   This is the frame body's own fixed member order, a spec fact, not an
   authored array position.
3. Remaining ties, among multiple meaning-join defects in the same
   member, break by the ascending digest of the offending entry, the same
   order rule the member array itself must satisfy.

A reader that instead reported "whichever defect its traversal reaches
first" would report different diagnostics for two frame bodies that carry
the same two defects arranged across different members; this precedence
rule fixes one diagnostic regardless of that arrangement.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-340-AC-1 | `modifies`, `creates` and `deletes` may each independently be the empty array; an all-empty frame body and the published fixture's fully-populated frame body are both admitted with no defect. | Test (TC-233) |
| FR-340-AC-2 | Every `modifies` entry names a declared dependency of the frame node that is a `relation`/`relationship` or `model`/`model_declaration` node; every `creates`/`deletes` entry names a declared dependency that is a `model`/`model_type` node. | Test (TC-233) |
| FR-340-AC-3 | An entry naming no node of the package, or no declared dependency of the frame node, refuses `missing_declaration`/`missing-name`; an entry naming a declared dependency of any other meaning refuses `invalid_model_binding`/`malformed-declaration`; a location-based frame body rooting a field selection at a literal is unrepresentable under this rule. | Test (TC-233) |
| FR-340-AC-4 | Each of `modifies`, `creates` and `deletes` is duplicate-free and sorted ascending by entry digest; a duplicated entry fails the schema and a misordered array refuses `invalid_semantic_graph`. | Test (TC-233) |
| FR-340-AC-5 | The frame body shape validates only as a `state`/`frame` node's `body`; a `value` node's `body` carrying this shape, and a `state`/`frame` node's `body` carrying an ordinary `SemanticTerm` shape, both fail the published schema. | Test (TC-233) |
| FR-340-AC-6 | When one frame body carries a meaning-join defect and a canonical-order defect at once, the reported diagnostic is the meaning-join defect regardless of which member carries which defect; two vectors with the same two defects assigned to different members report the same kind of diagnostic. | Test (TC-233) |
| FR-340-AC-7 | A frame body missing any of `modifies`, `creates` or `deletes` fails the schema; every published fixture's frame node is admitted with no defect by the same reader rule `read_package_semantics` applies. | Test (TC-233) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`
  property this artifact specifies the `state`/`frame` body of, including
  the reference-resolution and canonical-order rules this artifact reuses
  rather than redefines.
- [FR-013](../../functional/foundation/FR-013-validate-operation-frames.md)
  owns the runtime enforcement of an authored frame against observed
  populations; this artifact owns only the wire shape the frame is read
  from.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns
  "Effect frames" as a declared-artifact model feature (`Modifies:`,
  `Creates:`, `Deletes:` lines lowering to `Operation.frame`); this
  artifact is the checked-package V2 carrier of that same feature, not an
  alternative to it.
- `proposals/quire-v1/definitions/model-complete.md`'s Frames row is the
  business-model rule this artifact's `modifies`/`creates`/`deletes` shape
  and refusal causes carry unchanged onto the V2 wire.
- [FR-208](../foundation/FR-208-quire-meaning-vocabulary.md) fixes which
  business-model meanings (field, relationship, object type, process) are
  eligible frame-entry targets.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires this artifact's refusals to carry a stable catalogued cause and
  an exact available locus rather than message text.
- [interface_004](interface_004-checked-package.md) publishes the schema
  this artifact's shape is normative against, under
  [`proposals/checked-package-v2`](../../../proposals/checked-package-v2/).

## Status

Accepted complete-V1 boundary value contract for the `state`/`frame` node
body. Implementation and qualification remain tracked separately.
