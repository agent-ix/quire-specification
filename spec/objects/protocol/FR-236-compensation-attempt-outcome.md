---
id: FR-236
title: "Compensation attempt outcome vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-058
    type: specifies
---
# FR-236: Compensation attempt outcome vocabulary

## Description

When multiple compensation observations relate to one obligation, the conformance evaluator SHALL report per-attempt and aggregate recovery facts with explicit failed, timed-out, duplicate, pending and successful outcomes (FR-058).

## Values

| Value | Description |
| --- | --- |
| `failed` | An attempt that did not succeed. Held distinct from a timed-out attempt and from a duplicate effect (FR-058-AC-2). A failed attempt is retained even when a later retry succeeds (FR-058-AC-1). |
| `timed-out` | An attempt that timed out. Held distinct from a failed attempt and from a duplicate effect (FR-058-AC-2). |
| `duplicate` | A duplicate business effect. FR-058 requires the evaluator to "distinguish a repeated receipt, retried attempt and duplicate business effect"; FR-051-AC-2 refuses relabelling "a duplicate delivery as a second business effect". |
| `pending` | An outcome not yet established. FR-057-AC-4 states that missing state, commit or compensation-effect inputs "yield incomplete/pending rather than recovery success". |
| `successful` | An attempt whose compensation operation succeeded. FR-057 states that the evaluator "SHALL NOT infer recovery success from compensation-operation success", so this value alone is not a recovery result. |

## Identity and comparison

- **Identity kind** — *value*. An outcome label has no identity of its own; it is
  compared componentwise as an exact string.
- **Key** — the label itself, matched exactly. Spellings are not normalized
  aliases, and a hyphenated and an underscored spelling are different labels.
  Display name, source path, span, message text, locale, rendering and diagnostic
  prose never participate (FR-009, FR-047).
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no severity ranking, no precedence and no causal meaning; in
  particular `successful` does not dominate `failed` under it.
- **Causal order** — none. An outcome is a retained fact about one attempt. The
  order of the attempts it labels is the authored partial order of
  [FR-052](../../functional/FR-052-represent-bounded-control.md) and is never
  derived from key order, timestamp or source position.
- **Absence** — an attempt whose outcome has not been established has an absent
  outcome. Absence compares only to absence and is never defaulted to `pending`,
  `failed` or any other member.
- **Digest domain** — not applicable; the key is a label, not a digest.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-236-AC-1 | A timed-out attempt, a failed attempt and a duplicate effect compare unequal and remain three distinct outcomes. | Analysis |
| FR-236-AC-2 | Each member's value equals the exact label string in the table, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-236-AC-3 | An attempt with an absent outcome receives no default member. | Analysis |
| FR-236-AC-4 | A `successful` attempt outcome alone does not read as a recovery result. | Analysis |

## Dependencies

- [FR-058](../../functional/FR-058-preserve-retry-and-partial-recovery.md)
  establishes the outcome vocabulary and the distinctness rules.
- [FR-057](../../functional/FR-057-enforce-commit-recovery.md) establishes that
  recovery success is not inferred from compensation-operation success.
- [FR-051](../../functional/FR-051-preserve-communication-identities.md)
  establishes duplicate-delivery distinctness.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

FR-058 Outputs lists these five outcomes for "Per-attempt and aggregate recovery
facts" together, without stating which members are admissible per attempt and
which are admissible only for the aggregate. `duplicate` in particular labels a
business effect rather than an attempt. No requirement partitions the vocabulary,
and this artifact does not partition it.

## Status

Draft. This artifact records existing meaning structurally. It establishes no new
obligation and claims no implementation coverage.
