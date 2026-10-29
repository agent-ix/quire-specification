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
the last two defensive cases. In separate runs, inject one typed
`invalid_package`/`invalid-value` refusal with `path` equal to the empty root
pointer and `document_pointer` absent, and another
`stale_dependency`/`content-mismatch` refusal with `document_pointer` equal to
the empty root pointer and `path` set to `/lock/dependency_selections/0/package_id`.
Also inject refusals with each optional pointer absent. Inject a non-byte incomplete value
whose typed path is the empty root pointer, and a byte-limit value whose typed
path is absent. Before reading the emitted wire, compare each
producer record against the independent expected value and injected typed
refusal/incomplete value; check exact field names, types, values, and omitted
optional members. Then pass the unmodified wire through the FR-330 reader and
inspect graph roots and sibling records. Write each valid-JSON mutation back
as canonical RFC 8785 bytes.
Exercise nested closed-shape mutations by omitting the required `code` member
from `invalid_body.refusal` in array index 6, adding `extra: true` to that
object, setting `invalid_body.refusal.path` to `null` when the injected typed
refusal omits it, and setting the allowed `refusal.code` member to a JSON
number. Do not
recompute identity for these invalid shapes: the reader checks nested schema
before identity. Separately mutate the accounting array by omitting its
`unsupported` record and its matching `source_map.refusals` entry while
retaining the `source_map.requested_items` entry for that requested id, duplicating
the `lowered` record, changing `failed` to `invalid_input` while retaining
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
typed value has one; an empty typed pointer is preserved as `""`, while an
absent typed pointer is omitted. The unmodified ContractPackage is admitted. Nested shape
mutations are refused as `invalid_package` with the FR-322 `missing-member`,
`unknown-member`, or `wrong-value-kind` cause matching the defect; `null` in
the allowed `path` member is `wrong-value-kind`. The mutation that removes the
`unsupported` accounting and refusal rows still has a total requested-item
source map; it is refused for the missing accounting/request bijection, not
for missing requested-item provenance. Omitting a record or duplicating an item is refused as
`invalid_package`/`invalid-value` at `/feature_accounting` for
request/accounting mismatch before identity validation. The missing `code`
member is refused as `invalid_package`/`missing-member` at
`/feature_accounting/6/refusal`; the extra `refusal.extra` is refused as
`invalid_package`/`unknown-member` at `/feature_accounting/6/refusal`;
the null `path` is refused as `wrong-value-kind` at
`/feature_accounting/6/refusal/path`; the numeric `code` is refused as
`wrong-value-kind` at `/feature_accounting/6/refusal/code`. Changing `failed` to
`invalid_input` while retaining its payload is refused as
`invalid_package`/`unknown-member` at `/feature_accounting/5`; attaching an
emitted node to an `unsupported` record is refused as
`invalid_package`/`unknown-member` at `/feature_accounting/2` because
non-lowered variants forbid node payloads. A well-shaped `path`
mutation with the old identity is refused as
`invalid_package`/`invalid-value` at `/identity/digest`; recomputing identity for that
new value is outside this test because the reader checks canonical structure
and identity, not the truth of a producer's accounting. The two successful
sibling records remain unchanged.
