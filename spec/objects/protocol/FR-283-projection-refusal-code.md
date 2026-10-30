---
id: FR-283
title: "Projection refusal code vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
---
# FR-283: Projection refusal code vocabulary

## Description

When a projection adapter refuses to map a protocol result to portable verification, the adapter SHALL emit exactly one code from this closed vocabulary rather than an English explanation.

This vocabulary is selected as
`quire.protocol.projection-refusal/v1-draft.1`. Its kebab-case members are exact
values in that catalog, not aliases for native producer codes in another catalog.

## Values

| Value | Description |
| --- | --- |
| `invalid-source-result` | The canonical protocol result did not validate before mapping was attempted (FR-062). |
| `unsupported-source-version` | The source result's contract version is not one this adapter maps from (FR-062). |
| `unsupported-target-version` | The requested target contract version is not one this adapter maps to (FR-062). |
| `identity-mismatch` | A supplied target context is missing or conflicting (FR-062). It does not classify invalid canonical source bytes. |
| `unrepresented-source-state` | A source state has no mapping in the selected table. It cannot be reclassified as a malformed source (FR-062). |
| `finding-loss` | The output would lose an applicable FR-007 finding (FR-062). |

These are kebab-case spellings, distinct from the snake_case native producer
codes of [FR-271](../foundation/FR-271-diagnostic-code-vocabulary.md). The
selected catalog plus exact spelling determines membership; there is no global
case convention or cross-catalog spelling alias.

## Identity and comparison

- **Identity kind** — *value*. A code label has no identity of its own and is
  compared componentwise as an exact string.
- **Key** — the label itself, matched exactly. Hyphenated and underscored
  spellings stay exact and are not normalized aliases
  ([native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)).
  Message text, locale, formatting and rendering never participate: FR-047-AC-1
  requires that changing or localizing human-readable messages leaves the typed
  failure's classification unchanged.
- **Key order** — lexicographic over the label, for collection comparison only.
  It expresses no severity and no precedence between refusals.
- **Causal order** — none.
- **Absence** — a refusal emitted without a code has an absent code, which
  compares only to absence and is never defaulted to `invalid-source-result` or
  to any other member.

## Resolved code and namespace ruling

A canonical source result whose strict-reader validation fails yields `invalid-source-result` before
mapping. `identity-mismatch` applies only after valid source acceptance to a
missing/conflicting target context. The two outcomes cannot share a code merely
because both mention an identity.

This catalog owns the six projection-adapter refusal spellings. FR-271's native
producer catalog is a separate selected namespace; equal-looking strings across
catalogs remain unequal and no snake/kebab normalization is performed.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-283-AC-1 | Each member's value equals the exact label string in the table, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-283-AC-2 | An unrepresented source state yields `unrepresented-source-state` and is not reclassified as `invalid-source-result`. | Analysis |
| FR-283-AC-3 | Changing or localizing the human-readable refusal message leaves the emitted code unchanged. | Analysis |
| FR-283-AC-4 | A canonical source result failing strict-reader validation yields `invalid-source-result`, while a missing or conflicting target context yields `identity-mismatch`; neither emits a `TechniqueResult`. | Analysis |

## Dependencies

- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  establishes the six codes and the refusal conditions.
- [FR-249](FR-249-derived-projection-result.md) is the result these refusals
  replace.
- [FR-271](../foundation/FR-271-diagnostic-code-vocabulary.md) holds the native
  producer code vocabulary this one is held distinct from.

## Status

Accepted FS02 projection-refusal vocabulary ruling. Adapter implementation and
mutation evidence remain planned under FR-062/TC-064.
