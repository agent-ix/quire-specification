---
id: FR-240
title: "Canonical protocol result"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-059
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-060
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-241
    type: references
  - target: ix://agent-ix/quire-specification/FR-242
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-244
    type: references
  - target: ix://agent-ix/quire-specification/FR-245
    type: references
  - target: ix://agent-ix/quire-specification/FR-246
    type: references
  - target: ix://agent-ix/quire-specification/FR-247
    type: references
  - target: ix://agent-ix/quire-specification/FR-248
    type: references
  - target: ix://agent-ix/quire-specification/FR-280
    type: references
  - target: ix://agent-ix/quire-specification/FR-281
    type: references
  - target: ix://agent-ix/engineering-assurance/FR-022
    type: references
  - target: ix://agent-ix/quire-specification/FR-284
    type: references
  - target: ix://agent-ix/quire-specification/FR-285
    type: references
  - target: ix://agent-ix/quire-specification/FR-286
    type: references
  - target: ix://agent-ix/quire-specification/FR-295
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-291
    type: references
  - target: ix://agent-ix/quire-specification/FR-292
    type: references
---
# FR-240: Canonical protocol result

## Description

When a protocol assessment result is emitted or read, the strict reader SHALL resolve the result's contract version, kind, identity and retained fact set through this object artifact rather than through a restatement in an individual behavioral requirement.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contractVersion | String | 1..1 |  |
| kind | String | 1..1 |  |
| resultKey | CanonicalProtocolResultKey | 1..1 | identity |
| assessment_execution | AssessmentExecutionDispositionVocabulary | 1..1 |  |
| decision_scope_progress | ScopeProgressAndClosureStateVocabulary | 1..1 |  |
| decision_scope_closure | ScopeProgressAndClosureStateVocabulary | 1..1 |  |
| surrounding_execution_progress | ScopeProgressAndClosureStateVocabulary | 1..1 |  |
| surrounding_execution_closure | ScopeProgressAndClosureStateVocabulary | 1..1 |  |
| complete_global_conformance_closure | CompleteGlobalConformanceClosureStateVocabulary | 1..1 |  |
| truth | TruthDispositionVocabulary | 1..1 |  |
| settlement_basis | SettlementBasisVocabulary | 1..1 |  |
| activation | ActivationStateVocabulary | 1..1 |  |
| participation | ParticipationFact | 1..1 |  |
| assumptions | AssumptionStateVocabulary | 1..1 |  |
| completeness | String | 1..1 |  |
| observation_adequacy | ObservationAdequacyStateVocabulary | 1..1 |  |
| protocol_adequacy | ProtocolAdequacyStateVocabulary | 1..1 |  |
| claim | String | 1..1 |  |
| decision_support | DecisionSupportRecord | 1..1 |  |
| provenance | String | 1..1 |  |

## Field sources

Every property below is stated by [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
or [FR-059](../../functional/protocol/FR-059-assess-finite-global-conformance.md). This
artifact adds none.

| Property | Description |
| --- | --- |
| `contractVersion` | Exactly `quire.protocol.result/v1-draft.1` on every canonical protocol result (FR-061). |
| `kind` | Exactly `ProtocolResult` (FR-061). |
| `resultKey` | The structural key `{producer, request, ordinal}` defined by [FR-286](FR-286-canonical-protocol-result-identity-preimage.md). |
| assessment execution | One state stating whether the requested evaluator completed or why it did not ([FR-241](FR-241-assessment-execution-disposition.md)). |
| decision-scope progress | Independent fact with exact scope, authority and boundary identities, carrying one state from [FR-284](FR-284-scope-progress-and-closure-state.md) (FR-061). |
| decision-scope closure | Independent fact with exact scope, authority and boundary identities, carrying one state from [FR-284](FR-284-scope-progress-and-closure-state.md) (FR-061). |
| surrounding-execution progress | Independent fact with exact scope, authority and boundary identities, carrying one state from [FR-284](FR-284-scope-progress-and-closure-state.md) (FR-061). |
| surrounding-execution closure | Independent fact with exact scope, authority and boundary identities, carrying one state from [FR-284](FR-284-scope-progress-and-closure-state.md) (FR-061). |
| complete-global-conformance closure | The fifth independent fact, with its exact required execution, branch and workflow closure-premise identities for a global-conformance claim ([FR-247](FR-247-complete-global-conformance-closure-state.md)). |
| truth | One truth disposition ([FR-242](FR-242-truth-disposition.md)). |
| settlement basis | Exactly one basis per truth result, plus the exact supporting fact and progress identities ([FR-243](FR-243-settlement-basis.md)). |
| activation | One activation state ([FR-244](FR-244-activation-state.md)). |
| participation | Exactly one closed [FR-285](FR-285-participation-fact.md) `known` or `unknown` value object. It is not a source status enum. |
| assumptions | Each assumption retains its exact assumption identity and one state from `met`, `not-met`, `unknown` or `not-assessed` (FR-061). The state vocabulary is defined by [FR-280](FR-280-assumption-state.md). |
| completeness | One FR-295 state derived from the exact required population/boundary/assertion/per-fact status set; it remains independent from progress, closure and execution. |
| observation adequacy | Whether the exact observations required to interpret the requested result are adequate ([FR-245](FR-245-observation-adequacy-state.md)). |
| protocol adequacy | Whether the exact declared obligation and case set is demonstrated ([FR-246](FR-246-protocol-adequacy-state.md)). |
| claim | Exactly one FR-290 protocol-family claim kind and, for an advanced result, one FR-281 strength, FR-291 admitted fragment and FR-292 backend selection. |
| decision support | Every retained decision-support record ([FR-248](FR-248-decision-support-record.md)). |
| provenance | Source, profile, model, protocol, request, binding, input and backend identities (FR-061-AC-3). |

## Identity and comparison

- **Identity kind** — *entity*. FR-286 defines the structural `resultKey` and
  validation order.
- **Key** — the FR-286 `resultKey`. Equality is componentwise over the key and
  nothing else. Display name, source path, span, message text, locale and
  rendering never participate (FR-200, FR-009). FR-061 supports this directly: a
  strict reader "SHALL NOT infer independence from distinct display names or
  implementations".
- **Key order** — **none is established.** No requirement states a total order
  over result identities. This artifact does not supply one. Collection
  comparison therefore has no ordering authority here.
- **Byte uniqueness** — two accepted results with one identity have
  byte-identical RFC 8785 canonical result bytes. Same identity on different
  bytes is refused. Presentation permutations canonicalizing to the same bytes
  are not different results.
- **Causal order** — supersession lineage only. When a later observation,
  progress or closure record contradicts a premise in a settled result, the
  producer retains the earlier result bytes and emits a typed superseding or
  invalidating result that identifies the earlier result (FR-061, FR-061-AC-9).
  That lineage is never derived from key order, timestamp or source position.
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value (FR-200).

## Five independent closure and progress axes

FR-061 states that decision-scope progress, decision-scope closure,
surrounding-execution progress, surrounding-execution closure and
complete-global-conformance closure are five independent facts: "The result
producer SHALL NOT use completeness, assessment execution or any other
progress/closure fact as a substitute for any closure axis." This artifact
retains them as five properties and merges none of them.

## Vocabularies carried on this result

Each of the two closed vocabularies below has its own governing artifact,
which states how a result carries it. The assumption-state list is
repeated here only as a reading aid; where the list and FR-280 differ,
FR-280 governs.

**Assumption state** — defined by [FR-280](FR-280-assumption-state.md)
(FR-061: "one state from `met`, `not-met`, `unknown` or `not-assessed`";
restated as source states in the FR-062 mapping table):
`met`, `not-met`, `unknown`, `not-assessed`.

**Claim strength** — the vocabulary is owned by Engineering Assurance
[FR-022](ix://agent-ix/engineering-assurance/FR-022), and
[FR-281](FR-281-claim-strength.md) states how a result carries it (FR-061:
"The result producer SHALL retain each claim strength from the Engineering
Assurance FR-022 vocabulary as a distinct source-bound value").

## Protocol claim authority

FR-290 defines the six exact protocol-family claim-kind spellings. This result's
contract version selects the FR-290 vocabulary `quire.capability-kind/v1`. FR-281 governs the
single-valued strength drawn from Engineering Assurance FR-022, FR-291 the admitted fragment and FR-292 the backend
selection. Equal display labels or implementation names establish none of them.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-240-AC-1 | Every canonical protocol result carries `contractVersion` equal to `quire.protocol.result/v1-draft.1` and kind `ProtocolResult`. | Test (TC-061) |
| FR-240-AC-2 | Withdrawn: existed only for the enclosing artifact digest, which is no longer computed. | Withdrawn |
| FR-240-AC-3 | The five progress and closure axes remain independently readable; no axis is populated from completeness or assessment execution. | Test (TC-061) |
| FR-240-AC-6 | Each of the four decision-scope and surrounding-execution axes carries exactly one FR-284 member, and the complete-global-conformance axis carries exactly one FR-247 member. | Test (TC-061) |
| FR-240-AC-4 | Two results are compared over the result key alone; equal display names or implementations establish neither equality nor independence. | Test (TC-061) |
| FR-240-AC-5 | No result-identity ordering is asserted by this artifact or inferred by a reader. | Test (TC-061) |
| FR-240-AC-7 | `resultKey` is the structural FR-286 key; the same key with different result members and any reader-supplied default key member are refused. | Test (TC-061) |
| FR-240-AC-8 | Participation carries exactly one FR-285 value object and never an E02 target status label. | Test (TC-061) |
| FR-240-AC-9 | Completeness carries exactly one FR-295 state derived from its exact population and remains independent from the five progress/closure facts. | Test (TC-061) |
| FR-240-AC-10 | Claim kind, strength, admitted fragment and backend retain their exact FR-290/281/291/292 shapes and compatible membership. | Test (TC-061) |

## Dependencies

- [FR-059](../../functional/protocol/FR-059-assess-finite-global-conformance.md) supplies
  the complete-global-conformance closure record and its premise set.
- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md) supplies the
  independent per-claim dispositions.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) supplies the
  contract version, kind, result key and every
  retained fact.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-280](FR-280-assumption-state.md) defines the assumption state vocabulary
  carried here.
- [FR-281](FR-281-claim-strength.md) states how the Engineering Assurance
  FR-022 claim strength vocabulary is carried here.
- [FR-284](FR-284-scope-progress-and-closure-state.md) defines the state
  vocabulary the four progress and closure axes carry.
- [FR-285](FR-285-participation-fact.md) defines source participation without
  importing E02 status labels.
- [FR-286](FR-286-canonical-protocol-result-identity-preimage.md) defines the
  canonical result key, uniqueness rule and reader order.
- [FR-295](../observation/FR-295-input-completeness-state.md) defines input
  completeness independently from progress and closure.
- [FR-290](FR-290-protocol-claim-kind.md), [FR-291](FR-291-admitted-analysis-fragment.md)
  and [FR-292](FR-292-analysis-backend.md) define claim dispatch authority.

## Resolved FS02 rulings

The result key is structural; the same key with different
result members is invalid; and participation is a discriminated source value
object rather than a status enumeration. FR-285 intentionally delegates the
role-instance key and canonical member order to FR-212. If that selected rule is
incomplete, construction or reordering refuses rather than falling back to
display names.

## Status

Accepted FS02 canonical-result ruling. Implementation and mutation evidence
remain planned under FR-061/TC-061.
