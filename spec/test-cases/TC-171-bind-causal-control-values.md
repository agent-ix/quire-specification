---
id: TC-171
title: "Preserve causal structure and definite value availability"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-221
    type: verifies
---
## Description

Planned source/consumer correspondence checks for the native choreography
surface. No executable parser, replay or monitoring coverage is claimed.

## Test Procedure

Use two interleaved shipment branches with an explicit all-branch join, complementary owned-choice guards and a repeat maximum of three with an exhausted path. Compare independently authored expected causal/availability graphs. Move a value read before the join, outside one choice branch, outside its iteration and onto an await timeout path. Remove a join input or replace a repeating action with an empty/check-only group; also exercise repeat maximum zero. Admit a repeat with no maximum and no invariant whose body holds a step.

Project the admitted occurrence schema. Bind one valid key for a role-owned node
inside nested repeats and one structural node outside every repeat. Independently
change the definition, workflow, control and role components; omit/add/reorder an
ordinal; use each boundary and one-outside ordinal; omit the ordinal sequence;
and sort occurrence keys without changing the authored graph.

## Expected Results

Only definite predecessor values are available. The declared interleavings and repeat paths retain their source/iteration identities; no presentation order adds causality. Each unavailable value or invalid join/progress mutation refuses linking. A true guard at the authored repeat limit selects the explicit exhausted path, without silently forcing the guard false. The repeat with no maximum is admitted with no exhausted path, and the empty or check-only repeat refuses as zero-progress.

The valid nested key retains every dimension outer-to-inner, and the non-repeat
key retains a present empty ordinal sequence. Wrong role, missing/surplus/
reordered/out-of-domain ordinals and absent sequence refuse; lexicographic key
order adds no causal edge.
