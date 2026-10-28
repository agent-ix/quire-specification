---
id: TC-302
title: "Temporal request members and obligation identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-368
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-363
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-366
    type: verifies
---

# TC-302: Temporal request members and obligation identity

## Description

The subject, item kind, fairness set and limits in the FR-331 request, and
their place in the request and obligation identities.

## Test Procedure

From the TC-293 step 2 request `Q`, derive requests that differ from `Q` in
exactly one of: the universe (`{a}` in place of `{a, b}`); the initial state
(`a` at version 1); the item kind (the derived deadlock-freedom item in place
of the claim); the fairness set (`each` in place of `whole`); the limits
(`max_automaton_states` 50). Also derive two bounded-search forms of `Q`
that differ only in `max_depth` (5 and 10). Run `Q` and settle its item, then present that
verdict to each derived request. Submit `Q` with `max_automaton_states`
omitted. Build a request of two authored items over two subjects and a refinement
item over the first, add the derived items, and record the `request_index`
order with and without a `terminal any` declaration on the first subject.
Derive from `Q` a request whose `limits` add `max_solver_steps`
1000 and `max_solver_time` 5, then two that differ from it only in
`max_solver_time` (5 and 10) and two that differ only in `max_solver_steps`
(1000 and 2000); settle a `max_solver_time` stop for one of them and offer
the result to the cache. Submit the request with `max_solver_time` 1.5, with
`max_solver_time` 0 and with `max_solver_steps` 0. Reuse a completed result
produced under `max_solver_time` 5 for the request that sets 10. Derive
requests whose `smt_encoding` is `{max_constants: 7}` and `{max_constants:
8}`, and submit `smt_encoding` empty, with an unknown member, with
`max_constants` 0 and with `max_constants` 1.5.

## Expected Results

`Q` carries `subject` (initial-state references, universes, proof bounds),
`item_kind` `formula`, `fairness` and `limits` (FR-368-AC-1, FR-331-AC-11).
Each derived request has a different request identity and, except the limits
and horizon changes, a different obligation identity; `Q`'s verdict settles
none of the derived items whose obligation identity differs (FR-363-AC-7,
FR-368-AC-1). The two bounded-search forms have different request
identities. `Q`'s recorded result is not reused for the limits-changed
request, nor the depth-5 result for the depth-10 request; each runs and
records its own result (FR-368-AC-1, FR-331-AC-11).
The deadlock-freedom item's obligation identity is the subject with item kind
`deadlock-freedom` (FR-366-AC-4). The request with `max_automaton_states`
omitted runs under the provider's published default, and its terminal record
names the effective value (FR-368-AC-7).
The request with the solver budgets is admitted. The two `max_solver_time`
forms have equal request identity and cache key bytes, the two
`max_solver_steps` forms different ones, and the `max_solver_time` stop is
stored by neither request. The 1.5 and the two 0 values are refused before
consumption (FR-368-AC-11; vectors `temporal-limits-solver-time-fractional`,
`temporal-limits-solver-time-zero` and `temporal-limits-solver-steps-zero`).
The reused record states `max_solver_time` 5. The two `smt_encoding` forms
have different request identities and cache keys and share no result; the
empty object, the unknown member, the 0 and the 1.5 are refused before
consumption (FR-368-AC-12; vectors `temporal-smt-encoding-empty`,
`temporal-smt-encoding-unknown-member`, `temporal-smt-encoding-zero` and
`temporal-smt-encoding-fractional`).

The caller's items keep their `request_index` 0 to 2 whether or not the
deadlock-freedom item of the first subject exists, and the derived items follow
in subject-then-kind order, each of kind `temporal-satisfaction` with no
`hybrid` and no `requirements` (FR-368-AC-17, FR-368-AC-18).
