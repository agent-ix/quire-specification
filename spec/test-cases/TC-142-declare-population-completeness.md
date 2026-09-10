---
id: TC-142
title: "Declare population and completeness"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-112
    type: verifies
---
## Description

Planned OB01 population controls. No executable test or completed coverage is claimed.

## Test Procedure

Assess a finite snapshot and an event-time window under Producer interface 1.2.0;
change one identity; remove one required member; remove the external
payment-provider record; remove closure input; and independently mutate producer
version, membership digest, configuration digest, and closure digest.
Repeat with adjacent half-open windows: a member at the first window's `start`,
one exactly at its `end`, and the second window starting at that same instant.

## Expected Results

Scope identity changes the subject. Missing membership, provider evidence, and
closure remain explicit incomplete/refused reasons. Every producer-selection
mismatch is refused without an ambient default and cannot establish aggregate
success. The start member occurs once in the first window; the boundary member
occurs only in the second window and cannot be double-counted.
