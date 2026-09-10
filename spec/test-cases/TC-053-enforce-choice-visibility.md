---
id: TC-053
title: "Refuse unobservable or ambiguously owned choice"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-053
    type: verifies
---
## Description

Verify that a role can own a choice only when its branches are labeled, exclusive and distinguishable from visible facts.

## Test Procedure

Admit ship/cancel after fulfillment receives a payment outcome. Remove that message while leaving the fact at the provider, then mutate owner count, labels and guard overlap one axis at a time.

## Expected Results

The communicated choice admits. Every other case returns the exact choice/owner/fact refusal and no successful projection or conformance claim.

