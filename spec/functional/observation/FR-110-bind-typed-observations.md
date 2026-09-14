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
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
## Description

When admitting an observation, the observation binder SHALL retain the exact
observation identity, source revision/schema identity, selected binding-contract
definition identity, revision, and byte digest, declared binding identity, field
units, and selected subject identity, or refuse the observation with a typed
cause.

The observation binder SHALL derive the observation identity under FR-288 from
the complete closed canonical observation with only its identity omitted. A producer SHALL NOT
author the identity independently, and a strict reader SHALL refuse a missing,
stale, cross-domain or same-identity/different-bytes observation.

## Behavior

The binder SHALL NOT substitute a signal name, endpoint, trace ID, or
compatible-looking field for a declared entity, trigger, unit, or valuation.
When a selected binding-contract definition, revision, or byte digest changes,
the binder SHALL require a new linked subject. When only a concrete mapping or
admitted record set changes under the same selected definition and revision, the
binder SHALL retain the static subject and identify the changed assessment input.
A binding-contract definition digest SHALL use FR-201 `raw-artifact-digest`
over its exact immutable bytes; the binder SHALL compare the domain before the
digest bytes.
A required absent valuation SHALL be incomplete or refused, never false or an
implicit option-none. The binder SHALL preserve whether a record is internal or
externally observed.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-110-AC-1 | A wrong entity, signal, unit, trigger, or stale schema identity is refused with the affected binding identity. | Test (TC-140) |
| FR-110-AC-2 | A required missing valuation remains incomplete/refused and does not produce a Boolean value. | Test (TC-140) |
| FR-110-AC-3 | A valid observation retains source, binding, subject, and visibility provenance. | Test (TC-140) |
| FR-110-AC-4 | A changed binding-contract definition, revision, or byte digest requires a newly linked subject, while a changed concrete mapping or admitted record set under the same selection remains an identified assessment input. | Test (TC-140) |
| FR-110-AC-5 | The FR-288 preimage deterministically identifies the complete observation; every semantic/provenance mutation changes it, presentation permutation does not, and same identity on unequal bytes refuses. | Test (TC-140) |
| FR-110-AC-6 | A binding-contract digest with an absent/foreign domain or bytes from another definition revision refuses without a linked observation. | Test (TC-140) |

## Dependencies

- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md).
- [Observation binding contract](../../../proposals/quire-v1/observation-contract.md).
- D owns model declarations; A owns source/package/profile identity.
