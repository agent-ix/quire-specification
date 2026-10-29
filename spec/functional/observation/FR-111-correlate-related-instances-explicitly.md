---
id: FR-111
title: "Correlate related workflow instances only through declared relationships"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
  - target: ix://agent-ix/quire-specification/FR-262
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---

> BLOCKED-BY QSpec #513: this file still cites the removed `sha256-jcs` / `quire.verification.jcs` identity domain (FR-201); delete that wording when #513 lands.
## Description

When associating an observation with a workflow subject, the correlation service SHALL select only an explicit typed relationship whose
relationship declaration is an effective domain declaration of the checked
package, with its exact declaration key and ordered endpoints.
The correlation outcomes are exactly `resolved`, `missing`, `conflicting` and
`ambiguous`; the latter three are typed non-success outcomes rather than
heuristic matches.

## Behavior

Workflow instances, messages, receive occurrences, deliveries, attempts,
effects and receipt observations SHALL remain distinct. A receipt refers to a delivery;
it is not the message, receive occurrence or delivery itself. A duplicate receipt SHALL NOT establish a second business effect. A refund is admitted only with its
compensated effect and order relationship; a telemetry trace ID or shared
provider identity does not establish either relationship.
The correlation service SHALL refuse a missing domain package identity, a
relationship declaration or kind outside the checked package, a missing digest
domain, or a declaration digest mismatch rather than reconstructing a relationship
from a foreign key, trace ID, provider name, or ambient configuration.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-111-AC-1 | A refund linked to O1 is refused for O2 even when both orders share a provider or trace infrastructure. | Test (TC-141) |
| FR-111-AC-2 | Missing, conflicting and ambiguous required relationships yield three distinct non-success outcomes; missing is incomplete, while conflicting and ambiguous refuse. `unknown` is not a relationship outcome. | Test (TC-141) |
| FR-111-AC-3 | Repeated delivery is retained as a delivery and cannot alone create a second attempt or effect. | Test (TC-141) |
| FR-111-AC-4 | A missing relationship declaration key, a declaration outside the checked package, or a declaration digest mismatch is refused without reconstructing a relationship. | Test (TC-141) |
| FR-111-AC-5 | Each endpoint retains its complete FR-287 key in authored source/target order; reversing, sorting, emptying, foreign-qualifying or substituting an endpoint refuses. | Test (TC-141) |
| FR-111-AC-6 | Exact replay of one complete relationship value is idempotent, while multiple distinct admitted values satisfying one required slot are ambiguous. | Test (TC-141) |
| FR-111-AC-7 | A receipt observation, delivery, receive occurrence and message remain non-interchangeable even when they share transport data. | Test (TC-141) |

## Dependencies

- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md).
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md): relationship declarations are spec-declared and keyed by domain package identity, IR node identity and `sha256-jcs`; B owns protocol subject meaning.
- [FR-287](../../objects/protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  supplies each ordered endpoint key.

## Status

Accepted by FS04 for explicit relationship correlation and ambiguity/refusal.
Observation #18/#13 retain executable declaration-key/topology adoption.
