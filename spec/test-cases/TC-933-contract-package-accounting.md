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
the last two defensive cases. Inspect `feature_accounting`, graph roots and
sibling records. Mutate the result by omitting one record, duplicating one
item, changing one outcome tag, and attaching a node to a non-`lowered` record.

## Expected Results

There are exactly eight records in request order, each naming its exact
`CheckedNodeId` once. The two supported items each have one `lowered` record
and graph root; the remaining records exercise exactly one each of
`unsupported`, `requires_bound`, `invalid_input`, `failed`, `invalid_body`, and
`body_incomplete`, with their specified payloads and no substitute node. The
two defensive records retain the exact FR-322 refusal and incomplete value.
The two successful sibling records remain unchanged. Every mutation is
refused as a typed accounting error before consumption.
