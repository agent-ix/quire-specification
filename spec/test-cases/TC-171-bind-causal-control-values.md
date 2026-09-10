---
id: TC-171
title: "Preserve causal structure and definite value availability"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: verifies
---
## Description

Planned source/consumer correspondence checks for the native choreography
surface. No executable parser, replay or monitoring coverage is claimed.

## Test Procedure

Use two interleaved shipment branches with an explicit all-branch join, complementary owned-choice guards and a repeat maximum of three with an exhausted path. Compare independently authored expected causal/availability graphs. Move a value read before the join, outside one choice branch, outside its iteration and onto an await timeout path. Remove a join input or replace a repeating action with an empty/check-only group; also exercise repeat maximum zero.

## Expected Results

Only definite predecessor values are available. The declared interleavings and repeat paths retain their source/iteration identities; no presentation order adds causality. Each unavailable value or invalid join/progress mutation refuses linking. A true guard at the authored repeat limit selects the explicit exhausted path, without silently forcing the guard false.
