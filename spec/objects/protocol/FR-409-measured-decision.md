---
id: FR-409
title: "Measured decision vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-408
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-241
    type: references
  - target: ix://agent-ix/quire-specification/FR-242
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
---

# FR-409: Measured decision vocabulary

## Description

When a result carries statistical evidence, the result producer SHALL carry
exactly one member of this closed `measured` vocabulary on its own axis,
beside FR-241's execution disposition, and SHALL carry no FR-242 truth and no
FR-243 settlement basis.

## Values

| Value | Description |
| --- | --- |
| `accepted` | Every test of the run accepted, each with its method's error guarantee ([FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md)). Read as "measured: accepted". Pipeline category: success; it is never proof evidence. |
| `rejected` | At least one test rejected, with its method's error guarantee, or a drawn sample evaluated the claim undefined, with cause `undefined-evaluation` ([FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md)). Read as "measured: rejected". Pipeline category: violation; it fails the pipeline exactly as a violated claim does, and it is never a refutation. |
| `undecided` | The run completed its method and neither decision holds: the interval contains the threshold (`indifference-region`), or a step's contract evaluated undecided (`undecided-successor`). Read as "measured: undecided". Pipeline category: inconclusive. |

## Combination rules

- A result with a `measured` value carries FR-241 execution `completed`, the
  FR-331 result value `measured`, and the statistical verdict of
  [FR-410](../interfaces/FR-410-statistical-verdict.md).
- A result with a `measured` value carries no FR-242 truth and no FR-243
  settlement basis. A reader SHALL refuse a result that carries both a
  `measured` value and a truth or basis, with `invalid_result`/`measured-with-truth`.
- A statistical run stopped by a budget, refused, unsupported or failed
  carries FR-241 execution `resource-incomplete`, `refused`, `unsupported` or
  `failed` and no `measured` value.
- Proof accounting reads FR-331 result values; `measured` is not `proved`, so
  no `measured` value counts as proof evidence.
- A `rejected` value gates the pipeline as a violation does: a request whose
  items include a `rejected` measurement does not pass.

## Identity and comparison

- **Identity kind** — *value*. A decision label has no identity of its own and
  is compared as an exact string.
- **Key** — the label itself, matched exactly.
- **Key order** — lexicographic over the label, for collection comparison
  only; it carries no strength ranking.
- **Causal order** — none.
- **Absence** — an absent `measured` value compares only to absence and is
  never defaulted to any member, and never read as truth `unavailable`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-409-AC-1 | Each member's value equals the exact label string in the table. | Test (TC-358) |
| FR-409-AC-2 | A result carrying a `measured` value with an FR-242 truth or an FR-243 basis is refused `invalid_result`/`measured-with-truth`. | Test (TC-358) |
| FR-409-AC-3 | `accepted` maps to the success category and is never counted as proof evidence; `rejected` fails the pipeline as a violation and is never `refuted`; `undecided` maps to inconclusive. | Test (TC-358) |
| FR-409-AC-4 | A stopped, refused, unsupported or failed statistical run carries its FR-241 execution disposition and no `measured` value. | Test (TC-358) |
| FR-409-AC-5 | An absent `measured` value compares only to absence and receives no default member. | Test (TC-358) |

## Dependencies

- [FR-241](FR-241-assessment-execution-disposition.md) supplies the execution
  axis this vocabulary sits beside.
- [FR-242](FR-242-truth-disposition.md) and [FR-243](FR-243-settlement-basis.md)
  keep their meaning for truth results; this axis is distinct from both.
- [FR-331](../interfaces/FR-331-backend-provider-envelope.md) carries the
  result value `measured`.

## References

- QSL ADR-024 SV-1, SV-3, SV-10, RU-2, RU-5.
- Owner ruling recorded on QSL-371: `measured: rejected` fails the pipeline;
  a separate `measured` axis, not an FR-243 basis.
- Ticket: STD-137 (paired with QSL-371).
