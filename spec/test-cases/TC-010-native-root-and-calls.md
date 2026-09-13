---
id: TC-010
title: "Require a defined Boolean root and admitted calls"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-019
    type: verifies
---
## Description

Exercise root typing and prohibited effects/calls through actual native intake.
Planned composed state-profile acceptance, with independent refused controls.

## Test Procedure

Compile a defined Boolean root as the positive control. Offer integer/text/option
roots, unguarded potentially invalid values, nullable results, helper signatures
and calls, recursion, reflection, I/O and ambient-store forms. Put prohibited
constructs behind a false branch as well as an evaluated branch. Lower a valid
execution budget and retry with adequate limits.

## Expected Results

Only the defined Boolean root produces a checked clause. Prohibited constructs
refuse at their original locus even when unreachable. Exhaustion remains
incomplete and never becomes false; a fresh adequately budgeted run can complete.
