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

## Values

| Value | Description |
| --- | --- |
| `invalid-source-result` | The canonical protocol result did not validate, or its recomputed `sha256-jcs` digest did not match, before mapping was attempted (FR-062). |
| `unsupported-source-version` | The source result's contract version is not one this adapter maps from (FR-062). |
| `unsupported-target-version` | The requested target contract version is not one this adapter maps to (FR-062). |
| `identity-mismatch` | A supplied identity does not agree with the identity derived from the selected tuple (FR-062). |
| `unrepresented-source-state` | A source state has no mapping in the selected table. It cannot be reclassified as a malformed source (FR-062). |
| `finding-loss` | The output would lose an applicable FR-007 finding (FR-062). |

The `invalid-source-result` and `identity-mismatch` rows above do not agree with
[FR-249](FR-249-derived-projection-result.md) on which code a recomputed-digest
mismatch receives, and no requirement settles it. That disagreement is recorded
in full as the first entry of the [FR-249 open gap](FR-249-derived-projection-result.md#open-gap)
and is not resolved here.

These are kebab-case spellings, distinct from the snake_case native producer
codes of [FR-271](../foundation/FR-271-diagnostic-code-vocabulary.md). No
requirement states which convention belongs to which namespace; see
**Open gap**.

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
- **Digest domain** — not applicable. The digest whose mismatch produces
  `invalid-source-result` is `sha256-jcs` under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Open gap

**Which code a recomputed-digest mismatch receives is unresolved.** This
artifact assigns it to `invalid-source-result` and
[FR-249](FR-249-derived-projection-result.md) assigns it to `identity-mismatch`;
FR-062-AC-4 assigns it to neither. The full record of the two assignments is the
first entry of the [FR-249 open gap](FR-249-derived-projection-result.md#open-gap).

The repository carries two code-spelling conventions and no rule assigning them
to namespaces:

- FR-003-AC-2, FR-008-AC-4 and FR-009-AC-3 use bare snake_case literals
  (`invalid_source_map`, `unavailable_observation`, `foreign_reference`) without
  naming a selected catalog.
- FR-062 uses the kebab-case literals above.

The native diagnostics catalog states that "Hyphenated and underscored spellings
stay exact; they are not normalized aliases", which makes the difference
load-bearing. No requirement says which convention belongs to which namespace,
nor whether these projection codes belong to a versioned catalog at all in the
sense FR-047 requires. This artifact records both conventions and resolves
neither.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-283-AC-1 | Each member's value equals the exact label string in the table, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-283-AC-2 | An unrepresented source state yields `unrepresented-source-state` and is not reclassified as `invalid-source-result`. | Analysis |
| FR-283-AC-3 | Changing or localizing the human-readable refusal message leaves the emitted code unchanged. | Analysis |

## Dependencies

- [FR-062](../../functional/FR-062-project-results-to-portable-verification.md)
  establishes the six codes and the refusal conditions.
- [FR-249](FR-249-derived-projection-result.md) is the result these refusals
  replace.
- [FR-271](../foundation/FR-271-diagnostic-code-vocabulary.md) holds the native
  producer code vocabulary this one is held distinct from.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
