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
omitted.

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
