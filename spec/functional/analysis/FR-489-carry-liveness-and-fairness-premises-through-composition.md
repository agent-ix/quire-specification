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
guarantee implies the premise: a weaker guarantee, a guarantee over another
subject or a same-named guarantee from another package does not discharge it. A
result that merely references another result is not evidence of the premise's
validity (FR-354); the referenced result is read.

**Cycles.** Safety relies follow FR-176's discharge fixed point. A set of
liveness dependencies that depend on each other, a mutually dependent progress
pair, is not self-justifying: no guarantee in the cycle is available as an
assumption before the rule establishes it. A supported cycle is accepted only
when the selected rule declares independently justified premises (a
well-founded measure over the dependency or a declared fixed-point rule with
its convergence premise, FR-176) and each of them is a recorded premise with its
own discharge.

**Settlement.**

| Premises and checks | FR-360 label | FR-243 basis | Cause |
| --- | --- | --- | --- |
| Every premise discharged under a supported rule | `proved` | `decisive-witness` | none; the witness is the premise discharge record |
| A required premise with no discharging guarantee | `inconclusive` | `unsettled` | `premise-missing`, naming the premise and component |
| A discharging guarantee that is weaker, of another kind or over another subject | `inconclusive` | `unsettled` | `premise-mismatch`, naming the premise and the supplied guarantee |
| A premise supplied as a trusted assumption | per rule | `unsettled` until the assumption is accepted | the result keeps `trusted` and names the assumption |
| An unsupported composition rule | `unsupported` | `unavailable` | the missing capability (FR-348-AC-4) |
| A failed component check | `inconclusive` | `unsettled` | `premise-unproved`, naming the component |
| An unjustified liveness cycle | `inconclusive` | `unsettled` | `circular-reasoning`, naming the cycle |
| Work or resource bound reached | `incomplete` | `unavailable` | the reached limit |

Missing premise, trusted assumption, unsupported rule, failed check and
exhausted resources keep their distinct outcomes, trust basis and extent. A
forged or mismatched premise (an identity that names no settled result, or a
subject that differs from the premise's) is refused
`invalid_runtime_input`/`invalid-value`, naming the member; unrelated supported
claims in the request continue.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-489-AC-1 | A composition request names the component subjects, the composition rule, the environmental assumptions, the relied-on peer guarantees, the liveness dependencies and the fairness premises as separate lists, and a request that supplies them as one success flag is refused `invalid_runtime_input`/`invalid-value`. | Test (TC-983) |
| FR-489-AC-2 | The premise discharge record names, for each required premise, the proved guarantee that discharges it; a guarantee over another subject, a weaker guarantee and a guarantee of another kind each settle `inconclusive`, cause `premise-mismatch`, and do not discharge the premise. | Test (TC-983) |
| FR-489-AC-3 | A two-component fixture whose safety relies and liveness dependencies are all discharged under a supported rule settles `proved`, `decisive-witness`, with the discharge record. | Test (TC-983) |
| FR-489-AC-4 | Two components each depending on the other's progress settle `inconclusive`, cause `circular-reasoning`; the same pair under a rule that declares and discharges a well-founded measure settles `proved`. | Test (TC-983) |
| FR-489-AC-5 | A missing premise settles `premise-missing`, a trusted assumption keeps `trusted` and names the assumption, an unsupported rule settles `unsupported` naming the capability, a failed component check settles `premise-unproved` and an exhausted bound settles `incomplete`; the five are distinct with their own trust basis and extent. | Test (TC-983) |
| FR-489-AC-6 | A premise identity that names no settled result, and a premise whose subject differs from its component, are refused `invalid_runtime_input`/`invalid-value` naming the member; a referenced result that is itself `inconclusive` does not discharge the premise; sibling claims of the same request settle independently. | Test (TC-983) |

## Dependencies

- [FR-348](FR-348-admit-compositional-verification.md): composition negotiation;
  this requirement adds only the typed premise transport and its discharge record.
- [FR-176](../protocol/FR-176-compose-assume-guarantee-contracts.md) and
  [FR-177](../protocol/FR-177-check-protocol-refinement.md):
  the assume-guarantee algorithm and fixed-point rule (FR-176) and protocol refinement (FR-177), reused unchanged.
- [FR-380](FR-380-transfer-claims-through-refinement.md): the refinement
  transfer, stutter-closure and fairness rules.
- [FR-362](../temporal/FR-362-apply-fairness-constraints.md),
  [FR-354](../ir-proof/FR-354-record-proof-trust-basis.md),
  [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md) and
  [FR-243](../../objects/protocol/FR-243-settlement-basis.md).

## References

- Tickets: Linear QSL-1542 (TL-9, RES-74, RES-35).
