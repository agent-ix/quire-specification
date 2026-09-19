---
id: TC-238
title: "Admit the unbounded temporal operator grammar only under infinite-trace"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-048
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-090
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
`[a,b]` interval and with the Complete-V1 `[a,*]` form under all four
profiles. Independently mutate the selected profile identity on one accepted
infinite-trace source while keeping its bytes otherwise fixed.

## Expected Results

The bare interval-less spelling is admitted only under
`quire.temporal.infinite-trace/v1` (FR-048-AC-9, FR-090-AC-7); under any of the
other three profiles it receives `unsupported_construct` naming its exact
construct span, owning declaration and selected profile, as AD-003's required
diagnostic dispositions state for a recognized form prohibited by the selected
profile. The bracketed `[a,b]` form parses identically under all four profiles,
unchanged from its pre-edition behavior. The Complete-V1 `[a,*]` form keeps the
admission it already had under each profile, distinct from and unaffected by
the new bare production. Mutating only the selected profile identity on the
accepted infinite-trace source moves it to the same `unsupported_construct`
disposition, without a partial or approximated parse.
