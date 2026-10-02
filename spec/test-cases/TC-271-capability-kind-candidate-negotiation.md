---
id: TC-271
title: "Capability kinds, advertised modes and candidate-set negotiation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
---

# TC-271: Capability kinds, advertised modes and candidate-set negotiation

## Description

Admit every FR-290 capability kind, refuse every other label, assign each
claim form its kind, and settle requested items from their FR-331 candidate
sets and extents through `negotiate_*`, including tool absence after routing.

## Test Procedure

Hold the ten FR-290 labels as a test constant. Build registries from fixture
backend descriptors, each with a unique identity, advertising a set of
(kind, mode) pairs. Build
FR-331 envelopes with `capability_vocabulary` `quire.capability-kind/v1` whose
request items carry one required kind, an extent with its #222 classification,
an optional named backend and the registry-computed `candidates`. Execute each
vector through the real local Rust boundary; compare the typed disposition,
FR-331 result, accounting record and catalog code/cause with the criterion.
Repeat every registry vector with the descriptors registered in reversed order.

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| CK-01 | Request each of the ten FR-290 labels | Each admits as exactly that member |
| CK-02 | Request no kind, `null`, `""`, `Global-Conformance`, `value validity`, each of `FamilyCheck`, `StateOperation`, `FiniteReplay`, `TemporalProjection`, and a label outside the table | `invalid_capability` with `absent-kind` for the first two and `unknown-kind` for the rest, each retaining the received bytes and the item's request index; no label is mapped to a member |
| CK-03 | Set the FR-331 `capability_vocabulary` to `quire.capability-kind/v2`, to `quire.foreign-kind/v1`, and remove it | `invalid_capability`/`unsupported-version` for the whole envelope; no kind in it is read |
| CK-04 | Register a descriptor advertising the mode `finite`; another advertising the kind `smt-proof`; another with no kind | Each registration refuses as `invalid_capability` with `unknown-mode`, `unknown-kind` and `absent-kind` respectively, each retaining the refused backend identity; other registrations stand. TC-282 covers a repeated identity |
| CK-05 | Derive the required kind of every row of the FR-290 claim-form table: a function-application clause, a clause with a nested `case` expression and function application, an operation precondition, postcondition and invariant, a frame obligation, a contract refinement gate, a finite replay, a finite-trace and an infinite-trace temporal requirement, a protocol refinement, and a claim of each of `global-conformance`, `monitorability`, `local-projection`, `realizability` and `composition` | `value-validity` (twice, one kind each), `operation-contract` (five times), `finite-replay`, `temporal-satisfaction` (twice), `refinement`, then each of the five protocol members |
| CK-06 | Check a non-exhaustive `case` over a union | Refuses as `undefined_expression`/`unproved-exhaustiveness` during admission; no candidate set is computed and no backend is requested |
| CK-07 | Check an exhaustive `case` over a union, and an operation bound by an FR-353 abstraction relation | The exhaustiveness obligation and the abstraction relation request no kind and no backend; the operation's clauses request `operation-contract` only |
| CK-08 | Request a `value-validity` item and a `global-conformance` item together; settle the first `unsupported` and the second `supported`; then attach the complete-global-conformance premise set to a `local-projection` claim as its success evidence | Each item keeps its own disposition and outcome with no promotion or premise transfer; the `local-projection` claim refuses the transferred premise set |
| NG-01 | Request `temporal-satisfaction` from a registry with no registrant advertising it | `unsupported` with a warning naming `temporal-satisfaction`; `unsupported_projection`/`unsupported-requested-capability`; no artifact |
| NG-02 | Registrant A advertises (`value-validity`, `bounded`) and B advertises (`value-validity`, `unbounded`); the request names none | `invalid-request` naming both candidates in candidate order, as `invalid_capability`/`ambiguous-backend`, under both registration orders |
| NG-03 | The same registry; the request names one of the two | Candidates hold exactly that backend; its negotiation arm settles the item |
| NG-04 | The request names an unregistered backend; separately, a registered backend with no negotiation arm | `invalid-request` as `invalid_capability`/`unknown-backend`, naming it |
| NG-05 | Unbounded extent; the one candidate advertises (`value-validity`, `bounded`) only; a finite bound is available | `requires-bound`; no artifact |
| NG-06 | NG-05 with no finite bound available | `unsupported` warned, `unsupported_projection`/`unbounded-extent` naming the kind, candidate and advertised modes |
| NG-07 | Unbounded extent; the one candidate advertises (`value-validity`, `unbounded`) | The candidate's negotiation arm settles the item; never `requires-bound` |
| NG-08 | Remove `candidates` from one FR-331 request item | The request refuses before consumption |
| NG-09 | Two requests identical except for one item's `candidates` | Different request identities; candidates serialize ordered by identity under both registration orders |
| NG-10 | Bounded extent; the one candidate advertises (`value-validity`, `bounded`) | The candidate's negotiation arm settles the item |
| NG-11 | Bounded extent; the one candidate advertises (`value-validity`, `unbounded`) only | The candidate's negotiation arm settles the item |
| NG-12 | The request names a registered backend that does not advertise the item's kind, while another registrant does | Candidates are empty; `unsupported` with a warning naming the kind and the named backend; the other registrant is not run |
| NG-13 | Add to `candidates` a backend absent from the manifest; separately, one that does not advertise the kind; separately, a second backend beside the named one | `invalid-request` as `invalid_capability`/`inconsistent-candidates`, naming each offending candidate |
| NG-14 | Remove the extent classification from one request item | That item settles `invalid-request` as `invalid_capability`/`absent-extent`; the other items settle independently |
| NG-15 | The one candidate's negotiation arm does not discharge the item's obligation form | `unsupported` warned, `unsupported_projection`/`unsupported-requested-capability` naming the obligation form; no artifact |
| TA-01 | Registry of two registrants of `value-validity`; name one, settle the item `supported` and route it; remove its tool from `PATH` | FR-331 result `unsupported` naming the kind and backend; `unsupported_projection`/`tool-unavailable`; disposition stays `supported`; one terminal accounting record; artifacts emitted before the probe are retained with no verdict; the other registrant never runs |
| TA-03 | TA-01 with the probe erroring, and separately exceeding its limit | As TA-01 |
| TA-04 | The routed backend advertises both modes for the kind; remove its tool | As TA-01; no run under the other mode |
| TA-06 | TA-01's routed item passes the probe; remove the tool before the run completes | FR-331 result `failed` with `unsupported_projection`/`tool-unavailable` naming the kind and backend; disposition stays `supported`; one terminal accounting record; no other candidate or mode runs |

## Expected Results

Every positive result matches FR-290 and FR-331. Each refused label, mode,
vocabulary, registration or backend returns its exact `invalid_capability`
cause and is never mapped to an FR-290 member. Every item has exactly one
disposition and one terminal accounting record. Registration order never
changes a candidate set, a disposition or request bytes. No `unsupported`,
`requires-bound` or `invalid-request` disposition emits an artifact. Tool
absence is never a hold, a refusal of the language form, a fallback to another
candidate or a downgrade to another mode.
