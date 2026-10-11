---
id: TC-933
title: "ContractPackage records one terminal disposition per requested item"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-6
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-195-AC-6
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-195
    type: verifies
---
# TC-933: ContractPackage records one terminal disposition per requested item

## Description

Verify the per-item accounting bijection across all seven dispositions: the
five ordinary outcomes and two defensive walk outcomes.

## Test Procedure

Submit eight requested items in a fixed order: two independently lowerable
items, one unsupported item, one item requiring a bound, one `CheckedNodeId`
absent from the checked package, one item whose lowering reaches a typed
failure, one item whose body re-validation returns a typed refusal, and one
item whose body re-validation stops at a limit. Use fault-injection inputs for
the last two defensive cases. Before reading the emitted wire, compare each
producer record against the independent expected value and injected typed
refusal/incomplete value; check exact field names, types, values, and omitted
optional members. Then pass the unmodified wire through the FR-330 reader and
inspect graph roots and sibling records. Exercise closed-shape mutations by
omitting a required member, adding an unknown member, writing `null` for an
omitted optional member, changing a member's kind, omitting one accounting
record, duplicating one item, changing one outcome tag, and attaching a node
to a non-`lowered` record. Recompute identity for shape-only mutations so the
reader reaches the nested schema check. Separately change one typed payload
value without updating identity.

## Expected Results

There are exactly eight records in request order, each naming its exact
`CheckedNodeId` once. The two supported items each have one `lowered` record
and graph root; the remaining records exercise exactly one each of
`unsupported`, `requires_bound`, `invalid_input`, `failed`, `invalid_body`, and
`body_incomplete`, with their specified payloads and no substitute node. The
producer emits the exact fault-injected typed values using FR-330's nested
wire forms. Its `invalid_body.refusal` is a closed object with typed `code`,
optional `path`, `cause`, `locus`, `contract_version`, and `document_pointer`;
each optional member is present exactly when its typed value is present. Its
`body_incomplete.incomplete` is a closed object with typed `limit_kind`,
canonical decimal-string `limit` and `consumed`, and `path` exactly when the
typed value has one. The unmodified ContractPackage is admitted. Shape
mutations with recomputed identity are refused with the FR-330 nested-schema
cause corresponding to the defect. A payload-value mutation with the old
identity is refused for identity mismatch; recomputing identity for that new
value is outside this test because the reader checks canonical structure and
identity, not the truth of a producer's accounting. The two successful
sibling records remain unchanged.
