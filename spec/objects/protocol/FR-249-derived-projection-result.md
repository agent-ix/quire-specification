---
id: FR-249
title: "Derived portable projection result"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-240
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-249: Derived portable projection result

## Description

When a canonical protocol result is projected to portable verification, the projection adapter SHALL derive the `TechniqueResult.resultId` from the exact tuple stated by FR-062 so that changing any member of that tuple changes the derived result identity.

## Properties

| Property | Description |
| --- | --- |
| `resultId` | `sha256-jcs:<lowercase-hex>` over the RFC 8785 canonical identity preimage stated by FR-062. It is "distinct from the canonical protocol-result identity and stable for the exact tuple of canonical result identity and digest, target obligation and execution-binding identities, target contract version, and adapter identity and version" (FR-062). |
| `identityVersion` | `quire.protocol.e02-projection-identity/v1-draft.1`, the first member of the identity preimage (FR-062). |
| `sourceResultRef` | The exact canonical result identity and its `sha256-jcs` digest (FR-062). The adapter validates the canonical protocol result and recomputes its digest before mapping it. |
| `targetContractVersion` | `quire-verification/e02-draft-2` (FR-062). |
| `targetBinding` | `propertyRef`, `planId`, `taskId`, `runRef`, `profile`, `bindingRefs`, `dependencyClosure` and `executionScope`, each equal to its selected context; the adapter "SHALL NOT infer or substitute one from display text" (FR-062). |
| `adapter` | The exact adapter identity and version (FR-062). |
| `nativeArtifacts` | "Exactly one canonical-result entry with the source URI, recomputed digest and media type `application/vnd.agent-ix.quire-protocol-result+json;version=1-draft.1`" (FR-062). |
| `provenance` | Identifies the protocol producer plus this adapter and their versions (FR-062). |

## Identity and comparison

- **Identity kind** — *derived*. The identity is computed structurally from the
  named preimage members under the rule FR-062 states; it is not authored, and it
  is distinct from the canonical protocol-result identity
  ([FR-240](FR-240-canonical-protocol-result.md)).
- **Key** — the derivation tuple: canonical result identity and digest, target
  obligation and execution-binding identities, target contract version, and
  adapter identity and version. Equality is componentwise over that key and
  nothing else. Display name, source path, span, message text, locale and
  rendering never participate; FR-062 forbids inferring or substituting a binding
  field "from display text", and FR-061 forbids inferring independence "from
  distinct display names or implementations".
- **Key order** — `bindingRefs` carries the only order the prose establishes, and
  it is a *semantic* order, not a key order: "`bindingRefs` uses the selected E02
  semantic order and SHALL NOT be resorted by presentation text" (FR-062). No
  requirement states a total order over derived result identities, so none is
  declared here. Any collection comparison over the preimage is fixed by RFC 8785
  canonicalization, which is a serialization rule and carries no causal meaning.
- **Causal order** — the source-to-derived edge only: the derived result
  references the canonical protocol result it was projected from. That edge is
  never derived from key order, timestamp or source position.
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value. FR-062 refuses the projection when "an exact context or
  canonical-result reference is missing or conflicting"; an absent context is a
  refusal, not an empty binding.
- **Digest domain** — `sha256-jcs`, whose domain vocabulary is
  [`ix://agent-ix/quire-specification/FR-201`](../foundation/FR-201-canonical-identity-domain.md).
  FR-201 records that this preimage is a distinct per-use selection: the derived
  identity is taken "over a distinct projection-identity preimage under
  `quire.protocol.e02-projection-identity/v1-draft.1`". An identity crossing
  domains is unequal regardless of byte equality.

## Refusal codes

FR-062 Outputs states the stable codes verbatim: "one refusal carrying a stable
code from `invalid-source-result`, `unsupported-source-version`,
`unsupported-target-version`, `identity-mismatch`, `unrepresented-source-state` or
`finding-loss`, plus the affected exact identity."

| Code | Description |
| --- | --- |
| `invalid-source-result` | The source result fails its strict reader (FR-062). |
| `unsupported-source-version` | The source contract version or a required feature is unknown (FR-062). |
| `unsupported-target-version` | The target contract version or a required feature is unknown (FR-062). |
| `identity-mismatch` | An exact context or canonical-result reference is missing or conflicting; substituting any identity or digest refuses the projection (FR-062, FR-062-AC-2). |
| `unrepresented-source-state` | A source state has no mapping in the selected table (FR-062). |
| `finding-loss` | The output would lose an applicable quire-verification FR-007 finding (FR-062). |

A refusal is emitted without a `TechniqueResult` (FR-062, FR-062-AC-4). Because
the parent batch allocated ten identifiers for sixteen typed nouns, these codes
have no separate `enumeration` artifact yet; this artifact records them in place
and does not claim to be one.

## Retention boundary

FR-062 keeps the canonical protocol result authoritative. Protocol adequacy,
completeness, the four decision-scope and surrounding-execution progress/closure
axes, complete-global-conformance closure, settlement basis and claim are
"Retained in the canonical protocol-result artifact; never substituted for an E02
finding axis." The adapter "SHALL reuse Quoin evidence authority and SHALL NOT
emit a sufficiency decision."

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-249-AC-1 | The derived identity is distinct from the canonical protocol-result identity and is stable for the exact derivation tuple. | Analysis |
| FR-249-AC-2 | Changing any member of the derivation tuple changes the derived result identity. | Analysis |
| FR-249-AC-3 | `nativeArtifacts` carries exactly one canonical-result entry with source URI, recomputed digest and the stated media type. | Inspection |
| FR-249-AC-4 | `bindingRefs` retains the selected E02 semantic order and is not resorted by presentation text. | Analysis |
| FR-249-AC-5 | Each refusal carries exactly one stable code from the table plus the affected exact identity, and no `TechniqueResult` is emitted with it. | Inspection |
| FR-249-AC-6 | An absent context component compares only to absence and is refused rather than defaulted. | Analysis |

## Dependencies

- [FR-062](../../functional/FR-062-project-results-to-portable-verification.md)
  establishes the derivation tuple, preimage, refusal codes and retention
  boundary.
- [FR-240](FR-240-canonical-protocol-result.md) supplies the source canonical
  result.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain vocabulary.

## Open gap

The repo review
[`SR-208`](../../reviews/protocol-verification-projection-object.md) FND-001
records that FR-061 "defines no total conversion for protocol execution states
(`refused`, `resource-incomplete`), truth states (`pending`, `unavailable`),
activation, participation or assumptions into E02's different closed
vocabularies." FR-062 supplies a mapping table that it declares "total for the
selected source and target revisions", but two of its rows key on source axes no
requirement enumerates: participation (see
[FR-240](FR-240-canonical-protocol-result.md) open gap 4) and the four
progress/closure axes (open gap 3). Totality over an unenumerated source domain is
not established here, and this artifact does not supply the missing source
vocabularies.
