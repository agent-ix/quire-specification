---
id: TC-057
title: "Check commit ordering and recovery meaning"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-057
    type: verifies
---
## Description

Verify that compensation success and authored recovery remain separate and commit ordering is enforced.

## Test Procedure

Evaluate full and partial refund effects against full and partial recovery predicates. Move registration after commit, compensation before the forward effect and compensation after the commit; separately remove target state, commit or effect input.

## Expected Results

Only matching relations report recovery. The three ordering mutants yield distinct violations, while missing inputs remain pending/incomplete rather than successful.
