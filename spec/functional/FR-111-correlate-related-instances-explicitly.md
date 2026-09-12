---
id: FR-111
title: "Correlate related workflow instances only through declared relationships"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
  - target: ix://agent-ix/quire-specification/FR-262
    type: references
---
## Description

When associating an observation with a workflow subject, the correlation service SHALL select only an explicit typed relationship from the selected Producer interface 1.2.0 identity/version/digest; unknown, conflicting, and ambiguous relationships are typed non-success correlation outcomes rather than heuristic matches.

## Behavior

Workflow instances, deliveries, attempts, effects, and receipts remain distinct. A duplicate receipt SHALL NOT establish a second business effect. A refund is admitted only with its compensated effect and order relationship; a telemetry trace ID or shared provider identity does not establish either relationship.
The correlation service SHALL refuse a missing producer identity, unknown producer
version, or producer digest mismatch rather than reconstructing a relationship
from a foreign key, trace ID, provider name, or ambient configuration.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-111-AC-1 | A refund linked to O1 is refused for O2 even when both orders share a provider or trace infrastructure. | Test (TC-141) |
| FR-111-AC-2 | Missing, conflicting, and ambiguous required relationships yield distinct non-success correlation outcomes. | Test (TC-141) |
| FR-111-AC-3 | Repeated delivery is retained as a delivery and cannot alone create a second attempt or effect. | Test (TC-141) |
| FR-111-AC-4 | A missing producer relationship identity, unknown producer version, or producer digest mismatch is refused without reconstructing a relationship. | Test (TC-141) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- D owns the selected Producer interface 1.2.0 relationship identity; B owns protocol subject meaning.
