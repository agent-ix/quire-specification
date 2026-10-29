---
id: TC-430
title: "Withdrawn: node-key preimage integers"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-430: Withdrawn: node-key preimage integers

## Description

Withdrawn: node-key preimages are removed. A node id is the structural triple
`{package, path, ordinal}` of FR-322 and is never hashed, so no integer is
spelled into a preimage. The id and file name are kept so that the id is never
reused.

## Test Procedure

None. Withdrawn.

## Expected Results

None. Withdrawn.
