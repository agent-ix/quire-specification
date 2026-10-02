---
id: FR-396
title: "Classify hyperproperty and step-relation forms"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-395
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-397
    type: references
  - target: ix://agent-ix/quire-specification/FR-398
    type: references
  - target: ix://agent-ix/quire-specification/FR-390
    type: references
  - target: ix://agent-ix/quire-specification/FR-391
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
---

# FR-396: Classify hyperproperty and step-relation forms

## Description

When a `hyper` clause over `behaviours` or a `relation` clause over model
operations is admitted, the checker SHALL classify it into exactly one form
below from its quantifier prefix, body, fairness sets and alignment, SHALL
record the form beside its requested item, and SHALL route the item by its
form.

## Inputs

The checked clause from [FR-395](FR-395-analyze-hyperproperties-over-behaviours.md)
or [FR-397](FR-397-check-step-relations-over-model-operations.md).

## Outputs

The clause's form and its requested item or items.

## Forms

The safety fragment is the safety fragment of the infinite-trace profile,
interval operators included.

| Form | Shape | Settled by |
| --- | --- | --- |
| HP-1 step relation | a `relation` clause whose execution bindings name model operations | FR-397 |
| HP-2 universal | `hyper` over `behaviours`, every quantifier `forall` (at least one), lockstep, any infinite-trace body, any fairness sets | exhaustive exploration; bounded refutation; induction for a safety body |
| HP-3 `∀∃` safety | prefix of `n >= 0` `forall` then `m >= 1` `exists`, `n + m >= 2`, lockstep, body in the safety fragment, every existential variable with the empty fairness set | exhaustive exploration |
| HP-4 other | an `exists` before a `forall`; an existential variable beside another variable whose body is outside the safety fragment or whose fairness set is non-empty; `align skip` with an existential variable or a body outside the safety fragment | none: `unsupported` |
| HP-5 single existential | exactly one quantifier, `exists`, any infinite-trace body, any fairness set | the possibility family (below) |
| HP-6 projection-aligned universal | as HP-2, with `align skip` and a body in the safety fragment | FR-398 |

The admitted fragment is HP-1, HP-2, HP-3, HP-5 and HP-6. When a clause is
HP-4, negotiation SHALL settle its item `unsupported`, warned,
`unsupported_projection`/`unsupported-requested-capability`, and no engine
runs for it.

## Routing

Each instance of a `hyper` clause, and the model claim of a `relation`
clause, is one requested item that requires the FR-290 kind
`temporal-satisfaction` with an unbounded extent naming the clause; its form
is recorded beside the item as an obligation form. A candidate's negotiation
arm settles the item `supported` only when it discharges that form. A
`relation` clause's code claim is a separate item of kind
`operation-contract` (FR-397).

## Single-existential claims

An HP-5 clause is a possibility claim and joins the state-graph family
([FR-390](FR-390-analyze-state-graph-claims.md)) as a form with a temporal
body.

- **Meaning.** It holds when, from every initial state of the subject, some
  fair behaviour satisfies the body.
- **Admission.** A `match` block on an HP-5 clause SHALL be refused as
  `unsupported_construct`/`expression-form`, located at the block, since the
  clause has one trace. Its fairness set is admitted: fairness decides which
  infinite behaviours count as a witness.
- **Verdicts.** The rows of [FR-391](FR-391-settle-state-graph-claims.md)
  apply, read over the product of the subject with the body's automaton and
  the clause's fairness, including FR-391's rule that well-definedness is
  global. **Witnessed:** every initial state has a witness, a fair lasso on
  which the body is true, each replays, and the exploration of the product
  completed with no open node and no undefined evaluation of the body:
  `proved`, `decisive-witness`, with each source `sampled`, `explored` or
  `unrolled`. **Witness found, well-definedness unchecked:** every initial
  state has a witness and the exploration ends with open nodes, or a limit
  stops it, before refuting evidence is found: `inconclusive`, cause
  `well-definedness-unchecked`, with the witnesses. **Trap:** an initial state
  whose part of the product is closed and holds no fair accepting cycle:
  `refuted`, `closed-scope`. **Undefined:** the body evaluates undefined at a
  reachable position: `refuted`, cause `undefined-evaluation`, with the
  `undefined` arm. Refuting evidence is ordered as FR-391 orders it.
  **Otherwise** the "No decisive evidence" row of FR-391. Witness sampling
  ([FR-392](FR-392-sample-possible-witnesses.md)) runs first: a sampled walk
  that revisits a state gives a candidate lasso. The verdict does not depend
  on `witness_samples`.
- **Evidence.** A witness is the `witness` arm of
  [FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md) with one
  model lasso per initial state and its `loop_entry`. A trap is the `trap` arm
  naming the initial state with an empty stem. Each replays by the
  single-existential rows of
  [FR-394](FR-394-replay-state-graph-evidence.md).
- **Basis.** An exhaustive refutation of a possibility claim carries no trace,
  because it shows that no trace exists. Its basis is `closed-scope`, since
  the trap's closure, the whole decision scope, was examined completely, and
  its settlement method is exhaustive exploration.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-396-AC-1 | Each clause of the classification vectors is assigned exactly its listed form, and the form is recorded beside its item as an obligation form. | Test (TC-342) |
| FR-396-AC-2 | An `exists` before a `forall`, a `∀∃` clause with a liveness body, a `∀∃` clause whose existential variable has a fairness constraint, and `align skip` with an existential variable each settle `unsupported` with `unsupported_projection`/`unsupported-requested-capability`, and no engine runs. | Test (TC-342) |
| FR-396-AC-3 | Every hyper item and every relation model claim requires `temporal-satisfaction`; a single candidate whose arm does not discharge the form settles `unsupported`. | Test (TC-342) |
| FR-396-AC-4 | An HP-5 claim with a fair lasso witness from every initial state and a completed product exploration with no undefined evaluation settles `proved`, `decisive-witness`, and the witness replays by FR-394; an HP-5 claim whose initial state's closed product part holds no fair accepting cycle settles `refuted`, `closed-scope`, and the trap replays by re-exploration. | Test (TC-342) |
| FR-396-AC-5 | A `match` block on an HP-5 clause refuses as `unsupported_construct`/`expression-form`; a fairness set on it is admitted. | Test (TC-342) |
| FR-396-AC-6 | An HP-5 claim with a witness for every initial state whose product exploration a limit stops before refuting evidence is found settles `inconclusive`, cause `well-definedness-unchecked`; one whose body evaluates undefined at a reachable position settles `refuted`, cause `undefined-evaluation`, also when a witness exists. | Test (TC-342) |

## Dependencies

- [FR-395](FR-395-analyze-hyperproperties-over-behaviours.md) and
  [FR-397](FR-397-check-step-relations-over-model-operations.md) supply the
  checked clauses.
- [FR-390](FR-390-analyze-state-graph-claims.md),
  [FR-391](FR-391-settle-state-graph-claims.md),
  [FR-392](FR-392-sample-possible-witnesses.md) and
  [FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md) settle
  HP-5.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) assigns the
  capability kinds.
- [FR-363](../temporal/FR-363-settle-every-behaviour-temporal-verdicts.md) and [FR-364](../temporal/FR-364-replay-temporal-counterexamples.md)
  own the safety fragment of the infinite-trace profile and model lassos.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
