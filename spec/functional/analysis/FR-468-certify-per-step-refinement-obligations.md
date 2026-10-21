---
id: FR-468
title: "Query and certify a refinement's per-step simulation obligations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-379
    type: references
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
---

# FR-468: Query and certify a refinement's per-step simulation obligations

## Description

When a refinement declaration has no hidden field, no history field and no `any` row, the checker SHALL state its per-step simulation proof as one *initial query* for the initial-state rule and one *step query* for each step row, each a refutation query over the declared types; the refinement settles `Certified` only when every one of these queries carries a verified refutation certificate.

## Inputs

An admitted refinement declaration ([FR-375](FR-375-declare-model-refinement.md)) whose concrete subject has no hidden field, history field or `any` row, the concrete subject's invariant clauses, and a proof backend's outcome and refutation certificates for the queries.

## Outputs

For each query, the query itself and a settlement; for the refinement, the safety half settlement of [FR-379](FR-379-report-and-replay-refinement-counterexamples.md) with its certification level.

## Behavior

### The queries

The per-step obligations of [FR-377](FR-377-decide-model-refinement.md) (Engines) are the queries' content; each query is the negation of one obligation, so a refutation of the query proves the obligation:

- **Initial query.** The negation of: every concrete initial state maps to an abstract initial state. It has no hypotheses beyond the declared types.
- **Step query**, one per step row, identified by the row's position in the declaration node's `steps` order ([FR-375](FR-375-declare-model-refinement.md)). The negation of: for every pair `(s, s')` of states of the declared types, if the concrete invariants hold at `s`, the operation's effective precondition holds at `s`, `(s, s')` is within the operation's frame and its postcondition holds over `(s, s')`, then the row's rule (`stutter`: the mapped state is unchanged by state key; an abstract operation: that transition's precondition, frame and postcondition admit the mapped pair) holds over `(map(s), map(s'))`.

Each query is one `operation-contract` item ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)) and is settled on its own. No query reads another query's result: the step queries assume the concrete invariants as hypotheses, and the initial query is what establishes that the initial states satisfy them, which is its own item for each invariant clause.

### Certificate

The certificate of the refinement's safety half carries one *base* part, the refutation of the initial query, and one *step* part per step row, in the `steps` order, each the refutation of that row's step query. A part is a solver refutation in a checkable proof format, bound to the query it refutes. A certificate lacking its base part, or lacking the step part of any row, is not an inductive certificate of the refinement; a step part is never accepted in place of the base part.

### Settlement

- Every query settled `proved` with a verified certificate: the safety half settles `proved` with certification `Certified`.
- Every query settled `proved`, at least one with no certificate or an unverifiable one: the safety half settles `proved` with certification `Uncertified`.
- A query whose backend returns a counterexample, or that is not closed from a pre-state satisfying the hypotheses: the safety half settles `inconclusive`, cause induction not closed, and never `refuted`, because the counterexample's pre-state may be unreachable ([FR-379](FR-379-report-and-replay-refinement-counterexamples.md)).
- A certificate that fails verification settles `inconclusive` with the rejected part and proof step named.

A declaration with a hidden field, a history field or an `any` row has no per-step queries and no inductive certificate; its safety half is decided by the explicit-state engine only.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-468-AC-1 | A refinement of a model with seven step rows and no hidden field, history field or `any` row has one initial query and seven step queries, each its own `operation-contract` item, the step queries in `steps` order; changing one row to `any` leaves no query. | Test (TC-950) |
| FR-468-AC-2 | A certificate with the base part and a step part for every row is complete; one without the base part, or without any row's step part, is refused as not an inductive certificate, and a step part supplied in the place of the base part is refused. | Test (TC-950) |
| FR-468-AC-3 | With every query proved and every part verified the safety half settles `proved`, `Certified`; with one query's certificate absent or unverifiable it settles `proved`, `Uncertified`. | Test (TC-950) |
| FR-468-AC-4 | A backend counterexample for one step query settles the safety half `inconclusive`, induction not closed, naming the row, and never `refuted`; a certificate whose step part fails verification settles `inconclusive` naming the part. | Test (TC-950) |
| FR-468-AC-5 | A concrete invariant clause adds itself to every step query as a hypothesis and has its own initial-state item; the initial query's identity differs with and without the clause. | Test (TC-950) |

## Dependencies

- [FR-377](FR-377-decide-model-refinement.md) owns the obligations the queries negate.
- [FR-379](FR-379-report-and-replay-refinement-counterexamples.md) owns the verdicts and the rule that a per-step failure is never `refuted`.
- [FR-375](FR-375-declare-model-refinement.md) owns the `steps` order the certificate follows.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the `operation-contract` kind.
