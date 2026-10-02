---
id: SR-1183
title: "Base review of PR #178: effect-axis redefinition walk cost"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@ef8e9bc557b503ce4eab9a1f78b5449743a85d24; PR #178 diff against origin/main 58f25068, QSL-48 part (commit ef8e9bc): FR-151-AC-11, TC-196 R09, complete-v1/tests.md row; context value-accounting.md conformance.axis row, TC-196 fixture H, R02, R03"
review_set: subset
---
# Base review of PR #178 (QSL-48)

## Summary

Ticket: QSL-48. FR-151-AC-11 states the effect axis's work units, and TC-196
R09 adds a vector where a modified field reaches its redefined entry through
one `redefines` link (`r = 1`).

Worked by hand: in fixture H, `A/op`'s frame is `{modifies: [A/x], creates:
[A]}`. R09's `B/op` modifies `[B/w]`, and `B/w` redefines `A/x`. Its one write
is compared against `n = 1` redefined `modifies` entry and walks `r = 1` link,
so `t = 1 × (1 + 1) = 2`. There is no create or delete, so the effect axis
charges `max(1, 2) = 2`. Per axis: arity 1, parameter type 1 (`f(A) = 1`),
parameter multiplicity 1, result type 2 (`f(B) = 2`), result multiplicity 1,
effect 2, precondition 1, postcondition 1. That is ten work units over eight
axis charges, as R09 states. R03's effect charge is 1 (`n = 1`, `r = 0`). AC-11
matches the `conformance.axis` row of `value-accounting.md`, and the
complete-v1 row maps it to TC-196. `cargo test --locked` passes.

## Verdict

Clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
