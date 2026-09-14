---
id: TC-150
title: "Account for every output obligation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-121
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-297
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-298
    type: verifies
---
## Description

Accepted FS06 portable controls for common profile selection and complete
per-obligation loss accounting. TM-008 remains Planned; no executable standard
coverage is claimed.

## Test Procedure

Select each exact target profile over the same ordered obligation population.
Exercise preserved, conditional, unrepresented and refused cases. Independently
omit/duplicate/cross-wire each obligation record, disposition, dependency,
condition, cause and output region. Supply observation adequacy, protocol
adequacy, both, neither and a bare adequacy value.

## Expected Results

Exactly one record exists per requested obligation and no extra record exists.
Every classification satisfies its invariant; absence never defaults to
preserved. Both adequacy domains remain distinct and bare adequacy refuses.
