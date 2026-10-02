---
id: FR-110
title: "Bind typed observations to exact source and semantic identities"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
  - target: ix://agent-ix/quire-specification/FR-260
    type: references
  - target: ix://agent-ix/quire-specification/FR-261
    type: references
  - target: ix://agent-ix/quire-specification/FR-296
    type: references
---
## Description

When admitting an observation, the observation binder SHALL retain the exact
observation identity, source schema identity, selected binding-contract
definition identity, declared binding identity, field units, and selected subject identity, or refuse the observation with a typed
cause.

The observation binder SHALL derive the observation identity under FR-296 from
the complete closed canonical observation with only its identity omitted. A producer SHALL NOT
author the identity independently, and a strict reader SHALL refuse a missing,
stale, cross-domain or same-identity/different-bytes observation.

## Behavior

The binder SHALL NOT substitute a signal name, endpoint, trace ID, or
compatible-looking field for a declared entity, trigger, unit, or valuation.
When a selected binding-contract definition changes, the binder SHALL require a
new linked subject. When only a concrete mapping or admitted record set changes
under the same selected definition, the binder SHALL retain the static subject
and identify the changed assessment input.
A required absent valuation SHALL be incomplete or refused, never false or an
implicit option-none. The binder SHALL preserve whether a record is internal or
externally observed.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-110-AC-1 | A wrong entity, signal, unit, trigger, or schema identity is refused with the affected binding identity. | Test (TC-140) |
| FR-110-AC-2 | A required missing valuation remains incomplete/refused and does not produce a Boolean value. | Test (TC-140) |
| FR-110-AC-3 | A valid observation retains source, binding, subject, and visibility provenance. | Test (TC-140) |
| FR-110-AC-4 | A changed binding-contract definition identity requires a newly linked subject, while a changed concrete mapping or admitted record set under the same selection remains an identified assessment input. | Test (TC-140) |
| FR-110-AC-5 | The FR-296 preimage deterministically identifies the complete observation; every semantic/provenance mutation changes it, presentation permutation does not, and same identity on unequal bytes refuses. | Test (TC-140) |

## Dependencies

- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md).
- [Observation binding contract](../../../proposals/quire-v1/observation-contract.md).
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md): observed subjects are typed by the spec-declared domain declarations of the checked package; A owns source/package/profile identity.

## Status

Accepted by FS04 for typed observation binding and explicit absence/refusal.
Admission implementation remains separate.
