---
id: FR-293
title: "History boundary kind vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-092
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-257
    type: references
---
# FR-293: History boundary kind vocabulary

## Description

When a bounded past evaluator reads a history boundary, it SHALL match exactly
one member of this closed vocabulary and preserve the boundary's clock,
position, coverage and authority facts.

## Values

| Value | Description |
| --- | --- |
| `execution-origin` | The exact authoritative origin of the assessed execution; the selected false-extension profile may interpret positions before it. |
| `history-cutoff` | The earliest position supplied by a retained history or admitted state restoration; it proves no fact about earlier execution positions. |

A state-restoration contract may authenticate complete history through a
`history-cutoff`; it cannot relabel that cutoff as an execution origin. A
missing, evicted or unmatched predecessor remains missing history.

## Identity and comparison

- **Identity kind** — *value*.
- **Key** — the exact label. The containing FR-257 boundary additionally keys
  clock, position and authority.
- **Key order** — none.
- **Causal order** — none; the selected clock/sequence authority supplies any
  admitted causal order.
- **Absence** — absent/unknown kind is refused and never defaulted to either
  member.
- **Digest domain** — not applicable.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-293-AC-1 | Exactly `execution-origin` and `history-cutoff` are admitted, and equal positions under the two compare unequal. | Test (TC-112) |
| FR-293-AC-2 | State restoration authenticates cutoff coverage but cannot authorize false extension before the cutoff. | Test (TC-118) |
| FR-293-AC-3 | Unknown, absent, display-derived or cross-wired kinds refuse without a Boolean result. | Test (TC-112, TC-118) |

## Dependencies

- [FR-092](../../functional/temporal/FR-092-evaluate-bounded-past.md) defines
  origin/cutoff evaluation.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  defines restart and restoration behavior.
- [FR-257](FR-257-history-boundary.md) defines the containing boundary.

## Status

Proposed shared ruling for `quire-specification#40`.
