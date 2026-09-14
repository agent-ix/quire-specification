---
id: TC-057
title: "Check commit ordering and recovery meaning"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-057
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-210
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-237
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-238
    type: verifies
---
## Description

Verify that compensation success and authored recovery remain separate and commit ordering is enforced.

## Test Procedure

Evaluate full and partial refund effects against full and partial recovery
predicates. Compare commit boundary, boundary-occurrence and commit-observation
keys; exercise explicit `never`; rebind one commit-observation key to another
boundary and permute key order. Compare equal recovery-predicate text under two
declaration-local compensation handles, change the typed recovery-expression
handle and target/achieved operands independently under one definition, and
remove or mismatch a selected model/state/temporal/clock dependency. Move
registration after commit, compensation before the forward effect and
compensation after the commit; separately remove target state, commit or effect
input.

For two commit nodes, select one with `commit Node`, establish each node in turn
and place registration, attempt, effect and recovery after it. Repeat with
`commit never`; remove evidence for the selected non-`never` commit without
changing the static selection.

## Expected Results

Only matching linked relations report recovery. Equal predicate text under
distinct compensation handles remains unequal, changing the typed expression
handle selects another relation, operand changes do not, and missing/mismatched
semantic dependencies refuse linking. Boundary,
occurrence and observation identities remain separate; `never` has no default
identity; incompatible observation rebinding refuses; key permutation changes
no causal verdict. The three ordering mutants yield distinct violations, while
missing inputs remain pending/incomplete rather than successful.
Only the selected commit produces the applicable after-commit violations;
another commit does not. `commit never` supplies no earlier cutoff and no
unbounded closure. Missing evidence for a selected boundary remains
pending/incomplete rather than granting permission.
