---
id: FR-110
title: "Bind typed observations to exact source and semantic identities"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
---
## Description

When admitting an observation, the observation binder SHALL retain the exact observation identity, source revision/schema identity, declared binding identity, field units, and selected subject identity, or refuse the observation with a typed cause.

## Behavior

The binder SHALL NOT substitute a signal name, endpoint, trace ID, or compatible-looking field for a declared entity, trigger, unit, or valuation. A required absent valuation SHALL be incomplete or refused, never false or an implicit option-none. The binder SHALL preserve whether a record is internal or externally observed.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-110-AC-1 | A wrong entity, signal, unit, trigger, or stale schema identity is refused with the affected binding identity. | Test (TC-140) |
| FR-110-AC-2 | A required missing valuation remains incomplete/refused and does not produce a Boolean value. | Test (TC-140) |
| FR-110-AC-3 | A valid observation retains source, binding, subject, and visibility provenance. | Test (TC-140) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- D owns model declarations; A owns source/package/profile identity.

