---
id: TC-005
title: "Select replacement native definitions without fallback"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-001
    type: verifies
---
## Description

Distinguish the adopted historical definition from the proposed replacement.
Planned compiler acceptance under #30; this is not an executed standard test.

## Test Procedure

Compare historical definition/fixture bytes with e897f81. Select the unavailable
replacement through the actual parser/package reader and assert a located
refusal. After explicit adoption, select its exact version/digests and mutate
each independently; attempt replay of an old semantic subject under the new
selection. Use a valid selected definition as the positive control.

## Expected Results

Historical bytes and subjects retain their meaning. An unavailable or mismatched
definition yields no checked replacement clause and never falls back. A supported
exact replacement selection binds its own definition and new derivation.
