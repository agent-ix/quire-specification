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
inspect graph roots and sibling records. Write each valid-JSON mutation back
as canonical RFC 8785 bytes; keep the existing stale-identity case distinct.
Exercise nested closed-shape
mutations by omitting a required member, adding an unknown member, writing
`null` for an omitted optional member, and changing a member's kind. Do not
recompute identity for these invalid shapes: the reader checks nested schema
before identity. Separately mutate the accounting array by omitting its
`unsupported` record and its matching `source_map.refusals` entry, duplicating
the `unsupported` record, changing `failed` to `invalid_input` while retaining
the `failed` payload and changing its matching `source_map.refusals`
disposition, and adding a `node` member to an `unsupported` record; do
not recompute identity because accounting consistency and variant shape are
checked before identity. Separately change `invalid_body.refusal.path` to
another valid RFC 6901 pointer, preserving shape and request correspondence,
without updating identity.

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
typed value has one. The unmodified ContractPackage is admitted. Nested shape
mutations are refused as `invalid_package` with the FR-322 `missing-member`,
`unknown-member`, or `wrong-value-kind` cause matching the defect; `null` is
`wrong-value-kind`. Omitting a record or duplicating an item is refused as
`invalid_package`/`invalid-value` for request/accounting mismatch before
identity validation. Changing `failed` to `invalid_input` while retaining its
payload is refused as `invalid_package`/`unknown-member`; attaching an emitted
node to an `unsupported` record is refused as `invalid_package`/`unknown-member`
because non-lowered variants forbid node payloads. A well-shaped `path`
mutation with the old identity is refused as FR-330 specifies:
`stale_dependency`/`content-mismatch`, with the declared identity as
`expected` and recomputed identity as `actual`; recomputing identity for that
new value is outside this test because the reader checks canonical structure
and identity, not the truth of a producer's accounting. The two successful
sibling records remain unchanged.
