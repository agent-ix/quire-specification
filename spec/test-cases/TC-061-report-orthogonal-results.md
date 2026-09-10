---
id: TC-061
title: "Discriminate protocol result dimensions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: verifies
---
## Description

Verify the shared result contract and its monotonic response to missing evidence.

## Test Procedure

Construct healthy, violating, inactive, pending, incomplete, refused, unsupported and failed results. Exercise every valid row of the normative result-combination table, then mutate each constrained dimension independently. Delete each trigger, role, provider observation, population member and adverse case. Mutate every provenance identity.

## Expected Results

All valid boundary combinations admit and all one-axis forbidden combinations refuse. Result states remain structurally discriminable; missing evidence never improves truth, adequacy or package status; strict readers refuse stale identities while retaining independent valid results.
