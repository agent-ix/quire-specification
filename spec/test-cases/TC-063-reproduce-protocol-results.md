---
id: TC-063
title: "Reproduce results under equivalent presentations"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/NFR-021
    type: verifies
---
## Description

Verify deterministic semantic results while preserving authored order and detecting semantic identity changes.

## Test Procedure

Permute map-like members and causally independent events for one admitted case, retaining authored sequences. Repeat with each source/profile/model/protocol/request/binding/input/progress/backend dependency changed independently.

## Expected Results

Equivalent presentations produce identical ordered semantic results. Every semantic dependency change alters the corresponding identity; no reproduction omits its exact provenance.
