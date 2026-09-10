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

Construct healthy, violating, inactive, pending, incomplete, refused, unsupported and failed results. Delete each trigger, role, provider observation, population member and adverse case independently. Mutate each provenance identity and every forbidden dimension combination.

## Expected Results

All result states remain structurally discriminable. Missing evidence never improves truth, adequacy or package status. Strict readers refuse every stale identity and impossible combination while retaining independent valid results.

