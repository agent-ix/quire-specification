---
id: FR-488
title: "Attach a temporal guarantee to implementation Rust through checked correspondence"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-353
    type: references
  - target: ix://agent-ix/quire-specification/FR-350
    type: references
  - target: ix://agent-ix/quire-specification/FR-354
    type: references
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-380
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
---

# FR-488: Attach a temporal guarantee to implementation Rust through checked correspondence

## Description

When a temporal claim over a model subject is requested as a guarantee of the
implementation Rust bound to that subject, the verification planner SHALL
settle it `proved` only from the premises it carries by identity: the claim
proved over the model, a checked correspondence between the implementation and
the model and the environment premises, and, for a liveness or progress claim
only, a progress-preservation premise. A
state abstraction binding (FR-353) is one input of the correspondence and is
never by itself a temporal proof.

## Inputs

An implementation-guarantee item naming: an infinite-trace clause `P` with
fairness set `F_P`; the model subject; the implementation subject, the named
Rust functions and state of the FR-353 abstraction relation revision that binds
it; and the obligation identities of

- the **model claim**: `P` proved over the model subject, over its stutter
  closure when `P` is a liveness clause (FR-380 "Safety and liveness
  premises");
- the **correspondence** items below, one per obligation;
- the **progress** items below;
- the **environment premises**, each a typed assumption record with its
  supplier and trust basis (FR-354).

## Outputs

An FR-360 result for the item with its FR-243 basis, cause and the premises it
read, and, when `proved`, the conclusion record below.

## Behavior

**Correspondence.** The implementation corresponds to the model when each of
these is `proved` as its own item over the bound implementation:

1. *Initialization*: every implementation initial state maps through the
   abstraction relation to a model initial state.
2. *Transition simulation*: every step of every bound implementation function
   maps either to a model step of the operation the binding names, with the
   abstract pre and post states the relation gives, or to an **allowed
   stutter**, a step that leaves the abstract state unchanged.
3. *Frame*: no step changes abstract state outside the operation's frame
   (FR-013, FR-353-AC-1).

A step that violates the abstraction relation refutes the transition item with
a replayable counterexample for the emitted function (FR-350-AC-4) and the
guarantee item settles `inconclusive`, cause `premise-unproved`, naming the
transition premise. A valid per-step contract (FR-350) alone discharges none
of the progress items.

**Progress preservation.** Correspondence is a safety fact; the guarantee of a
liveness clause also requires, each as its own item:

1. *No unbounded divergence*: the allowed stutters cannot continue forever. The
   premise names a well-founded measure that every stutter step strictly
   decreases, or a bound on consecutive stutter steps; an implementation
   behaviour with infinitely many consecutive allowed-stutter steps
   (divergence) defeats the premise.
2. *Fairness correspondence*: every constraint of `F_P` is implied by the
   fairness the implementation actually has: each model operation named by a
   constraint is executed by the implementation under the scheduling premise
   the environment records, with the kind and granularity rule of FR-380
   ("Fairness implication"). A scheduling premise stronger than the
   environment supplies is not assumed.

A safety clause needs the model claim, initialization, transition simulation,
frame and the environment premises it reads; it needs no progress item.

**Settlement.**

| Premises and checks | FR-360 label | FR-243 basis | Cause |
| --- | --- | --- | --- |
| A liveness clause with every premise and every correspondence and progress item `proved` | `proved` | `decisive-witness` | none; the witness is the set of premise records |
| A safety clause with the model claim, initialization, transition simulation, frame and environment premises `proved` or supplied | `proved` | `decisive-witness` | none; no progress item is read |
| The model claim not `proved` | `inconclusive` | `unsettled` | `premise-unproved`, naming the model claim and its result |
| A correspondence item not `proved` (a violating step included) | `inconclusive` | `unsettled` | `premise-unproved`, naming the premise and its result |
| A progress item not `proved` (an infinite stutter behaviour, an absent measure, a fairness premise stronger than the environment's, or no progress item for a liveness clause) | `inconclusive` | `unsettled` | `premise-unproved`, naming the progress item and its result |
| A required environment premise the item does not list | `inconclusive` | `unsettled` | `premise-unproved`, naming the premise (the item is well formed; nothing discharges it) |
| A premise identity that names no settled result | -- | -- | refused `invalid_runtime_input`/`invalid-value`, naming the member (FR-380) |
| A model element with no bound abstraction relation | -- | -- | refused `missing_declaration`/`missing-name` (FR-353-AC-3), scoped to the item |
| Verus or the checking backend not registered | `unsupported` | `unavailable` | the missing capability (FR-350-AC-5) |
| Work or resource bound reached | `incomplete` | `unavailable` | the reached limit |

A guarantee item is never `refuted`: an implementation counterexample refutes a
correspondence premise, not the model claim. A finite implementation trace,
however long, is evidence for that trace only and never promotes a liveness
guarantee to `proved`. Missing backend, missing premise identity, missing binding and
exhausted work stay distinct outcomes, each with its own extent; a result that
only references another result is not evidence of its validity (FR-354).

**Conclusion record.** A `proved` item carries one source-bound record naming
the implementation/model relation (the FR-353 relation revision and the
implementation sources it binds), the temporal claim `P` with `F_P`, the result
of each premise, the environment assumptions used, the proof extent (the
bound of any bounded premise, or unbounded) and the trust basis
(FR-354 `certified`, `uncertified` or `trusted`) of each premise. Changing the
relation revision, an implementation source or an environment premise
invalidates the record.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-488-AC-1 | A liveness item whose model claim, initialization, transition simulation, frame, no-divergence and fairness-correspondence premises are all `proved` and whose environment premises are supplied settles `proved`, basis `decisive-witness`, with the conclusion record naming the relation revision, the claim and `F_P`, every premise result, the assumptions, the extent and the trust basis. | Test (TC-982) |
| FR-488-AC-2 | An implementation step that violates the abstraction relation refutes the transition premise with a replayable counterexample and the guarantee item settles `inconclusive`, cause `premise-unproved`, naming the transition premise. | Test (TC-982) |
| FR-488-AC-3 | A liveness item whose implementation has only the FR-350 per-step contracts proved and no progress item settles `inconclusive`, cause `premise-unproved`, naming the first progress item. | Test (TC-982) |
| FR-488-AC-4 | An implementation with an infinite run of allowed-stutter steps, one whose stutter steps decrease no declared measure, and one whose fairness premise is stronger than the environment's each settle `inconclusive`, cause `premise-unproved`, naming the progress item. | Test (TC-982) |
| FR-488-AC-5 | A safety item settles `proved` from the model claim, initialization, transition simulation, frame and environment premises with no progress item. | Test (TC-982) |
| FR-488-AC-6 | A required environment premise the item does not list settles `inconclusive`, cause `premise-unproved`, naming it, and an environment premise whose identity names no settled record is refused `invalid_runtime_input`/`invalid-value`; a model claim not `proved` settles `inconclusive`, cause `premise-unproved`, naming the model claim; a premise identity naming no settled result is refused `invalid_runtime_input`/`invalid-value`. | Test (TC-982) |
| FR-488-AC-7 | A model element with no bound abstraction relation refuses `missing_declaration`/`missing-name`, an unregistered backend settles `unsupported` naming the capability and an exhausted bound settles `incomplete`; the three are distinct and the sibling items with resolved bindings settle independently. | Test (TC-982) |
| FR-488-AC-8 | A finite implementation trace over which the clause holds does not settle a liveness item `proved`; a guarantee item never settles `refuted`. | Test (TC-982) |
| FR-488-AC-9 | Rebinding the model to a new relation revision, changing an implementation source or changing an environment premise invalidates the recorded conclusion, and the item is settled again from its premises. | Test (TC-982) |

## Dependencies

- [FR-353](FR-353-declare-abstraction-relation.md) and
  [FR-350](FR-350-emit-verus-contracts.md): the binding and per-step contract
  this requirement uses and does not replace; real Verus execution stays with
  the FR-350 backend.
- [FR-354](FR-354-record-proof-trust-basis.md): the trust basis of each premise.
- [FR-377](../analysis/FR-377-decide-model-refinement.md) and
  [FR-380](../analysis/FR-380-transfer-claims-through-refinement.md): the
  model-to-model refinement and the stuttering and fairness rules this
  requirement applies to an implementation.
- [FR-362](../temporal/FR-362-apply-fairness-constraints.md),
  [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
  and [FR-243](../../objects/protocol/FR-243-settlement-basis.md): fairness
  constraints, result labels and bases.

## References

- Tickets: Linear QSL-1541 (RES-74 comparison); QSL-36 abstraction bindings.
