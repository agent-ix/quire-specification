---
id: FR-295
title: "Input completeness state vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-115
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-284
    type: references
---
# FR-295: Input completeness state vocabulary

## Description

When input completeness is reported, the producer SHALL retain exactly one
state from this closed vocabulary together with the exact required population,
boundary, assertion authority and per-fact status set that establishes it.

## Values

| Value | Description |
| --- | --- |
| `complete` | Every member of the exact required population is available through the selected inclusive assessment boundary. |
| `incomplete` | At least one required member is missing or incomplete and none is contradicted. |
| `contradicted` | At least one retained fact or completeness premise is contradicted by an admitted correction or conflicting assertion. |

The state is derived from the complete retained per-fact population in the
precedence `contradicted`, `incomplete`, `complete`; it is not caller-authored.
Removing a required population member, omitting its status, or supplying a
foreign boundary cannot yield `complete`.

## Identity and comparison

- **Identity kind** — *value*. A state label has no identity of its own.
- **Key** — the exact label. The containing assertion is separately identified
  by its authority, population and boundary tuple.
- **Key order** — none. The derivation precedence above selects a state; it is
  not an ordering or severity relation over labels.
- **Causal order** — only an admitted correction relation may contradict an
  earlier fact. Label or timestamp order supplies no causal edge.
- **Absence** — absence compares only to absence and is never defaulted to
  `incomplete` or `complete`; a required absent axis is refused.

## Independence from progress and closure

FR-284 remains the sole two-member `open`/`closed` vocabulary for each of the
four decision-scope and surrounding-execution progress/closure axes. No FR-295
member is valid on one of those axes, and no FR-284 member is valid as
completeness. A producer SHALL NOT derive either axis from the other.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-295-AC-1 | Exact complete, incomplete and contradicted populations derive respectively `complete`, `incomplete` and `contradicted`. | Test (TC-061, TC-116, TC-145) |
| FR-295-AC-2 | Contradiction takes precedence over missing/incomplete members, and omission of a required member or status cannot derive `complete`. | Test (TC-061, TC-116) |
| FR-295-AC-3 | Every independent combination of an FR-295 state with the four FR-284 axes remains representable; cross-vocabulary labels are refused. | Test (TC-061, TC-116) |
| FR-295-AC-4 | An absent completeness axis is refused and is not defaulted. | Test (TC-061) |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
  declares the retained completeness population and result combination rules.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  declares completeness independence and correction behavior.
- [FR-284](../protocol/FR-284-scope-progress-and-closure-state.md) defines the
  distinct progress/closure vocabulary.

## Status

Proposed shared ruling for `quire-specification#40`; implementation evidence
remains owned by the cited behavioral test cases.
