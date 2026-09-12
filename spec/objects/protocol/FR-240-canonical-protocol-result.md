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
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
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
  - target: ix://agent-ix/quire-specification/FR-284
    type: references
---
# FR-240: Canonical protocol result

## Description

When a protocol assessment result is emitted or read, the strict reader SHALL resolve the result's contract version, kind, identity, enclosing artifact reference and retained fact set through this object artifact rather than through a restatement in an individual behavioral requirement.

## Properties

Every property below is stated by [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
or [FR-059](../../functional/protocol/FR-059-assess-finite-global-conformance.md). This
artifact adds none.

| Property | Description |
| --- | --- |
| `contractVersion` | Exactly `quire.protocol.result/v1-draft.1` on every canonical protocol result (FR-061). |
| `kind` | Exactly `ProtocolResult` (FR-061). |
| result identity | The result's own identity (FR-061, FR-061-AC-3). Its derivation rule is not established; see **Open gap**. |
| enclosing artifact reference | Carries the `sha256-jcs` digest over the complete RFC 8785 canonical result bytes. The digest is **not** a self-referential field inside those bytes, and a strict reader recomputes it before accepting the result (FR-061, FR-061-AC-3). |
| assessment execution | One state stating whether the requested evaluator completed or why it did not ([FR-241](FR-241-assessment-execution-disposition.md)). |
| decision-scope progress | Independent fact with exact scope, authority and boundary identities, carrying one state from [FR-284](FR-284-scope-progress-and-closure-state.md) (FR-061). |
| decision-scope closure | Independent fact with exact scope, authority and boundary identities, carrying one state from [FR-284](FR-284-scope-progress-and-closure-state.md) (FR-061). |
| surrounding-execution progress | Independent fact with exact scope, authority and boundary identities, carrying one state from [FR-284](FR-284-scope-progress-and-closure-state.md) (FR-061). |
| surrounding-execution closure | Independent fact with exact scope, authority and boundary identities, carrying one state from [FR-284](FR-284-scope-progress-and-closure-state.md) (FR-061). |
| complete-global-conformance closure | The fifth independent fact, with its exact required execution, branch and workflow closure-premise identities for a global-conformance claim ([FR-247](FR-247-complete-global-conformance-closure-state.md)). |
| truth | One truth disposition ([FR-242](FR-242-truth-disposition.md)). |
| settlement basis | Exactly one basis per truth result, plus the exact supporting fact and progress identities ([FR-243](FR-243-settlement-basis.md)). |
| activation | One activation state ([FR-244](FR-244-activation-state.md)). |
| participation | An independently retained axis (FR-061, FR-115). Not enumerated by FR-061 or FR-115; see **Open gap**. |
| assumptions | Each assumption retains its exact assumption identity and one state from `met`, `not-met`, `unknown` or `not-assessed` (FR-061). The state vocabulary is defined by [FR-280](FR-280-assumption-state.md). |
| completeness | States which facts required by the returned claim are available and identifies every missing dependency (FR-061). |
| observation adequacy | Whether the exact observations required to interpret the requested result are adequate ([FR-245](FR-245-observation-adequacy-state.md)). |
| protocol adequacy | Whether the exact declared obligation and case set is demonstrated ([FR-246](FR-246-protocol-adequacy-state.md)). |
| claim | The requested protocol claim and its retained claim strength. The strength vocabulary is defined by [FR-281](FR-281-claim-strength.md); the protocol claim kinds have no label spellings in the prose, see **Protocol claim kind**. |
| decision support | Every retained decision-support record ([FR-248](FR-248-decision-support-record.md)). |
| provenance | Source, profile, model, protocol, request, binding, input and backend identities (FR-061-AC-3). |

## Identity and comparison

- **Identity kind** — the result carries an explicit **result identity** (FR-061).
  The prose does not state whether that identity is authored by the producer or
  derived from the result's own components, so this artifact declares neither;
  see **Open gap**.
- **Key** — the result identity. Equality is componentwise over the key and
  nothing else. Display name, source path, span, message text, locale and
  rendering never participate (FR-200, FR-009). FR-061 supports this directly: a
  strict reader "SHALL NOT infer independence from distinct display names or
  implementations".
- **Key order** — **none is established.** No requirement states a total order
  over result identities. This artifact does not supply one. Collection
  comparison therefore has no ordering authority here.
- **Causal order** — supersession lineage only. When a later observation,
  progress or closure record contradicts a premise in a settled result, the
  producer retains the earlier result bytes and emits a typed superseding or
  invalidating result that identifies the earlier result (FR-061, FR-061-AC-9).
  That lineage is never derived from key order, timestamp or source position.
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value (FR-200).
- **Digest domain** — the enclosing artifact reference's digest is
  `sha256-jcs`, whose domain vocabulary is
  [`ix://agent-ix/quire-specification/FR-201`](../foundation/FR-201-canonical-identity-domain.md).
  An identity crossing domains is unequal regardless of byte equality.

## Five independent closure and progress axes

FR-061 states that decision-scope progress, decision-scope closure,
surrounding-execution progress, surrounding-execution closure and
complete-global-conformance closure are five independent facts: "The result
producer SHALL NOT use completeness, assessment execution or any other
progress/closure fact as a substitute for any closure axis." This artifact
retains them as five properties and merges none of them.

## Vocabularies carried on this result

The two closed vocabularies below are stated verbatim by the behavioral
requirements and each has its own `enumeration` artifact, which is the
definition. The tables are repeated here only as a reading aid; where the two
differ the referenced artifact governs.

**Assumption state** — defined by [FR-280](FR-280-assumption-state.md)
(FR-061: "one state from `met`, `not-met`, `unknown` or `not-assessed`";
restated as source states in the FR-062 mapping table):
`met`, `not-met`, `unknown`, `not-assessed`.

**Claim strength** — defined by [FR-281](FR-281-claim-strength.md)
(FR-061: "The result producer SHALL retain proven, bounded-checked, tested and
observed claim strengths as distinct source-bound values"):
`proven`, `bounded-checked`, `tested`, `observed`.

## Protocol claim kind

**Protocol claim kind** ([FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md):
"one independent disposition for global conformance, monitorability, local
projection, refinement, realizability and composition without transferring
premises or success among them"): global conformance, monitorability, local
projection, refinement, realizability, composition. FR-060 names these in prose
and does not give them label spellings; this artifact does not invent any.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-240-AC-1 | Every canonical protocol result carries `contractVersion` equal to `quire.protocol.result/v1-draft.1` and kind `ProtocolResult`. | Inspection |
| FR-240-AC-2 | The enclosing artifact reference's digest is computed over the complete canonical result bytes and does not appear as a field inside those bytes. | Analysis |
| FR-240-AC-3 | The five progress and closure axes remain independently readable; no axis is populated from completeness or assessment execution. | Analysis |
| FR-240-AC-6 | Each of the four decision-scope and surrounding-execution axes carries exactly one FR-284 member, and the complete-global-conformance axis carries exactly one FR-247 member. | Analysis |
| FR-240-AC-4 | Two results are compared over the result identity alone; equal display names or implementations establish neither equality nor independence. | Analysis |
| FR-240-AC-5 | No result-identity ordering is asserted by this artifact or inferred by a reader. | Inspection |

## Dependencies

- [FR-059](../../functional/protocol/FR-059-assess-finite-global-conformance.md) supplies
  the complete-global-conformance closure record and its premise set.
- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md) supplies the
  independent per-claim dispositions.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) supplies the
  contract version, kind, identity, enclosing reference digest rule and every
  retained fact.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain vocabulary.
- [FR-280](FR-280-assumption-state.md) defines the assumption state vocabulary
  carried here.
- [FR-281](FR-281-claim-strength.md) defines the claim strength vocabulary
  carried here.
- [FR-284](FR-284-scope-progress-and-closure-state.md) defines the state
  vocabulary the four progress and closure axes carry.

## Open gap

1. **Result-identity derivation.** FR-061 requires "a result identity" and
   FR-061-AC-4 refuses a "stale identity", but no requirement states whether the
   identity is authored by the producer or derived from named components. The
   identity kind is therefore not declared here. By contrast,
   [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
   fully specifies the *derived* projection identity
   ([FR-249](FR-249-derived-projection-result.md)).
2. **Identity versus bytes.** No requirement states whether two results bearing
   the same result identity and different canonical bytes are admissible.
3. **Participation is unenumerated at the source.** FR-061 and FR-115 both name
   participation as a retained axis and neither lists its members. The only
   enumerated participation vocabulary in the specification is FR-062's **E02
   target** vocabulary (`complete` / `partial` / `not-started` / `not-required` /
   `unknown`), which is the projection target rather than the protocol source
   vocabulary. This artifact does not adopt a target vocabulary as a source one.
4. The repo review
   [`SR-208`](../../reviews/protocol-verification-projection-object.md) FND-001
   already records that "FR-061 names a mapping but defines no total conversion
   for protocol execution states ... into E02's different closed vocabularies."
   Gap 3 is that finding seen from the source side; the four progress and closure
   axes it also covered are now enumerated by
   [FR-284](FR-284-scope-progress-and-closure-state.md).

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
