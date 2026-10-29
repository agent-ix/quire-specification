---
id: FR-294
title: "Late-data classification"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-113
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-268
    type: references
---
# FR-294: Late-data classification

## Description

When an admitted observation is compared with the exact selected late-data
cutoff, the observation service SHALL retain exactly one timing classification.

## Values

| Value | Description |
| --- | --- |
| `on-time` | The observation satisfies the selected cutoff rule. |
| `late` | The observation does not satisfy the selected cutoff rule. |

Matching is exact. Aliases, case folding, Boolean coercion and unknown values
are refused. The selected cutoff contract owns the comparison; this enumeration
does not infer a cutoff from ingestion order or wall-clock time.

## Identity and comparison

- **Identity kind** — *value*.
- **Key** — the exact label.
- **Key order** — lexicographic for deterministic collection handling only.
- **Causal order** — none; `late` does not imply causation.
- **Absence** — absent compares only to absent and cannot default to `on-time`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-294-AC-1 | Exactly `on-time` and `late` are admitted, with exact spelling. | Test (TC-143) |
| FR-294-AC-2 | A missing or unknown classification is refused rather than defaulted. | Test (TC-143) |
| FR-294-AC-3 | A `late` classification never mutates an already selected result and never substitutes for FR-295 completeness. | Test (TC-143) |

## Dependencies

- [FR-113](../../functional/observation/FR-113-preserve-time-progress-and-late-data.md) establishes classification against a selected cutoff.
- [FR-268](FR-268-progress-and-late-data-authority.md) defines that cutoff's identity and use.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity discipline.

## Status

Proposed late-data vocabulary closure for `quire-specification#40`.
