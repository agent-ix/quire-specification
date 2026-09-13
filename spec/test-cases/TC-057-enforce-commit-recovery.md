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

## Expected Results

Only matching linked relations report recovery. Equal predicate text under
distinct compensation handles remains unequal, changing the typed expression
handle selects another relation, operand changes do not, and missing/mismatched
semantic dependencies refuse linking. Boundary,
occurrence and observation identities remain separate; `never` has no default
identity; incompatible observation rebinding refuses; key permutation changes
no causal verdict. The three ordering mutants yield distinct violations, while
missing inputs remain pending/incomplete rather than successful.
