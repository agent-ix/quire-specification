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
  - target: ix://agent-ix/quire-specification/FR-283
    type: references
---
# FR-249: Derived portable projection result

## Description

When a canonical protocol result is projected to portable verification, the projection adapter SHALL derive the `TechniqueResult.resultId` from the exact tuple stated by FR-062 so that changing any member of that tuple changes the derived `resultId` or the target binding.

## Properties

| Property | Description |
| --- | --- |
| `resultId` | The closed object `{sourceResultKey, adapter}` stated by FR-062: the FR-286 `resultKey` of the source and the adapter identity and version. It nests at most three levels, carries no hash and does not repeat the target binding. |
| `sourceResultRef` | The exact canonical result key (FR-062), which is `resultId.sourceResultKey`. The adapter validates the canonical protocol result before mapping it. |
| `targetContractVersion` | `quire-verification/e02-draft-2` (FR-062). |
| `targetBinding` | `propertyRef`, `planId`, `taskId`, `runRef`, `profile`, `bindingRefs` and `executionScope`, each equal to its selected context; the adapter "SHALL NOT infer or substitute one from display text" (FR-062). |
| `adapter` | The exact adapter identity and version (FR-062). |
| `nativeArtifacts` | "Exactly one canonical-result entry with the source URI and media type `application/vnd.agent-ix.quire-protocol-result+json;version=1-draft.1`" (FR-062). |
| `provenance` | Identifies the protocol producer plus this adapter and their versions (FR-062). |

## Identity and comparison

- **Identity kind** — *derived*. The `resultId` is built structurally from the
  source `resultKey` and the adapter; it is not authored, and it
  is distinct from the canonical protocol-result key
  ([FR-240](FR-240-canonical-protocol-result.md)).
- **Key** — the canonical result key, target
  obligation and execution-binding identities, target contract version, and
  adapter identity and version; `resultId` carries only the result key and adapter. Equality is componentwise over that key and
  nothing else. Display name, source path, span, message text, locale and
  rendering never participate; FR-062 forbids inferring or substituting a binding
  field "from display text", and FR-061 forbids inferring independence "from
  distinct display names or implementations".
- **Key order** — `bindingRefs` carries the only order the prose establishes, and
  it is a *semantic* order, not a key order: "`bindingRefs` uses the selected E02
  semantic order and SHALL NOT be resorted by presentation text" (FR-062). No
  requirement states a total order over derived result identities, so none is
  declared here.
- **Causal order** — the source-to-derived edge only: the derived result
  references the canonical protocol result it was projected from. That edge is
  never derived from key order, timestamp or source position.
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value. FR-062 refuses the projection when "an exact context or
  canonical-result reference is missing or conflicting"; an absent context is a
  refusal, not an empty binding.

## Refusal codes

The refusal code vocabulary is **defined by
[FR-283](FR-283-projection-refusal-code.md)**; the table below is repeated here
only as a reading aid and FR-283 governs.

FR-062 Outputs states the stable codes verbatim: "one refusal carrying a stable
code from `invalid-source-result`, `unsupported-source-version`,
`unsupported-target-version`, `identity-mismatch`, `unrepresented-source-state` or
`finding-loss`, plus the affected exact identity."

| Code | Description |
| --- | --- |
| `invalid-source-result` | The source result fails its strict reader (FR-062). |
| `unsupported-source-version` | The source contract version or a required feature is unknown (FR-062). |
| `unsupported-target-version` | The target contract version or a required feature is unknown (FR-062). |
| `identity-mismatch` | An exact target context is missing/conflicting or the derived projection identity does not match its exact key (FR-062, FR-062-AC-2). |
| `unrepresented-source-state` | A source state has no mapping in the selected table (FR-062). |
| `finding-loss` | The output would lose an applicable quire-verification FR-007 finding (FR-062). |

A refusal is emitted without a `TechniqueResult` (FR-062, FR-062-AC-4).

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
| FR-249-AC-1 | The derived result ID is distinct from the canonical protocol-result key and is stable for the exact source result key and adapter identity and version. | Analysis |
| FR-249-AC-2 | Changing the source result key or the adapter identity or version changes the derived `resultId` or the target binding. | Analysis |
| FR-249-AC-3 | `nativeArtifacts` carries exactly one canonical-result entry with source URI and the stated media type. | Inspection |
| FR-249-AC-4 | `bindingRefs` retains the selected E02 semantic order and is not resorted by presentation text. | Analysis |
| FR-249-AC-5 | Each refusal carries exactly one stable code from the table plus the affected exact identity, and no `TechniqueResult` is emitted with it. | Inspection |
| FR-249-AC-6 | An absent context component compares only to absence and is refused rather than defaulted. | Analysis |

## Dependencies

- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  establishes the `resultId` object, refusal codes and retention
  boundary.
- [FR-240](FR-240-canonical-protocol-result.md) supplies the source canonical
  result.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-283](FR-283-projection-refusal-code.md) defines the refusal code
  vocabulary recorded above.

## Resolved refusal and source-domain boundary

The adapter validates the canonical source result
before deriving or checking the projection identity. Any failure in that source
validation yields
`invalid-source-result`. Only a valid source can reach target-context and
projection key checks; those mismatches yield `identity-mismatch`.

FR-062's mapping is total over the selected source and target revisions. FR-284
defines the four progress/closure axes, and
[FR-285](FR-285-participation-fact.md) defines the closed source participation
value object. Neither domain is inferred from E02 display text.

## Status

Accepted FS02 projection-result ruling. Implementation and mutation evidence
remain planned under FR-062/TC-064.
