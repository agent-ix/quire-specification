---
id: FR-290
title: "Protocol claim kind vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-060
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
---
# FR-290: Protocol claim kind vocabulary

## Description

When a protocol claim is requested, the dispatcher SHALL match exactly one
member of this closed vocabulary. The strict reader SHALL keep each reported
claim's premises and outcome independent from every other member.

## Values

| Value | Description |
| --- | --- |
| `global-conformance` | Finite global protocol conformance under the exact selected closure premises. |
| `monitorability` | Whether the selected admitted fragment can be monitored under the declared observations. |
| `local-projection` | Whether one selected participant projection is defined under its exact premises. |
| `refinement` | Whether one exact protocol/contract refines another under a selected relation. |
| `realizability` | Whether the selected protocol admits a causal implementation strategy. |
| `composition` | Whether selected components compose under independently discharged assumptions. |

## Identity and comparison

- **Identity kind** — *value*.
- **Key** — the exact label.
- **Key order** — none; declaration order carries no strength or dispatch
  precedence.
- **Causal order** — none.
- **Absence** — a requested/result claim with no kind is refused and never
  defaulted to global conformance.
- **Digest domain** — not applicable.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-290-AC-1 | The six exact labels are the complete accepted set; unknown, absent and display-derived labels are refused. | Test (TC-060, TC-061) |
| FR-290-AC-2 | Every pair of claim kinds may retain independent execution and outcome states without promotion or premise transfer. | Test (TC-060) |
| FR-290-AC-3 | A non-global claim cannot carry the complete-global-conformance premise set as evidence for its own success. | Test (TC-060, TC-061) |

## Dependencies

- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)
  declares independent dispatch for the six claim kinds.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
  declares their canonical result retention and non-substitution rules.

## Status

Proposed shared ruling for `quire-specification#40`.
