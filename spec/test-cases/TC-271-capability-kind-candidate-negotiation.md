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
backend descriptors, each advertising a set of (kind, mode) pairs and a pinned
tool identity. Build FR-331 requests whose items carry required kinds, an
extent, an optional named backend and the registry-computed `candidates`.
Execute each vector through the real local Rust boundary; compare the typed
disposition, FR-331 result, accounting record and catalog code/cause with the
criterion. Repeat every registry vector with the descriptors registered in
reversed order.

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| CK-01 | Request each of the ten FR-290 labels | Each admits as exactly that member |
| CK-02 | Request no kind, `null`, `""`, `Global-Conformance`, `value validity` and a label outside the table | `invalid_capability` with `absent-kind` for the first two and `unknown-kind` for the rest; no label is mapped to a member |
| CK-03 | Request a kind under a vocabulary version other than v1, and with no version | `invalid_capability`/`unsupported-version`; no label of that carrier is read |
| CK-04 | Register a descriptor advertising the mode `finite` | Registration refuses as `invalid_capability`/`unknown-mode` |
| CK-05 | Derive the required kind of a function-application clause, a clause with a `case` expression, an operation postcondition, a frame obligation, a finite replay, a temporal requirement and a `global-conformance` claim | `value-validity`, `value-validity`, `operation-contract`, `operation-contract`, `finite-replay`, `temporal-satisfaction`, `global-conformance` respectively |
| CK-06 | Check a non-exhaustive `case` over a union | Refuses as `undefined_expression`/`unproved-exhaustiveness` during admission; no candidate set is computed and no backend is requested |
| NG-01 | Request `temporal-satisfaction` from a registry with no registrant advertising it | `unsupported` with a warning naming `temporal-satisfaction`; `unsupported_projection`/`unsupported-requested-capability`; no artifact |
| NG-02 | Two registrants advertise `value-validity`; the request names none | `invalid-request` naming both candidates in ascending identity order, as `invalid_capability`/`ambiguous-backend`, under both registration orders |
| NG-03 | The same registry; the request names one of the two | Candidates hold exactly that backend; its negotiation arm settles the item |
| NG-04 | The request names an unregistered backend | `invalid-request` as `invalid_capability`/`unknown-backend`, naming it |
| NG-05 | Unbounded extent; the one candidate advertises (`value-validity`, `bounded`) only; a finite bound is available | `requires-bound`; no artifact |
| NG-06 | NG-05 with no finite bound available | `unsupported` warned, `unsupported_projection`/`unbounded-extent` naming the kind, candidate and advertised modes |
| NG-07 | Unbounded extent; the one candidate advertises (`value-validity`, `unbounded`) | The candidate's negotiation arm settles the item; never `requires-bound` |
| NG-08 | Remove `candidates` from one FR-331 request item | The request refuses before consumption |
| NG-09 | Two requests identical except for one item's `candidates` | Different request identities; candidates serialize in ascending identity order under both registration orders |
| TA-01 | Settle an item `supported` and route it; remove the backend's tool from `PATH` | FR-331 result `unsupported` with a warning naming the kind, backend and expected tool identity; `unsupported_projection`/`tool-unavailable`; disposition stays `supported`; one terminal accounting record; no other candidate runs |
| TA-02 | TA-01 with the tool present at a different pinned identity | As TA-01, retaining the actual tool identity |
| TA-03 | Instrument the adapter probe | The probe runs after routing and before the run, never before routing |

## Expected Results

Every positive result matches FR-290 and FR-331. Each refused label, mode,
version or backend returns its exact `invalid_capability` cause and is never
mapped to an FR-290 member. Every item has exactly one disposition and one
terminal accounting record. Registration order never changes a candidate set, a
disposition or request bytes. No `unsupported`, `requires-bound` or
`invalid-request` item emits an artifact. Tool absence is never a hold, a
refusal of the language form, a fallback to another candidate or a downgrade to
another mode.
