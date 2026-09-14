---
id: NFR-060
title: "Bound and reproduce output mapping"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-125
    type: constrains
---
## Statement

The output-mapping subsystem SHALL use checked arithmetic and explicit aggregate
limits for request bytes, obligations, expression nodes, nesting, mapping work,
records and emitted bytes; equal admitted inputs SHALL produce byte-identical
packages, and limit failure SHALL expose no partial or smaller successful output.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Checked bound overflows accepted | 0 | 0 | property-based-testing |
| Emitted packages above an admitted limit | 0 | 0 | boundary testing |
| Partial packages exposed after failure/cancellation | 0 | 0 | fault-injection |
| Byte or identity drift for equal admitted input | 0 differing bytes/fields | 0 | metamorphic-testing |

## Verification

Exercise zero, exact-limit and just-over-limit requests for every aggregate axis;
overflow composed bounds; inject cancellation/failure at validation, mapping and
assembly; and repeat equal requests under different path/time/locale/observer
environments.

## Status

Accepted by FS06. Concrete limits and evidence remain in qcir #55/#56/#57.
