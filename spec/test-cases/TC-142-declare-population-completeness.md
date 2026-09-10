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

Assess a finite snapshot and an event-time window; change one identity; remove one required member; remove the external payment-provider record; and remove closure input.

## Expected Results

Scope identity changes the subject. Missing membership, provider evidence, and closure remain explicit incomplete/refused reasons and cannot establish aggregate success.

