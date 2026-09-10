---
id: TC-140
title: "Bind typed observations without implicit values"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: verifies
---
## Description

Planned OB01 controls for [FR-110](../functional/FR-110-bind-typed-observations.md). No executable test or completed coverage is claimed.

## Test Procedure

Bind a valid provider effect, then independently substitute its entity, signal, unit, trigger, and schema revision; remove a required field; and change internal/external visibility.

## Expected Results

Only the valid record is admitted with exact identities. Each substitution is typed refusal, missing value is incomplete/refused rather than false, and visibility remains retained.
