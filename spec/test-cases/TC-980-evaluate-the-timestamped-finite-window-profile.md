---
id: TC-980
title: "Evaluate the timestamped-event finite-window profile"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: verifies
---

# TC-980: Evaluate the timestamped-event finite-window profile

## Description

Verify FR-091-AC-10 to FR-091-AC-19 and FR-094-AC-11: an established activation
has a definite verdict, an unestablished one is definite only when every
admissible continuation agrees, and otherwise it is `pending` while declared
progress could resolve it and `indeterminate` once closed evidence cannot.

Catches false extension on a sparse window, an empty window left unknown, a
missing valuation read as false, an incomplete input read as closed, an instant
at the inclusive deadline dropped, a half-open watermark read as covering its end,
and a result that depends on ingestion order.

## Test Procedure

Apply the established-window rule of FR-091 and the possibility sets of FR-160 to
each vector of `proposals/finite-window/finite-window-vectors.json` (`FWV-01` to
`FWV-47`; QSL TC-1295 rows 1 to 22) and compare the outcome class, deciding
position, deadline and reason with the stated expectation. The tick-cover vectors
`TCV-01` to `TCV-15` are consumed from `tick-cover-vectors.json`.

## Expected Results

- Outcomes are compared as fields (class, position, deadline, reason), never
  message text.
- No vector marked `pending` or `indeterminate` returns a truth value, and none
  marked definite returns `pending` or `indeterminate`.
- Expected values come from the authored stamps and the rule, never from an
  evaluator's own output.
