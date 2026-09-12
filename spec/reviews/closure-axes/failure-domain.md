---
id: SR-512
title: "Failure-domain review of the scope progress and closure vocabulary"
type: SpecReview
analysis: failure-domain
scope: "Branch objects/closure-axes-vocabulary at 7a050c9: what a conforming implementation can still get wrong once the four axes are enumerated"
review_set: subset
evaluated_revision: "7a050c9"
review_date: "2026-09-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-284, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-258, type: reviews }
---
## Summary

Enumerating the four axes removes one failure mode — an implementation inventing
labels — and exposes four that were previously hidden behind the gap. Each is
either closed by an acceptance criterion here or recorded below as open.

## Mutability

The state on each of the four axes is mutable: it is observed about an identified
scope and changes as progress advances or closure is stamped. FR-258 already holds
those states outside the scope key for exactly this reason, and that arrangement
is unchanged. FR-284's members are immutable label values; nothing in this change
makes a mutable fact part of an identity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A single shared enum across all five axes passes the member check and silently admits cross-axis substitution. | FR-284, FR-247 |
| FND-002 | medium | `Option<State>` with a `None` arm that renders as `open` reintroduces the defaulting FR-061 refuses. | FR-061, FR-284 |
| FND-003 | medium | A progress axis derived from its closure axis satisfies every per-axis check while destroying independence. | FR-061, FR-094 |
| FND-004 | low | Input completeness remains unenumerated, so a five-axis product state is formable while a completeness-bearing result is not fully typed. | FR-258, FR-094 |
| FND-005 | low | A regression that rolls a progress axis from `closed` back to `open` is a contradiction, not a state change, and no criterion here tests it. | FR-094-AC-8 |

### FND-001 — one enum, five axes

The most likely implementation shortcut. Four axes take two members, the fifth
takes five; a single enum of five members typechecks everywhere and lets
`not-required` reach a decision-scope closure field, which FR-061 refuses
outright. FR-284-AC-3 is written to catch it: the three extra members must be
accepted on the fifth axis and refused on each of the four. An implementation
that shares one type must enforce the restriction at the field boundary instead.

### FND-002 — absence rendered as `open`

FR-061 now says an absent axis "compares only to absence, is never defaulted to
`open` or to `closed`, and remains the omitted required axis a strict reader
refuses". The failure is not in the data model — `Option` is the right shape — but
in the read path: a `None` that reaches a comparison, a projection or a report as
`open` has defaulted. FR-061-AC-12 and FR-284-AC-6 both test refusal rather than
substitution. Note that this is the same trap FR-247 guards for the fifth axis
("never defaulted to `open` or to `not-required`"), now stated for four more
fields.

### FND-003 — a derived progress axis

An implementation that computes progress from closure (or the reverse) passes
FR-284-AC-2 on every axis, since every value is a legal member, and passes AC-4
and AC-5 only if the tests vary the axes independently rather than checking a
single consistent state. FR-061-AC-13 is the criterion that bites: a `closed`
progress fact must leave its own scope's closure fact *independently readable*.
The strongest check is B's own `FR-028-AC-1`, which requires each axis to be
varied independently — now executable for the first time.

### FND-004 — the fifth product dimension

With four axes at two members and the global axis at five, the product state is
80 combinations, all formable. Input completeness is a further independent axis
under FR-094-AC-3 and is not enumerated, so a result that carries a completeness
fact is still not fully typed. That gap is retained on FR-258 rather than closed
by extension, because FR-094-AC-3 holds completeness distinct from the two
scopes in the same sentence that names them.

### FND-005 — regression is not a transition

FR-094 requires progress to be monotonic within its binding, and FR-094-AC-8
requires a same-binding regression to produce "a typed contradiction/refusal"
that "cannot roll back progress". Under the new vocabulary that means a progress
axis observed at `closed` and then asserted `open` for the same binding is a
contradiction, not a state change. FR-284 states monotonicity in its causal-order
bullet but declares no criterion for the regression case, because FR-094-AC-8
already owns it. An implementation must route that case through FR-094, not
through a state assignment.
