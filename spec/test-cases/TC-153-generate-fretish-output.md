---
id: TC-153
title: "Generate bounded FRETish output"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-124
    type: verifies
---
## Description

Accepted FS06 portable FRETish profile controls.

## Test Procedure

Map one universal state predicate, immediate response, complete fixed-sample
`[0,b]` eventual response and exact `[1,1]` next-timepoint response. Mutate clock,
unit, period, trigger, capture and completeness independently. Exercise event-
position, timestamped, nonselected bounds, past, open/incomplete, protocol and
probabilistic cases plus a timing-free source obligation.

## Expected Results

Only selected Classic FRETish fields are emitted. Every temporal correspondence
condition remains in the mapping record. Unsupported cases emit no substitute
sentence; timing-free input gains no timing and FRET output is not FR-095 evidence.
