---
id: TC-115
title: "Settle inclusive deadlines from valid progress"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-094
    type: verifies
---
## Description

Planned progress controls for timestamped, fixed-sample and event-position
obligations, including a silent refund deadline.

## Test Procedure

For the same active obligation, supply a matching response exactly at deadline,
one tick after deadline and no response. Advance a validated timestamp watermark
and fixed-sample progress to just before, exactly at and just after the inclusive
deadline without adding another business event. Repeat using event-position
time. Substitute progress from a foreign clock, source set, subject or binding,
and mutate the inclusive comparison to strict greater-than.

## Expected Results

The at-deadline response participates and the late response does not. Complete
timestamp/sample progress through the deadline settles silence; event-position
time remains open without a new position or closure. Foreign progress cannot
settle the obligation, and the endpoint mutation is detected.
