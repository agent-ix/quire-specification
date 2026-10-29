---
id: TC-004
title: "Bind a disposition without a circular core digest"
type: TC
relationships:
  - target: "ix://agent-ix/quire-specification/FR-018"
    type: verifies
---
# TC-004: Bind a disposition without a circular core digest

## Description

Accepted portable contract control for FR-018-AC-5. The fixture core and
disposition are explicitly synthetic correspondence inputs, not a B report or
actual external human evidence.

## Test Procedure

Validate the complete fixture core against its closed fixture profile before
passing that core alone to B's JCS functions. Compare with the recorded canonical
bytes/digest, then attach the separate disposition and recompute from the same
selected core. Independently mutate a core field, disposition core digest, its
prefix from sha256-jcs to sha256, and disposition reference role to observation.
Finally repeat the projection
rule through B's real report profile/assembler under IT-003-SC-03 when available.

## Expected Results

Attaching the matching disposition leaves the core digest unchanged; hashing the
whole wrapper gives a different value. A changed core or foreign disposition
fails correspondence, raw digest substitution refuses, and observation cannot
substitute for review-disposition. No synthetic fixture is counted as an actual
human disposition or completed portfolio; unavailable B report execution remains
planned rather than passing.
