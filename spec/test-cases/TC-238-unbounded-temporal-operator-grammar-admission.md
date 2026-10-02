---
id: TC-238
title: "Admit the unbounded temporal operator grammar only under infinite-trace"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-048
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-367
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-250
    type: verifies
---

# TC-238: Admit the unbounded temporal operator grammar only under infinite-trace

## Description

Grammar-admission mutation controls for the interval-less
`temporal-unary-unbounded`/`temporal-relation-unbounded` edition
([AD-012](../assurance/AD-012-unbounded-temporal-operator-grammar.md)). Confirms
the bare spelling is admitted only under `quire.temporal.infinite-trace/v1` and
that every other v1 profile receives `unsupported_construct` for it.

## Test Procedure

For each of `always`, `eventually`, `once`, `historically`, `until`, `release`,
`since` and `triggered`, author the bare interval-less spelling (no `[a,b]` or
`[a,*]`) with a `holds(...)` operand, once under a clause selecting
`quire.temporal.infinite-trace/v1` and once under each of
`quire.temporal.event-position.false-extension/v1`,
`quire.temporal.fixed-sample.false-extension/v1` and
`quire.temporal.timestamped-event.finite-window/v1`. Repeat with a bracketed
`[a,b]` interval and with an `[a,*]` spelling under all four profiles, and
author `always (holds(fail) implies eventually always[0,10] holds(healthy))`
and `once[1,1] holds(p) since holds(q)` under all four profiles. Independently mutate the selected profile identity on one accepted
infinite-trace source while keeping its bytes otherwise fixed.

## Expected Results

The bare interval-less spelling is admitted only under
`quire.temporal.infinite-trace/v1` (FR-048-AC-9, FR-090-AC-7); under any of the
other three profiles it receives `unsupported_construct` naming its exact
construct span, owning declaration and selected profile, as AD-003's required
diagnostic dispositions state for a recognized form prohibited by the selected
profile. The bracketed `[a,b]` form parses identically under all four profiles,
unchanged from its pre-edition behavior. `[a,*]` parses as a lower-bounded
operator under `quire.temporal.infinite-trace/v1` and receives
`unsupported_construct` under each of the other three profiles (FR-048-AC-10,
FR-367-AC-7). The two nested formulas are admitted under
`quire.temporal.infinite-trace/v1` with their interval and unbounded operators
nested as written, and under each bounded profile their interval-less
operators receive `unsupported_construct` (FR-090-AC-7, FR-367-AC-1, FR-250-AC-6). Mutating only the selected profile identity on the
accepted infinite-trace source moves it to the same `unsupported_construct`
disposition, without a partial or approximated parse.
