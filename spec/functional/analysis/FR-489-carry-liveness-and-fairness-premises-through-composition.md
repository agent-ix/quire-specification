---
id: FR-489
title: "Carry liveness dependencies and fairness premises through component composition"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-348
    type: references
  - target: ix://agent-ix/quire-specification/FR-176
    type: references
  - target: ix://agent-ix/quire-specification/FR-177
    type: references
  - target: ix://agent-ix/quire-specification/FR-380
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-354
    type: references
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
---

# FR-489: Carry liveness dependencies and fairness premises through component composition

## Description

When a system claim is composed from claims over component packages, the
verification planner SHALL carry in the composition request, and record in its
result, each safety rely-guarantee, liveness dependency, fairness premise and
environmental assumption as its own typed premise, and SHALL discharge each only
by a proved guarantee that names the same component subject and implies the
premise.

## Inputs

A composition request naming: the component subjects (checked package and
subject identity each); the selected composition rule (FR-348 negotiation, FR-176 assume-guarantee, FR-177 protocol refinement, FR-380 refinement transfer); and four
premise lists, none collapsed into a flag:

- `environment`: the environmental assumptions, each with its supplier;
- `relies`: for each component, the peer safety guarantees it relies on, each
  naming the peer subject and guarantee;
- `liveness_dependencies`: for each component liveness guarantee, the peer
  liveness guarantees whose progress it depends on, each naming the peer
  subject and guarantee;
- `fairness`: the fairness constraints (FR-362) each component guarantee holds
  under, and the fairness the composition supplies.

And the obligation identity of each supplied component guarantee.

## Outputs

An FR-360 result for the composed claim and a **premise discharge record**: for
every required premise, the identity of the proved guarantee that discharges it,
the subject it names, the relation by which it implies the premise, its trust
basis (FR-354) and extent.

## Behavior

**Premises are typed and separate.** A safety rely is discharged by a safety
guarantee, a liveness dependency by a liveness guarantee over the peer subject's
stutter closure where FR-380 requires it, a fairness premise by a supplied
fairness constraint under FR-380's kind and granularity rule and an
environmental assumption by a supplied assumption record. A guarantee of one
kind never discharges a premise of another.

**Discharge.** A guarantee discharges a premise only when its subject equals the
premise's subject (the same checked package and subject identity) and the
guarantee implies the premise. Two input shapes are distinguished:

- a supplied component guarantee that is over another subject, weaker than the
  premise, of another kind, or a same-named guarantee from another package
  does not discharge the premise; the premise stays unmet and the composition
  settles `inconclusive`, cause `premise-unproved`, naming the premise, unless
  another supplied guarantee discharges it;
- a request that names a discharge record (a premise paired with the
  guarantee that is claimed to discharge it) whose guarantee subject differs
  from the premise's subject, or whose identity names no settled result, is a
  malformed request, refused `invalid_runtime_input`/`invalid-value` naming the
  member (FR-380).

A result that merely references another result is not evidence of the
premise's validity (FR-354); the referenced result is read.

**Cycles.** Safety relies follow FR-176's discharge fixed point. Liveness
dependencies follow the same rule unchanged: a cycle among them, a mutually
dependent progress pair, is accepted only under FR-176's explicit sound
fixed-point rule (a monotone complete lattice, initial element and convergence
rule declared by the selected profile), with every iteration recorded
(FR-176-AC-4). Otherwise it is refused as circular reasoning exactly as
FR-176-AC-3 refuses it: no guarantee in the cycle is available as an
assumption and no discharge is granted. This requirement defines no other
route for a cycle.

**Settlement.**

| Premises and checks | FR-360 label | FR-243 basis | Cause |
| --- | --- | --- | --- |
| Every premise discharged under a supported rule | `proved` | `decisive-witness` | none; the witness is the premise discharge record |
| A premise discharged only by a trusted assumption | `proved` with FR-354 certification `trusted` | `decisive-witness` | none; the discharge record names the assumption |
| A premise no supplied guarantee discharges (absent, weaker, of another kind or over another subject) | `inconclusive` | `unsettled` | `premise-unproved`, naming the premise |
| A failed component check | `inconclusive` | `unsettled` | `premise-unproved`, naming the component's premise |
| An unsupported composition rule | `unsupported` | `unavailable` | the missing capability (FR-348-AC-4) |
| A liveness cycle outside FR-176's fixed-point rule | -- | -- | refused as FR-176-AC-3 refuses circular reasoning |
| A discharge record whose subject differs, or whose identity names no settled result | -- | -- | refused `invalid_runtime_input`/`invalid-value`, naming the member |
| Work or resource bound reached | `incomplete` | `unavailable` | the reached limit |

Unmet premise, trusted assumption, unsupported rule, failed check and
exhausted resources keep their distinct outcomes, trust basis and extent.
Unrelated supported claims in the request continue.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-489-AC-1 | A composition request names the component subjects, the composition rule, the environmental assumptions, the relied-on peer guarantees, the liveness dependencies and the fairness premises as separate lists, and a request that supplies them as one success flag is refused `invalid_runtime_input`/`invalid-value`. | Test (TC-983) |
| FR-489-AC-2 | The premise discharge record names, for each required premise, the proved guarantee that discharges it; a supplied guarantee over another subject, a weaker guarantee and a guarantee of another kind each leave the premise undischarged and the composition `inconclusive`, cause `premise-unproved`, naming the premise. | Test (TC-983) |
| FR-489-AC-3 | A two-component fixture whose safety relies and liveness dependencies are all discharged under a supported rule settles `proved`, `decisive-witness`, with the discharge record. | Test (TC-983) |
| FR-489-AC-4 | Two components each depending on the other's progress, under a rule with no declared fixed-point lattice, are refused as FR-176-AC-3 refuses circular reasoning and no guarantee of the pair is available as an assumption. | Test (TC-983) |
| FR-489-AC-5 | The same pair under a profile that declares FR-176's monotone lattice, initial element and convergence rule is accepted with every iteration recorded in the discharge record (FR-176-AC-4). | Test (TC-983) |
| FR-489-AC-6 | A premise discharged only by a trusted assumption settles `proved` with certification `trusted` and names the assumption; an unsupported rule settles `unsupported` naming the capability; a failed component check settles `inconclusive`/`premise-unproved`; an exhausted bound settles `incomplete`; the four are distinct with their own trust basis and extent. | Test (TC-983) |
| FR-489-AC-7 | A discharge record whose guarantee subject differs from its premise's subject, and one whose identity names no settled result, are refused `invalid_runtime_input`/`invalid-value` naming the member; a referenced result that is itself `inconclusive` does not discharge the premise; sibling claims of the same request settle independently. | Test (TC-983) |

## Dependencies

- [FR-348](FR-348-admit-compositional-verification.md): composition negotiation;
  this requirement adds only the typed premise transport and its discharge record.
- [FR-176](../protocol/FR-176-compose-assume-guarantee-contracts.md) and
  [FR-177](../protocol/FR-177-check-protocol-refinement.md):
  the assume-guarantee algorithm and fixed-point rule (FR-176) and protocol refinement (FR-177), .
- [FR-380](FR-380-transfer-claims-through-refinement.md): the refinement
  transfer, stutter-closure and fairness rules.
- [FR-362](../temporal/FR-362-apply-fairness-constraints.md),
  [FR-354](../ir-proof/FR-354-record-proof-trust-basis.md),
  [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md) and
  [FR-243](../../objects/protocol/FR-243-settlement-basis.md).

## References

- Tickets: Linear QSL-1542 (TL-9, RES-74, RES-35).
